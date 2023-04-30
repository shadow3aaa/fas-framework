mod unit;
mod utils;
use crate::{ControllerNeed, WatcherNeed};

pub struct Status {
    pub fresh_rate: u64,
    pub game: bool,
}

pub enum Jank {
    Janked,
    UnJanked,
    Static,
}

type ControllerRef<'a> = &'a mut Box<dyn ControllerNeed + 'a>;

pub fn process<'a>(
    watcher_list: &'a mut [Box<dyn WatcherNeed>],
    controller_list: &'a mut [Box<dyn ControllerNeed>],
) {
    // 匹配设备支持的方法
    let watcher = utils::get_watcher(watcher_list);
    let controller_list = utils::get_controller_list(controller_list);

    // 分开通用和游戏
    let (ctrl_all_sup, ctrl_only_game) = filter_controllers(&controller_list);

    loop {}
}

// 分别获取支持日用模式的和不支持的
fn filter_controllers<'a>(
    list: &'a Vec<ControllerRef<'a>>,
) -> (Vec<ControllerRef<'a>>, Vec<ControllerRef<'a>>) {
    let mut support_daily = Vec::new();
    let mut only_game = Vec::new();

    for ctrl in list.iter_mut() {
        if ctrl.d_support() {
            support_daily.push(Box::new(**ctrl));
        }
        only_game.push(Box::new(**ctrl));
    }

    let support_daily_refs = support_daily.iter_mut().map(|x| x.as_mut()).collect();
    let only_game_refs = only_game.iter_mut().map(|x| x.as_mut()).collect();

    (support_daily_refs, only_game_refs)
}
