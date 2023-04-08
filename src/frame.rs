use std::time::Duration;
use crossbeam_channel::{bounded, Receiver};
use crate::{WatcherNeed, Mode, ControllerNeed, Jank};

pub struct Watcher<'a> {
    controller: &'a Box<dyn ControllerNeed>,
    ft_rx: Receiver<usize>,
    fps_fn: fn(Duration) -> u64,
    target_fps_rx: Receiver<u64>,
    target_fps: u64,
    inline_mode: Mode,
    janked: bool
}

enum UpOrDown {
        Up,
        Down
}

impl Watcher<'_> {
    fn get_current() -> Mode {
        use crate::misc;
        match misc::ask_is_game() {
            true => {
                return Mode::GameMode;
            },
            false => {
                return Mode::DailyMode(misc::get_refresh_rate());
            }
        }
    }
    fn get_target_fps(&mut self) -> u64 {
        match Watcher::get_current() {
            Mode::DailyMode(f) => {
                return f;
            },
            Mode::GameMode => {
                match self.target_fps_rx.try_recv() {
                    Ok(o) => {
                        self.target_fps = o;
                        return self.target_fps;
                    }
                    Err(_) => {
                        return self.target_fps;
                    }
                }
            }
        }
    }
    fn game_freq(&mut self, u: UpOrDown) {
        match u {
            UpOrDown::Up => {
                self.controller.g_up();
            },
            UpOrDown::Down => {
                if self.janked {
                    return;
                }
                self.controller.g_down();
            }
        }
    }
    fn daily_freq(&mut self, u: UpOrDown) {
        match u {
            UpOrDown::Up => {
                self.controller.d_up();
            },
            UpOrDown::Down => {
                if self.janked {
                    return;
                }
                self.controller.d_down();
            }
        }
    }
    fn game_reset(&mut self) {
        self.janked = false;
        self.controller.g_reset();
    }
    fn daily_reset(&mut self) {
        self.janked = false;
        self.controller.d_reset();
    }
    
    // fas运行逻辑
    fn run(&mut self, t: Duration) {
        self.daily_reset();
        match Watcher::get_current() {
            Mode::DailyMode(a) => {
                if self.inline_mode != Mode::DailyMode(a) {
                    self.inline_mode = Mode::DailyMode(a);
                    self.daily_reset();
                }
                if !self.controller.d_support() {
                    return;
                }
                let target_fps = self.get_target_fps();
                let fps_janked = self.get_fps_jank(Duration::from_millis(300));
                let ft_janked = match self.get_ft_jank(target_fps) {
                    Ok(o) => o,
                    Err(_) => {
                        false
                    }
                };
                match fps_janked {
                    Jank::Janked => {
                        self.janked = true;
                        self.daily_freq(UpOrDown::Up);
                    },
                    Jank::UnJanked => {
                        if ft_janked {
                            self.janked = true;
                            self.daily_freq(UpOrDown::Up);
                        } else {
                            self.janked = false;
                            self.daily_freq(UpOrDown::Down);
                        }
                    },
                    Jank::Static => {
                        self.janked = false;
                        self.daily_freq(UpOrDown::Down);
                    }
                }
            },
            Mode::GameMode => {
                if self.inline_mode != Mode::GameMode {
                    self.inline_mode = Mode::GameMode;
                    self.game_reset();
                }
                let target_fps = self.get_target_fps();
                let fps_janked = self.get_fps_jank(t);
                let ft_janked = match self.get_ft_jank(target_fps / 12) {
                    Ok(o) => o,
                    Err(o) => {
                        o
                    }
                };
                match fps_janked {
                    Jank::Janked => {
                        self.janked = true;
                        self.game_freq(UpOrDown::Up);
                    },
                    Jank::UnJanked => {
                        if ft_janked {
                            self.janked = true;
                            self.game_freq(UpOrDown::Up);
                        } else {
                            self.janked = false;
                            self.game_freq(UpOrDown::Down);
                        }
                    },
                    _ => {}
                }
            }
        }
    }
    // 传入具体实现的监视器列表，匹配第一个支持的
    pub fn start<'a>(w: &'a[Box<dyn WatcherNeed>], c: &'a[Box<dyn ControllerNeed>]) {
        use std::{thread, time::Instant};
        for i in w {
            if !i.support() {
                continue;
            }
            // 创建监视器
            let ft_rx = i.get_ft();
            let fps_fn = i.get_fps();
            let (sender, target_fps_rx) = bounded(1);
            thread::spawn(move || {
                let mut data: Vec<u64> = Vec::new();
                let mut timer = Instant::now();
                loop {
                    let fps = (fps_fn)(Duration::from_secs(1));
                    data.push(fps);
                    if timer.elapsed() >= Duration::from_secs(10) {
                        timer = Instant::now();
                        let max_fps = *data.iter().max().unwrap_or(&120);
                        data.clear();
                        sender.send(max_fps).unwrap();
                    }
                }
            });
            // 创建多个实例
            let mut w_vec: Vec<Watcher> = Vec::new();
            for ci in c {
                if !ci.support() {
                    continue;
                }
                let n = Watcher {
                    ft_rx : ft_rx.clone(),
                    controller : ci,
                    fps_fn ,
                    target_fps_rx : target_fps_rx.clone(),
                    target_fps : 120,
                    inline_mode : Mode::DailyMode(120),
                    janked : false
                };
                w_vec.push(n);
            }
            // 处理错误
            if w_vec.len() == 0 {
                eprintln!("没有支持的控制器!");
                std::process::exit(-1);
            }
            // 处理实例，最多4个
            if w_vec.len() > 4 {
                w_vec.truncate(4);
            }
            // 分配给每个实例的时间
            let t = Duration::from_millis((400 / w_vec.len())
                .try_into()
                .unwrap());
            // 控制多个实例
            loop {
                let timer = Instant::now();
                for w in &mut w_vec {
                    w.run(t);
                }
                if timer.elapsed() < Duration::from_millis(100) {
                    spin_sleep::sleep(Duration::from_millis(100));
                }
            }
        }
        eprint!("似乎该程序不支持你的设备");
        std::process::exit(-1);
    }
    /* 消耗frametime消息管道所有数据
       返回指定最近帧内是否有超时 */
    fn get_ft_jank(&mut self, count: u64) -> Result<bool, bool> {
        use crate::misc;
        let mut ft_vec: Vec<usize> = Vec::new();
        let iter = self.ft_rx.try_iter().peekable();
        ft_vec.extend(iter.map(|x| x.clone()));
        ft_vec.reverse();
        ft_vec.truncate(count.try_into().unwrap());
        let fresh_rate = misc::get_refresh_rate();
        let target_fps = self.get_target_fps();
        let jank_count = ft_vec.iter()
            .filter(| &v | {
                let sc_f = (1000 * 1000 * 1000 / fresh_rate) as usize;
                let o = *v % sc_f;
                o >= (1000 * 1000 * 1000 / target_fps * 11 / 10) as usize
            })
            .count();
        if ft_vec.len() < count.try_into().unwrap() {
            return Err(jank_count > 3);
        }
        Ok(jank_count > 3)
    }
    /* 等待指定时间，并且返回指定时间通过fps看是否掉帧 */
    fn get_fps_jank(&mut self, t: Duration) -> Jank {
        let fps = (self.fps_fn)(t);
        let target_fps = self.get_target_fps();
        match Watcher::get_current() {
            Mode::DailyMode(f) => {
                if fps > f / 12 && fps < f - 10 {
                    return Jank::Janked;
                } else if fps <= f / 4 || (fps > 30 && fps < 38 && target_fps != 30) || (fps > 60 && fps < 68 && target_fps != 60) || (fps > 120 && fps < 128 && target_fps != 120) {
                    return Jank::Static;
                } else {
                    return Jank::UnJanked;
                }
            }
            Mode::GameMode => {
                if fps < self.get_target_fps() - 2 {
                    return Jank::Janked;
                } else {
                    return Jank::UnJanked;
                }
            }
        }
    }
}