use std::{sync::mpsc::Receiver, time::Duration};
use crate::{JankType, WatcherNeed, Mode};

pub fn get_target() -> Mode {
    use crate::misc;
    fn ask_is_game() -> bool {
        let current_surface_view = misc::exec_cmd("dumpsys", &["SurfaceFlinger", "--list"])
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
    fn get_refresh_rate() -> u64 {
        let i = match misc::exec_cmd("dumpsys", &["SurfaceFlinger"])
            .expect("Err : Failed to execute dumpsys SurfaceView")
            .lines()
            .find(| l | l.contains("refresh-rate")) {
                Some(o) => o,
                None => {
                    return 0;
                }
            };
        misc::cut(&misc::cut(i, ".", 0), ":", 1)
            .trim()
            .parse::<u64>()
            .unwrap()
    }
    match ask_is_game() {
        true => {
            
        },
        false => {
            return Mode::DailyMode(get_refresh_rate());
        }
    }
}

pub struct Watcher {
    ft_rx: Receiver<JankType>,
    fps_fn: fn(Duration) -> u64,
}

impl Watcher {
    pub fn new<T>(w: &[T]) -> Watcher where
            T: WatcherNeed {
        let ft_rx: Receiver<JankType>;
        for i in w {
            if i.support() {
                let ft_rx = i.get_ft();
                let fps_fn = i.get_fps();
                return Watcher {
                    ft_rx,
                    fps_fn
                }
            }
        }
        eprint!("似乎该程序不支持你的设备");
        std::process::exit(-1);
    }
    pub fn start(&self) {
        loop {
            
        }
    }
    fn get_ft_jank(&self) -> Result<bool, &'static str> {
    
    }
    fn get_fps_jank(&self) -> bool {
    
    }
}
