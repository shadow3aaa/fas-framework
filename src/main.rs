use fas_framework::{frame::Watcher, misc};

/* 在这里导入模块 */
mod watcher_fbt_info;
use watcher_fbt_info::FBTWatcher;
mod mtk_gpu;
mod mtk_gpu_miui;

fn main() {
    misc::bound_to_little();
    let miui = !misc::exec_cmd("getprop", &["ro.miui.ui.version.code"]).unwrap().is_empty();
    let watcher_list = [FBTWatcher::give()];
    let mut controller_list = Vec::new();
    if !miui {
        // 如果不是miui
        controller_list.push(mtk_gpu::Gpu::give());
    } else {
        // 如果是miui
        controller_list.push(mtk_gpu_miui::Gpu::give());
    }
    Watcher::start(&watcher_list, &controller_list);
}
