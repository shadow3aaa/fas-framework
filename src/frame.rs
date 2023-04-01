use std::{sync::mpsc::Receiver, time::Duration};
use crate::{JankType, WatcherNeed};

pub fn get_target_fps() -> 

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
