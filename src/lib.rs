pub mod frame;
pub mod misc;

use crossbeam_channel::Receiver;
use std::time::Duration;

#[derive(PartialEq)]
pub enum Mode {
    DailyMode(u64),
    GameMode,
    None,
}

pub enum Jank {
    Janked,
    UnJanked,
    Static,
}

#[derive(PartialEq)]
pub enum UpOrDown {
    Up,
    Down,
    None,
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
    // 是否支持日用模式
    fn d_support(&self) -> bool;
    // 检测是否支持该控制器
    fn support(&self) -> bool;
    // 游戏内增加性能和功耗的方法
    fn g_up(&self);
    // 游戏外降低性能和功耗的方法
    fn g_down(&self);
    // 日用增加性能和功耗的方法(如果没有就写个空函数)
    fn d_up(&self);
    // 日用降低性能和功耗的方法(同上)
    fn d_down(&self);
    fn g_reset(&self);
    fn d_reset(&self);
}
