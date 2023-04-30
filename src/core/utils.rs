use super::{Status, Jank};
use crate::{misc, WatcherNeed, ControllerNeed};
use std::process;

// 获取第一个支持的监视器
pub fn get_watcher<'a>(watcher_list : &'a mut [Box<dyn WatcherNeed>]) -> &'a mut Box<dyn WatcherNeed> {
    for w in watcher_list {
        if w.support() {
            return w;
        }
    }
    eprintln!("There is no supported frame rendering time monitor.");
    process::exit(-1);
}

pub fn get_controller_list<'a>(controller_list : &'a mut [Box<dyn ControllerNeed>]) -> Vec<&'a mut Box<dyn ControllerNeed>> {
    let mut r = Vec::new();
    for c in controller_list {
        if c.support() {
            r.push(c);
        }
    }
    if r.is_empty() {
        eprintln!("There are no supported performance controllers.");
        process::exit(-2);
    }
    r
}

pub fn check() -> Jank {
    
}

impl Status {
    pub fn new() -> Status {
        Status {
            fresh_rate : 0,
            top_app : String::new(),
        }
    }
    pub fn update(&mut self) {
    
    }
}