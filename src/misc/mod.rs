mod android_utils;
mod mount;

// 把自身线程绑定到小核
pub fn set_self_sched() {
    let self_pid = &std::process::id().to_string();
    write_file(self_pid, "/dev/cpuset/background/tasks");

    let policy0 = std::fs::read_to_string("/sys/devices/system/cpu/cpufreq/policy0/related_cpus");

    let policy0: Vec<usize> = policy0
        .unwrap_or_default()
        .split_whitespace()
        .map(|s| s.trim().parse().unwrap_or(0))
        .collect();

    let _ = affinity::set_thread_affinity(&policy0);
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

// android_utils
#[inline(always)]
pub fn get_top_app() -> String {
    android_utils::get_top_app()
}

#[inline(always)]
pub fn ask_is_game() -> bool {
    android_utils::ask_is_game()
}

#[inline(always)]
pub fn get_refresh_rate() -> u64 {
    android_utils::get_refresh_rate()
}

pub fn cut(str: &str, sym: &str, f: usize) -> String {
    let mut fs = str.split(sym);
    let s = fs.nth(f).unwrap_or("").trim();
    s.to_string()
}

pub fn cut_whitespace(str: &str, f: usize) -> String {
    let mut fs = str.split_whitespace();
    let s = fs.nth(f).unwrap_or("").trim();
    s.to_string()
}

pub fn write_file(content: &str, path: &str) {
    use std::{
        fs::{set_permissions, OpenOptions},
        io::Write,
        os::unix::fs::PermissionsExt,
    };

    // debug
    // println!("path: {}, value: {}", path, content);

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

pub fn timer_exec<F, R>(t: std::time::Duration, f: F) -> Option<Vec<R>>
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

// 改变并锁定文件内容
pub fn lock_value(value: &str, path: &str) {
    mount::unmount(path);
    write_file(value, path);

    let mount_path = format!("/cache/mount_mask_{}", value);
    write_file(value, &mount_path);

    let _ = mount::mount_bind(&mount_path, path);
}
