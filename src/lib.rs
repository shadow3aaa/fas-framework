pub mod frame;
pub mod misc;

use std::time::Duration;
use crossbeam_channel::Receiver;

pub enum Mode {
    DailyMode(u64),
    GameMode
}

/* 监视器类型必须实现该trait */
pub trait WatcherNeed {
    // 检测是否支持该监视器
    fn support(&self) -> bool;
    // 返回一个用于接收frametime的Receiver
    fn get_ft(&self) -> Receiver<usize>;
    // 给出时间，得出从现在开始到指定时间内的平均fps
    fn get_fps(&self) -> fn(Duration) -> u64;
}

/* 控制器类型必须实现该trait */
pub trait ControllerNeed {
    // 检测是否支持该控制器
    fn support(&self) -> bool;
    // 增加性能和功耗的方法
    fn up(&self);
    // 降低性能和功耗的方法
    fn down(&self);
}