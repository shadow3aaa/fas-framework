use fas_framework::WatcherNeed;
use crossbeam_channel::{Receiver, bounded};

pub struct FBTWatcher;

impl FBTWatcher {
    fn read_ft() -> usize {
        use std::fs;
        let fbt_info = fs::read_to_string("/sys/kernel/fpsgo/fbt/fbt_info")
            .unwrap();
        let fbt_info: Vec<&str> = fbt_info.lines()
            .collect();
        let fbt_info = fbt_info.get(8)
            .unwrap();
        let fbt_info: Vec<&str> = fbt_info.split_whitespace()
            .collect();
        fbt_info.get(6)
            .unwrap()
            .parse::<usize>()
            .unwrap()
    }
    pub fn give() -> Box<dyn WatcherNeed> {
        Box::new(FBTWatcher{})
    }
}

impl WatcherNeed for FBTWatcher {
    
    fn support(&self) -> bool {
        use fas_framework::misc;
        misc::test_file("/sys/kernel/fpsgo/fbt/fbt_info")
    }
    fn get_ft(&self) -> Receiver<usize> {
        use std::{time::Duration, thread};
        use spin_sleep::SpinSleeper;
        
        let sleeper = SpinSleeper::default();
        let (tx, rx) = bounded(147);
        
        thread::spawn(move || {
            loop {
                let cur_a = FBTWatcher::read_ft();
                let mut cur_b = FBTWatcher::read_ft();
                while cur_b == cur_a {
                    cur_b = FBTWatcher::read_ft();
                    sleeper.sleep(Duration::from_millis(3));
                }
                let frametime = cur_b - cur_a;
                tx.send(frametime).unwrap();
            }
        });
        return rx;
    }
    fn get_fps(&self) -> fn(std::time::Duration) -> u64 {
        fn fps_method(avg_time: std::time::Duration) -> u64 {
            use fas_framework::misc::{exec_cmd, cut};
            use std::time::Instant;
            use spin_sleep::SpinSleeper;
            let sleeper = SpinSleeper::default();
    
            let data_a = exec_cmd("service", &["call", "SurfaceFlinger", "1013"])
                .unwrap();
            let now = Instant::now();
            let data_a = cut(&cut(&data_a, "(", 1), "\'", 0);
            let data_a = u64::from_str_radix(&data_a, 16).unwrap();
    
            sleeper.sleep(avg_time);
    
            let data_b = exec_cmd("service", &["call", "SurfaceFlinger", "1013"])
                .unwrap();
            let data_b = cut(&cut(&data_b, "(", 1), "\'", 0);
            let data_b = u64::from_str_radix(&data_b, 16).unwrap();
            (data_b - data_a) * 1000 / (now.elapsed().as_millis() as u64)
        }
        fps_method
    }
}