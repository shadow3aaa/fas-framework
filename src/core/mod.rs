mod unit;
mod utils;
use crate::{ControllerNeed, WatcherNeed};
use unit::Unit;

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
    mut controller_list: &'a mut [Box<dyn ControllerNeed>],
) {
    // 匹配设备支持的方法
    let watcher = utils::get_watcher(watcher_list);
    let controller_list = utils::get_controller_list(&mut controller_list);

    // 分开通用和游戏
    let mut controller_list: Vec<Box<dyn ControllerNeed>> = controller_list.iter_mut().cloned().collect();
let (ctrl_all_sup, ctrl_only_game) = filter_controllers(&mut controller_list);

    // 封装为单元
    let mut ctrl_all = Unit::trans(&mut controller_list);
    let mut ctrl_all_sup = Unit::trans(&mut ctrl_all_sup);
    let mut ctrl_only_game = Unit::trans(&mut ctrl_only_game);

    // 保存了当前需要状态信息，可以更新的结构体
    let mut status = Status::new();

    loop {
        if status.update() {

        }
    }
}

// 分别获取支持日用模式的和支持的
fn filter_controllers(
    controller_list: &[Box<dyn ControllerNeed>],
) -> (Vec<Box<dyn ControllerNeed>>, Vec<Box<dyn ControllerNeed>>) {
    let mut support_daily = Vec::new();
    let mut only_game = Vec::new();

    for c in controller_list.iter() {
        if c.d_support() {
            support_daily.push(c.clone());
        } else {
            only_game.push(c.clone());
        }
    }

    (support_daily, only_game)
}