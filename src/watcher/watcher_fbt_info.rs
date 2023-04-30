use crossbeam_channel::{bounded, Receiver};
use fas_framework::{misc, WatcherNeed};
use std::fs;

pub struct FBTWatcher;

impl FBTWatcher {
    fn read_ft() -> usize {
        Self::enable();
        let fbt_info = match fs::read_to_string("/sys/kernel/fpsgo/fbt/fbt_info") {
            Ok(o) => o,
            Err(e) => {
                eprintln!("{}", e);
                return 0;
            }
        };
        let fbt_info: Vec<&str> = fbt_info.lines().collect();
        let fbt_info = match fbt_info.get(8) {
            Some(o) => o,
            None => {
                return 0;
            }
        };
        let fbt_info: Vec<&str> = fbt_info.split_whitespace().collect();
        match fbt_info.get(6) {
            Some(o) => o.parse::<usize>().unwrap(),
            None => 0,
        }
    }
    fn read_fps() -> u64 {
        Self::enable();
        let fpsgo_status = fs::read_to_string("/sys/kernel/fpsgo/fstb/fpsgo_status").unwrap();
        let top_app = misc::get_top_app();
        let mut r = 0;

        for line in fpsgo_status.lines() {
            let app = misc::cut_whitespace(line, 2);

            let fps = misc::cut_whitespace(line, 3);
            let fps = match fps.trim().parse::<u64>() {
                Ok(o) => o,
                Err(_) => {
                    continue;
                }
            };

            if top_app.contains(&app) && !app.is_empty() {
                r = std::cmp::max(r, fps);
            }
        }
        r
    }
    fn enable() {
        misc::write_file("1", "/sys/kernel/fpsgo/common/fpsgo_enable")
    }
    pub fn give() -> Box<dyn WatcherNeed> {
        Box::new(FBTWatcher {})
    }
}

impl WatcherNeed for FBTWatcher {
    fn support(&mut self) -> bool {
        misc::test_path("/sys/kernel/fpsgo/fbt/fbt_info")
    }
    fn get_ft(&mut self) -> Receiver<usize> {
        use spin_sleep::SpinSleeper;
        use std::{thread, time::Duration};

        let sleeper = SpinSleeper::default();
        let (tx, rx) = bounded(147);

        thread::spawn(move || loop {
            let cur_a = FBTWatcher::read_ft();
            let mut cur_b = FBTWatcher::read_ft();
            while cur_b == cur_a {
                cur_b = FBTWatcher::read_ft();
                sleeper.sleep(Duration::from_millis(6));
            }
            let frametime = cur_b - cur_a;
            tx.send(frametime).unwrap();
        });
        rx
    }
    fn get_fps(&mut self, avg_time: std::time::Duration) -> u64 {
        let r = match misc::timer_exec(avg_time, FBTWatcher::read_fps) {
           Some(o) => o,
           None => {
               return 0
           }
        };
         
        r.iter().sum() / r.len() as u64
    }
}
