use fas_framework::{frame::Watcher, misc};

/* 在这里导入模块 */
mod watcher_fbt_info;
use watcher_fbt_info::FBTWatcher;
mod mtk_gpu;
use mtk_gpu::Gpu;

fn main() {
    misc::bound_to_little();
    let watcher_list = [FBTWatcher::give()];
    let controller_list = [Gpu::give()];
    let mut w = Watcher::new(&watcher_list, &controller_list);
    w.start();
}
