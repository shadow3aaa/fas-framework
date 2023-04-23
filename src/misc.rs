pub fn set_self_sched() {
    let self_pid = &std::process::id().to_string();
    write_file(self_pid, "/dev/cpuset/foreground/tasks");

    let policy0 = std::fs::read_to_string("/sys/devices/system/cpu/cpufreq/policy0/related_cpus");

    let policy0: Vec<usize> = policy0
        .unwrap_or_default()
        .split_whitespace()
        .map(|s| s.trim().parse().unwrap_or(0))
        .collect();

    affinity::set_thread_affinity(&policy0).unwrap_or_default();
}

pub fn exec_cmd(command: &str, args: &[&str]) -> Result<String, i32> {
    use std::process::Command;
    let output = Command::new(command).args(args).output();

    match output {
        Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).into_owned()),
        Err(e) => {
            eprintln!("{}", e);
            Err(-1)
        }
    }
}

pub fn cut(str: &str, sym: &str, f: usize) -> String {
    let fs: Vec<&str> = str.split(sym).collect();
    match fs.get(f) {
        Some(s) => s.trim().to_string(),
        None => String::new(),
    }
}

pub fn write_file(content: &str, path: &str) {
    use std::{
        fs::{set_permissions, OpenOptions},
        io::Write,
        os::unix::fs::PermissionsExt,
    };
    match set_permissions(path, PermissionsExt::from_mode(0o644)) {
        Ok(()) => {
            match OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(path)
            {
                Ok(mut file) => match file.write_all(content.as_bytes()) {
                    Ok(()) => {}
                    Err(e) => eprintln!("Write failed: {}", e),
                },
                Err(e) => eprintln!("Open failed: {}", e),
            }
        }
        Err(e) => eprintln!("Set permissions failed: {}", e),
    }
}

pub fn test_path(x: &str) -> bool {
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
        let x = rx.recv();
        if let Ok(o) = x {
            r.push(o);
        }
    }
    drop(rx);
    if r.is_empty() {
        None
    } else {
        Some(r)
    }
}

pub fn get_top_app() -> String {
    use std::fs;
    use std::path::Path;
    let mut topapp = String::new();
    if Path::new("/sys/kernel/gbe/gbe2_fg_pid").exists() {
        let pid = fs::read_to_string("/sys/kernel/gbe/gbe2_fg_pid")
            .expect("Err : Fail to read pid")
            .trim()
            .to_string();
        topapp = fs::read_to_string(format!("/proc/{}/cmdline", pid))
            .unwrap_or(String::new())
            .trim_matches('\0') // cmdline文件尾端有很多NULL🥶
            .to_string();
        return topapp;
    }
    let dump_top = exec_cmd("dumpsys", &["activity", "activities"])
        .expect("Err : Failed to dumpsys for Topapp");
    for line in dump_top.lines() {
        if line.contains("topResumedActivity=") {
            topapp = cut(line, "{", 1);
            topapp = cut(&topapp, "/", 0);
            topapp = cut(&topapp, " ", 2);
        }
    }
    topapp
}

pub fn ask_is_game() -> bool {
    let current_surface_view = exec_cmd("dumpsys", &["SurfaceFlinger", "--list"])
        .expect("Err : Failed to execute dumpsys SurfaceView");
    // 忽略被误判的
    let ignore = [
        "tv.danmaku.bili",
        "com.perol.pixez",
        "jp.pxv.android",
        "com.lemurbrowser.exts",
        "mark.via",
        "com.tencent.mm",
        "com.tencent.mobileqq",
        "com.miui.gallery",
    ];
    if ignore.contains(&&get_top_app()[..]) {
        return false;
    }
    for line in current_surface_view.lines() {
        if line.contains("SurfaceView[") && line.contains("BLAST") || line.contains("SurfaceView -")
        {
            return line.contains(&get_top_app());
        }
    }
    false
}

pub fn get_refresh_rate() -> u64 {
    let i = match exec_cmd("dumpsys", &["SurfaceFlinger"])
        .expect("Err : Failed to execute dumpsys SurfaceView")
        .lines()
        .find(|l| l.contains("refresh-rate"))
    {
        Some(o) => o.to_string(),
        None => {
            return 0;
        }
    };
    let c = cut(&i, ".", 0);
    cut(&c, ":", 1).trim().parse::<u64>().unwrap_or(0)
}

#[inline]
pub fn look_for_head(s: &str, h: usize) -> Option<&str> {
    s.lines().nth(h)
}

#[inline]
pub fn look_for_tail(s: &str, t: usize) -> Option<&str> {
    s.lines().rev().nth(t)
}

#[inline]
pub fn next_multiple<T>(input_num: T, multiple: T) -> T
where
    T: std::ops::Rem<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::cmp::PartialOrd
        + Copy,
{
    let remainder = input_num % multiple;
    let low = input_num - remainder;
    let high = low + multiple;
    
    if input_num - low > high - input_num {
        high
    } else {
        low
    }
}