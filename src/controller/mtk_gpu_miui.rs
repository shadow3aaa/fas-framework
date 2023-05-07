use crate::{misc, Controller};

pub struct Gpu {
    max: u32,
}

impl Gpu {
    fn write(o: u32) {
        misc::write_file(&o.to_string(), "/proc/gpufreqv2/fix_target_opp_index");
    }
    fn get_cur(&mut self) -> u32 {
        use std::fs;
        let cur = fs::read_to_string("/proc/gpufreqv2/gpufreq_status").unwrap();
        let cur = match cur.lines().nth(7) {
            Some(o) => o,
            None => {
                return self.max;
            }
        };
        let cur = misc::cut(cur, ":", 1);
        misc::cut(&cur, ",", 0).trim().parse().unwrap_or(self.max)
    }
    pub fn give() -> Box<dyn Controller> {
        use std::fs;
        let opp = fs::read_to_string("/proc/gpufreqv2/stack_signed_opp_table")
            .expect("Failed to read opp");
        let max = opp.lines().rev().next().unwrap_or("");
        let max = misc::cut(max, "*", 0);
        let max = misc::cut(&max, "[", 1);
        let max = max.trim().parse().unwrap();
        misc::write_file("0", "/proc/gpufreqv2/fix_target_opp_index");
        Box::new(Gpu { max })
    }
}

impl Controller for Gpu {
    fn d_support(&mut self) -> bool {
        false
    }
    // 检测是否支持该控制器
    fn support(&mut self) -> bool {
        misc::test_path("/proc/gpufreqv2/fix_target_opp_index")
    }
    fn g_down(&mut self) {
        if self.get_cur() < self.max {
            Gpu::write(self.get_cur() + 1);
        } else {
            Gpu::write(self.max);
        }
    }
    fn g_up(&mut self) {
        if self.get_cur() >= 1 {
            Gpu::write(self.get_cur() - 1);
        } else {
            Gpu::write(0);
        }
    }
    fn g_reset(&mut self) {
        misc::write_file("0", "/proc/gpufreqv2/fix_target_opp_index");
    }
    fn d_reset(&mut self) {
        misc::write_file("-1", "/proc/gpufreqv2/fix_target_opp_index");
    }
}
