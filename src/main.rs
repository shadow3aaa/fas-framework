use fas_framework::{frame::Watcher, misc};

/* 在这里导入模块 */
mod watcher_fbt_info;
use watcher_fbt_info::FBTWatcher;
mod mtk_gpu;
mod mtk_gpu_miui;

fn main() {
    misc::bound_to_little();
    let miui = misc::exec_cmd("getprop", &["ro.miui.ui.version.code"]).unwrap().is_empty();
    let watcher_list = [FBTWatcher::give()];
    let controller_list;
    if miui {
        controller_list = [mtk_gpu::Gpu::give()];
    } else {
        controller_list = [mtk_gpu_miui::Gpu::give()];
    }
    let mut w = Watcher::new(&watcher_list, &controller_list);
    w.start();
}
