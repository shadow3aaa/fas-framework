use fas_framework::{JankType, WatcherNeed};

struct FBTWatcher;

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
}

impl WatcherNeed for FBTWatcher {
    fn support(&self) -> bool {
        use fas_framework::misc;
        misc::test_file("/sys/kernel/fpsgo/fbt/fbt_info")
    }
    fn get_ft(&self) -> Receiver<JankType> {
        use std::{sync::mpsc::Receiver, time::Duration, thread};
        use spin_sleep::SpinSleeper;
        
        let sleeper = SpinSleeper::default();
        let (tx, rx) = mpsc::channel();
        
        thread::spawn(move || {
            loop {
                let cur_a = FBTWatcher::read_ft();
                let (mut target_fps, mut cur_b) = FBTWatcher::read_ft();
                while cur_b == cur_a {
                    cur_b = FBTWatcher::read_ft();
                    sleeper.sleep(Duration::from_millis(3));
                }
                let frametime = cur_b - cur_a;
                tx.send(frametime).unwrap();
            }
        });
    }
}