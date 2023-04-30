pub mod core;
pub mod misc;

use crossbeam_channel::Receiver;
use std::time::Duration;

pub enum Jank {
    Janked,
    UnJanked,
    Static,
}

#[derive(PartialEq)]
pub enum Mode {
    DailyMode(u64),
    GameMode,
    None,
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
    fn support(&mut self) -> bool;
    // 返回一个用于接收frametime的Receiver
    fn get_ft(&mut self) -> Receiver<usize>;
    // 给出时间，得出从现在开始到指定时间内的平均fps
    fn get_fps(&mut self, _: Duration) -> u64;
}

/* 控制器类型必须实现该trait */
pub trait ControllerNeed {
    // 是否支持日用模式
    fn d_support(&mut self) -> bool;
    // 检测是否支持该控制器
    fn support(&mut self) -> bool;
    // 游戏内增加性能和功耗的方法
    fn g_up(&mut self);
    // 游戏外降低性能和功耗的方法
    fn g_down(&mut self);
    // 日用增加性能和功耗的方法(如果没有就写个空函数)
    fn d_up(&mut self);
    // 日用降低性能和功耗的方法(同上)
    fn d_down(&mut self);
    fn g_reset(&mut self);
    fn d_reset(&mut self);
}
