use std::{sync::mpsc::Receiver, time::Duration};
use crate::{JankType, WatcherNeed, Mode};

pub fn get_target() -> Mode {
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

pub struct Watcher {
    ft_rx: Receiver<usize>,
    fps_fn: fn(Duration) -> u64,
}

impl Watcher {
    // 传入具体实现的监视器列表，匹配第一个支持的
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
    // fas运行逻辑
    pub fn start(&self) {
        loop {
            
        }
    }
    /* 消耗frametime消息管道所有数据
       返回指定最近帧内是否有超时 */
    fn get_ft_jank(&mut self, count: u32) -> Result<bool, &'static str> {
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
        ft_vec.reverse();
        if ft_vec.len() < count.try_into().unwrap() {
            return Err("data too less");
        }
        ft_vec.truncate(fc.try_into().unwrap());
        let result: u32 = re.iter()
            .fliter(|&n| n == IfJank::Janked)
            .count();
        Ok(result > ign)
    }
    /* 等待指定时间*/
    fn get_fps_jank(&self) -> bool {
    
    }
    /* 添加控制器类型 */
    pub fn add(&self) {
    
    }
}
