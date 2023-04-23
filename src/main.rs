use fas_framework::{frame::Watcher, misc};

/* 在这里导入模块 */
mod watcher_fbt_info;
use watcher_fbt_info::FBTWatcher;
mod cpu_common;
mod mtk_gpu;
mod mtk_gpu_miui;

fn main() {
    misc::set_self_sched();
    let miui = !misc::exec_cmd("getprop", &["ro.miui.ui.version.code"])
        .unwrap()
        .is_empty();
    let mut watcher_list = [FBTWatcher::give()];
    let mut controller_list = Vec::new();
    if !miui {
        // 如果不是miui
        controller_list.push(mtk_gpu::Gpu::give());
    } else {
        // 如果是miui
        controller_list.push(mtk_gpu_miui::Gpu::give());
    }
    Watcher::start(&mut watcher_list, &mut controller_list);
}
