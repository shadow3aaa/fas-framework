use std::{sync::{mpsc::Receiver, Mutex}, time::Duration};
use crate::{JankType, WatcherNeed, Mode};

pub struct Watcher {
    ft_rx: Receiver<usize>,
    fps_fn: fn(Duration) -> u64,
    target_fps: Mutex<u64>
}

impl Watcher {
    pub fn get_current() -> Mode {
        use crate::misc;
        
        match misc::ask_is_game() {
            true => {
                return  Mode::GameMode;
            },
            false => {
                return Mode::DailyMode(misc::get_refresh_rate());
            }
        }
    }
    // 传入具体实现的监视器列表，匹配第一个支持的
    pub fn new<T>(w: &[T]) -> Watcher where
            T: WatcherNeed {
        let ft_rx: Receiver<JankType>;
        for i in w {
            if i.support() {
                let ft_rx = i.get_ft();
                let mut fps_fn = i.get_fps();
                return Watcher {
                    ft_rx,
                    fps_fn,
                    target_fps : Mutex::new(120)
                }
            }
        }
        eprint!("似乎该程序不支持你的设备");
        std::process::exit(-1);
    }
    // fas运行逻辑
    pub fn start(&mut self) {
        use std::{thread, time::Instant};
        // 不断获取target_fps的线程
        thread::spawn(|| {
            let mut fps = (self.fps_fn)(Duration::from_secs(1));
            let mut data: Vec<u64> = Vec::new();
            let mut clock = Instant::now();
            loop {
                fps = (self.fps_fn)(Duration::from_secs(1));
                data.push(fps);
                if clock.elapsed() > Duration::from_secs(30) {
                    clock = Instant::now();
                    *self.target_fps.lock()
                        .unwrap() = *data.iter().max().unwrap_or(&120) - 2;
                    data.clear();
                }
            }
        });
        // 控制逻辑
        loop {
            
        }
    }
    /* 消耗frametime消息管道所有数据
       返回指定最近帧内是否有超时 */
    fn get_ft_jank(&mut self, count: u32) -> Result<bool, &'static str> {
        use crate::misc;
        let mut ft_vec: Vec<usize> = Vec::new();
        loop {
            match self.ft_rx.try_recv() {
                Ok(o) => {
                    ft_vec.push(o);
                },
                _ => {
                    break;
                }
            }
        }
        if ft_vec.len() < count.try_into().unwrap() {
            return Err("data too less");
        }
        ft_vec.reverse();
        ft_vec.truncate(count.try_into().unwrap());
        let fresh_rate = misc::get_refresh_rate();
        let target_fps = *self.target_fps.lock().unwrap();
        let jank_count = ft_vec.iter()
            .filter(| &v | {
                if target_fps >= fresh_rate {
                    // 意思是，如果frametime超过目标frametime的1.1倍
                    return *v > (1000 * 1000 * 1000 / target_fps * 11 / 10) as usize;
                }
                return *v > *v as usize > 1000 * 1000 * 1000 / target_fps * 11 / 10
                    && 2 * 1000 * 1000 * 1000 / target_fps - 1000 * 1000 * 1000 /  ;
            })
            .count();
        Ok(result > ign)
    }
    /* 等待指定时间，并且返回指定时间通过fps看是否掉帧 */
    fn get_fps_jank(&self) -> u64 {
        (self.fps_fn)(Duration::from_secs(secs))
    }
    /* 添加控制器类型 */
    pub fn add(&self) {
    
    }
}
