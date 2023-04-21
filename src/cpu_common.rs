use fas_framework::{misc, ControllerNeed};
use std::fs;

pub struct Cpu {
    freq_table: Vec<u32>,
    path: String
}

impl Cpu {
    fn new() -> Cpu {

    }
    pub fn give() -> Vec<Cpu> {
        let policys = fs::read_dir("/sys/devices/system/cpu/cpufreq");
        for policy in policys {
            if policy.
        }
    }
    fn get_freq() -> u64 {

    }
}

impl ControllerNeed for Cpu {
    // 是否支持日用模式
    fn d_support(&self) -> bool {
        false
    }
    // 检测是否支持该控制器
    fn support(&self) -> bool {
        misc::test_path(&self.path)
    }
    // 游戏内增加性能和功耗的方法
    fn g_up(&self) {
        self.freq_table.find(self.get_freq);
    }
    // 游戏外降低性能和功耗的方法
    fn g_down(&self);
    // 日用增加性能和功耗的方法(如果没有就写个空函数)
    fn d_up(&self);
    // 日用降低性能和功耗的方法(同上)
    fn d_down(&self);
    fn g_reset(&self);
    fn d_reset(&self);
}