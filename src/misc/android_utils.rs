use super::{cut, exec_cmd};

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
    let top_app: &str = &get_top_app();

    if top_app.is_empty() {
        return false;
    }

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
    if ignore.contains(&top_app) {
        return false;
    }

    for line in current_surface_view.lines() {
        if (line.contains("SurfaceView[") && line.contains("BLAST")
            || line.contains("SurfaceView -"))
            && line.contains(top_app)
        {
            return true;
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
