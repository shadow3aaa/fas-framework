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