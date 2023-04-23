use fas_framework::{misc, ControllerNeed};
use std::fs;

pub struct Cpu {
    freq_table: Vec<i32>,
    path: String,
}

enum NextOrLast {
    Next,
    Last,
}

impl Cpu {
    fn make(path: &str) -> Cpu {
        // 读取频率表
        let freq_file = format!("{}/scaling_available_frequencies", path);
        let freq_table =
            fs::read_to_string(freq_file).expect("Failed to read cpu frequencies table");

        // 处理，转i32，排序(从小到大)
        let mut freq_table: Vec<i32> = freq_table
            .split_whitespace()
            .map(|f| f.parse::<i32>().unwrap())
            .collect();
        freq_table.sort();

        Cpu {
            freq_table,
            path: String::from(path),
        }
    }
    pub fn give() -> Vec<Cpu> {
        // 大部分soc有多个cpu集群，因此返回一个Vec
        let mut r: Vec<Cpu> = Vec::new();

        // 读取cpu集群目录
        let all = match fs::read_dir("/sys/devices/system/cpu/cpufreq") {
            Ok(o) => o,
            // 乐了，怎么你这手机连cpu节点都没有
            Err(e) => {
                eprint!("{}", e);
                return r;
            }
        };

        // 每个集群创建一个控制器
        for cpu in all {
            let cpu = match cpu {
                Ok(o) => {
                    // 忽略小核
                    if o.file_name() == "policy0" {
                        continue;
                    }
                    o
                }
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                }
            };

            r.push(Self::make(cpu.path().to_str().unwrap()));
        }
        r
    }
    fn find_or_refine(&mut self, x: i32) -> usize {
        if let Some(i) = self.freq_table.iter().position(|&y| y == x) {
            i
        } else {
            let c = self
                .freq_table
                .iter()
                .min_by_key(|&y| (y - x).abs())
                .unwrap();
            let i = self.freq_table.iter().position(|&y| y == *c).unwrap();
            self.freq_table[i] = x;
            i
        }
    }
    fn get_freq(&mut self, option: NextOrLast) -> i32 {
        // 读取现在的最大频率
        let path = format!("{}/scaling_max_freq", self.path);
        let max_freq = fs::read_to_string(path)
            .expect("Failed to read max frequencies")
            .parse::<i32>()
            .unwrap_or(*self.freq_table.last().unwrap());

        // 获取在频率表中的下标
        let cur = self.find_or_refine(max_freq);

        // 根据需要返回要写入的频率
        match option {
            NextOrLast::Next => *self
                .freq_table
                .get(cur + 1)
                .unwrap_or(self.freq_table.last().unwrap()),
            NextOrLast::Last => *self
                .freq_table
                .get(cur - 1)
                .unwrap_or(self.freq_table.first().unwrap()),
        }
    }
    fn write_freq(&self, freq: i32) {
        let freq = freq.to_string();
        let path = format!("{}/scaling_max_freq", &self.path);
        misc::write_file(&freq, &path);
    }
}

impl ControllerNeed for Cpu {
    // 是否支持日用模式
    fn d_support(&mut self) -> bool {
        false
    }
    // 检测是否支持该控制器
    fn support(&mut self) -> bool {
        misc::test_path(&self.path)
    }
    // 游戏内增加性能和功耗的方法
    fn g_up(&mut self) {
        let freq = self.get_freq(NextOrLast::Next);
        self.write_freq(freq);
    }
    // 游戏外降低性能和功耗的方法
    fn g_down(&mut self) {
        let freq = self.get_freq(NextOrLast::Last);
        self.write_freq(freq);
    }
    // 日用增加性能和功耗的方法(如果没有就写个空函数)
    fn d_up(&mut self) {}
    // 日用降低性能和功耗的方法(同上)
    fn d_down(&mut self) {}
    fn g_reset(&mut self) {
        let freq = *self.freq_table.last().unwrap();
        self.write_freq(freq);
    }
    fn d_reset(&mut self) {
        self.g_reset();
    }
}
