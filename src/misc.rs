pub fn bound_to_little() {
    let cpu0 = std::fs::read_to_string("/sys/devices/system/cpu/cpufreq/policy0/related_cpus");
    let cpu0 = match cpu0 {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    
    let cpu0 :Vec<&str> = cpu0.split_whitespace().collect();
    let cpu0 :Vec<usize> = cpu0
        .iter()
        .map(|s| s.trim().parse().unwrap())
        .collect();
    affinity::set_thread_affinity(&cpu0).unwrap();
}

pub fn exec_cmd(command :&str, args :&[&str]) -> Result<String, i32> {
    use std::process::Command;
    let output = Command::new(command)
        .args(args)
        .output();
    
    match output {
        Ok(o) => {
            Ok(String::from_utf8(o.stdout).expect("utf8 error"))
        }
        Err(e) => {
            eprintln!("{}", e);
            Err(-1)
        }
    }
}

pub fn cut(str: &str, sym: &str, f: usize) -> String {
    let fs: Vec<&str> = str.split(sym).collect();
    match fs.get(f) {
        Some(s) => s.trim()
            .to_string(),
        None => String::new()
    }
}

pub fn write_file(content: &str, path: &str) {
    use std::{io::Write, os::unix::fs::PermissionsExt, fs::{OpenOptions, set_permissions}};
    println!("path: {}, value: {}", &content, &path);
    match set_permissions(path, PermissionsExt::from_mode(0o644)) {
        Ok(()) => {
            match OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(path) {
                    Ok(mut file) => {
                        match file.write_all(content.as_bytes()) {
                            Ok(()) => {}
                            Err(e) => eprintln!("Write failed: {}", e),
                        }
                    },
                    Err(e) => eprintln!("Open failed: {}", e),
                }
        },
        Err(e) => eprintln!("Set permissions failed: {}", e),
    }
}

pub fn test_file(x: &str) -> bool {
    std::path::Path::new(x).exists()
}

use std::time::Duration;
pub fn timer_exec<F, R>(t: Duration, f: F) -> Option<Vec<R>>
where
F: Fn() -> R + Send + 'static,
R: Send + 'static,
{
    use std::{sync::mpsc, thread, time::Instant};
    let mut r = Vec::new();
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || loop {
        let x = f();
        match tx.send(x) {
            Ok(_) => (),
            Err(_) => break,
        }
    });
    let s = Instant::now();
    while s.elapsed() < t {
        let x = rx.recv().unwrap();
        r.push(x);
    }
    drop(rx);
    if r.is_empty() {
        None
    } else {
        Some(r)
    }
}

pub fn get_top_app() -> String {
    use std::path::Path;
    use std::fs;
    let mut topapp = String::new();
    if Path::new("/sys/kernel/gbe/gbe2_fg_pid").exists() {
        let pid = fs::read_to_string("/sys/kernel/gbe/gbe2_fg_pid")
            .expect("Err : Fail to read pid")
            .trim()
            .to_string();
        topapp = fs::read_to_string(format!("/proc/{}/cmdline", pid))
            .expect("Err : Fail to read cmdline")
            .trim_matches('\0') // cmdline文件尾端有很多NULL🥶
            .to_string();
        return topapp;
    }
    let dump_top = exec_cmd("dumpsys", &["activity", "activities"])
        .expect("Err : Failed to dumpsys for Topapp");
    for line in dump_top.lines() {
        if line.contains("topResumedActivity=") {
            topapp = cut(&line, "{", 1);
            topapp = cut(&topapp, "/", 0);
            topapp = cut(&topapp, " ", 2);
        }
    }
    topapp
}

pub fn ask_is_game() -> bool {
    let current_surface_view = exec_cmd("dumpsys", &["SurfaceFlinger", "--list"])
        .expect("Err : Failed to execute dumpsys SurfaceView");
    for line in current_surface_view.lines() {
        if line.contains("SurfaceView[") && line.contains("BLAST") {
            return true;
        } else if line.contains("SurfaceView -") {
            return true;
        }
    }
    return false;
}

pub fn get_refresh_rate() -> u64 {
    let i = match exec_cmd("dumpsys", &["SurfaceFlinger"])
        .expect("Err : Failed to execute dumpsys SurfaceView")
        .lines()
        .find(| l | l.contains("refresh-rate")) {
            Some(o) => o.to_string(),
            None => {
                return 0;
            }
        };
    let c = cut(&i, ".", 0);
    cut(&c, ":", 1)
        .trim()
        .parse::<u64>()
        .unwrap()
}

pub fn close_to<T: ToString>(n: T, m: T) -> bool {
    use std::cmp::Ordering;
    match n.to_string().len().cmp(&m.to_string().len()) {
        Ordering::Equal => n
            .to_string()
            .chars()
            .take(2)
            .eq(m.to_string().chars().take(2)),
        _ => false,
    }
}

pub fn look_for_line<'a>(s: &'a str, t: &str) -> &'a str {
    s.lines()
        .find(| l | l.contains(t))
        .unwrap_or("")
}

pub fn look_for_head<'a>(s: &'a str, h: usize) -> Option<&'a str>{
    s.lines().nth(h)
}

pub fn look_for_tail<'a>(s: &'a str, t: usize) -> Option<&'a str>{
    s.lines().nth(t)
}