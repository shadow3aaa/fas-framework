mod utils;

use crate::{Controller, Watcher};
use std::process::exit;

pub fn process(
    watcher_list: impl IntoIterator<Item = Box<dyn Watcher>>,
    controller_list: impl IntoIterator<Item = Box<dyn Controller>>,
) {
    // 获取watcher
    let watcher = match utils::get_watcher(watcher_list) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{}", e);
            exit(-1)
        }
    };

    // 获取controller
    let (all_support, only_game) = match utils::get_controller(controller_list) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{}", e);
            exit(-1)
        }
    };

    
}
