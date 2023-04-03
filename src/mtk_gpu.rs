use fas_framework::{ControllerNeed, misc};

pub struct Gpu {
    max: u32,
}

impl Gpu {
    fn write(o: u32) {
        misc::write_file(&o.to_string(), "/proc/gpufreqv2/fix_target_opp_index");
    }
    fn get_cur() -> u32 {
        use std::fs;
        let cur = fs::read_to_string("/proc/gpufreqv2/gpufreq_status")
            .unwrap();
        let cur = misc::look_for_head(&cur, 7).unwrap();
        println!("{}", cur);
        let cur = misc::cut(cur, ":", 1);
        misc::cut(&cur, ",", 0)
            .trim()
            .parse()
            .unwrap()
    }
    pub fn give() -> Box<dyn ControllerNeed> {
        use std::fs;
        let opp = fs::read_to_string("/proc/gpufreqv2/stack_signed_opp_table")
            .expect("Failed to read opp");
        let max = match misc::look_for_tail(&opp, 0) {
            Some(o) => o,
            None => ""
        };
        println!("{}", max);
        let max = misc::cut(&max, "*", 0);
        let max = misc::cut(&max, "[", 1);
        let max = max.trim()
            .parse()
            .unwrap();
        misc::write_file("0", "/proc/gpufreqv2/fix_target_opp_index");
        Box::new(Gpu {
            max
        })
    }
}

impl ControllerNeed for Gpu{
    // 检测是否支持该控制器
    fn support(&self) -> bool {
        misc::test_file("/proc/gpufreqv2/fix_target_opp_index")
    }
    fn g_down(&self) {
        if Gpu::get_cur() <= self.max - 2 {
            Gpu::write(Gpu::get_cur() + 2);
        } else {
            Gpu::write(self.max);
        }
    }
    fn g_up(&self) {
        if Gpu::get_cur() >= 3 {
            Gpu::write(Gpu::get_cur() - 3);
        } else {
            Gpu::write(0);
        }
    }
    // 日用增加性能和功耗的方法(如果没有就写个空函数)
    fn d_down(&self) {
        if Gpu::get_cur() + 3 <= self.max {
            Gpu::write(Gpu::get_cur() + 3);
        } else {
            Gpu::write(self.max);
        }
    }
    // 日用降低性能和功耗的方法(同上)
    fn d_up(&self) {
        if Gpu::get_cur() >= 3 {
            Gpu::write(Gpu::get_cur() - 3);
        } else {
            Gpu::write(0);
        }
    }
    fn g_reset(&self) {
        misc::write_file("0", "/proc/gpufreqv2/fix_target_opp_index");
    }
    fn d_reset(&self) {
        self.g_reset();
    }
}