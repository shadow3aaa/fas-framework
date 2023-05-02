use super::{Jank, Status};
use crate::{misc, ControllerNeed, WatcherNeed};
use std::process;

// 获取第一个支持的监视器
pub fn get_watcher<'a>(
    watcher_list: &'a mut [Box<dyn WatcherNeed>],
) -> &'a mut Box<dyn WatcherNeed> {
    for w in watcher_list {
        if w.support() {
            return w;
        }
    }
    eprintln!("There is no supported frame rendering time monitor.");
    process::exit(-1);
}

pub fn get_controller_list<'a>(
    controller_list: &'a mut [Box<dyn ControllerNeed>],
) -> Vec<&'a mut Box<dyn ControllerNeed>> {
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

/*pub fn check(watcher: &mut dyn WatcherNeed) -> Jank {

}*/

impl Status {
    pub fn new() -> Status {
        Status {
            fresh_rate: 0,
            game: false,
        }
    }
    // 返回true代表状态变化，需要reset
    pub fn update(&mut self) -> bool {
        self.fresh_rate = misc::get_refresh_rate();
        let game = misc::ask_is_game();
        if self.game != game {
            self.game = game;
            return true;
        }
        false
    }
}
