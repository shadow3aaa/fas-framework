mod unit;
mod utils;
use crate::{WatcherNeed, ControllerNeed};

pub struct Status {
    fresh_rate : i32,
    top_app : String
}

pub enum Jank {
    Janked,
    UnJanked,
    Static,
}

pub fn run<'a>(watcher_list: &'a mut [Box<dyn WatcherNeed>], controller_list: &'a mut [Box<dyn ControllerNeed>]) {
    // 匹配设备支持的方法
    let watcher = utils::get_watcher(watcher_list);
    let controller_list = utils::get_controller_list(controller_list);

    loop {
        
    }
}