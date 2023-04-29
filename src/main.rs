use self::{frame::Watcher, misc};
mod watcher;
use crate::watcher::*;
mod controller;
use crate::controller::*;

fn main() {
    misc::set_self_sched();

    let mut watcher_list = vec![watcher_fbt_info::FBTWatcher::give()];
    let mut controller_list = vec![mtk_gpu_miui::Gpu::give()];

    let mut cpu = cpu_common::Cpu::give();
    controller_list.append(&mut cpu);

    let miui = !misc::exec_cmd("getprop", &["ro.miui.ui.version.code"])
        .unwrap()
        .is_empty();
    if !miui {
        // 如果不是miui
        controller_list.push(mtk_gpu::Gpu::give());
    } else {
        // 如果是miui
        controller_list.push(mtk_gpu_miui::Gpu::give());
    }

    Watcher::start(&mut watcher_list, &mut controller_list);
}
