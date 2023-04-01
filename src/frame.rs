use std::time::Duration;
use crossbeam_channel::{bounded, Receiver};
use crate::{WatcherNeed, Mode};

pub struct Watcher {
    ft_rx: Receiver<usize>,
    fps_fn: fn(Duration) -> u64,
    target_fps_rx: Receiver<u64>,
    target_fps: u64
}

impl Watcher {
    fn get_current() -> Mode {
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
    fn get_target_fps(&mut self) -> u64 {
        match self.target_fps_rx.try_recv() {
            Ok(o) => {
                self.target_fps = o;
                self.target_fps
            }
            Err(_) => self.target_fps
        }
    }
    // 传入具体实现的监视器列表，匹配第一个支持的
    pub fn new<T>(w: &[T]) -> Watcher where
            T: WatcherNeed {
        use std::{thread, time::Instant};
        for i in w {
            if i.support() {
                let ft_rx = i.get_ft();
                let fps_fn = i.get_fps();
                let (sender, receiver) = bounded(1);
                let target_fps_rx = receiver;
                thread::spawn(move || {
                    let mut data: Vec<u64> = Vec::new();
                    let mut timer = Instant::now() + Duration::from_secs(30);
                    loop {
                        let fps = (fps_fn)(Duration::from_secs(1));
                        data.push(fps);
                        if Instant::now() >= timer {
                            timer += Duration::from_secs(30);
                            let max_fps = *data.iter().max().unwrap_or(&120) - 2;
                            data.clear();
                            sender.send(max_fps).unwrap();
                        }
                    }
                });
                return Watcher {
                    ft_rx,
                    fps_fn,
                    target_fps_rx,
                    target_fps : 120
                }
            }
        }
        eprint!("似乎该程序不支持你的设备");
        std::process::exit(-1);
    }
    // fas运行逻辑
    pub fn start(&mut self) {
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
        let target_fps = self.get_target_fps();
        let jank_count = ft_vec.iter()
            .filter(| &v | {
                if target_fps >= fresh_rate {
                    // 意思是，如果frametime超过目标frametime的1.1倍
                    return *v > (1000 * 1000 * 1000 / target_fps * 11 / 10) as usize;
                }
                let r = 1000 * 1000 * 1000 / fresh_rate;
                misc::close_to(*v, r as usize)
            })
            .count();
        Ok(jank_count > 3)
    }
    /* 等待指定时间，并且返回指定时间通过fps看是否掉帧 */
    fn get_fps_jank(&mut self, t: Duration) -> bool {
        let fps = (self.fps_fn)(t);
        fps < self.get_target_fps() - 3
    }
    /* 添加控制器类型 */
    pub fn add<T>(&self, : T) {
    
    }
}
