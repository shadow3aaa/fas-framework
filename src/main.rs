use fas_framework::frame::Watcher;

/* 在这里导入编写的jank监视器模块 */
mod watcher_fbt_info;
use watcher_fbt_info::FBTWatcher;

fn main() {
    Watcher::new(&[FBTWatcher::give()], c);
}
