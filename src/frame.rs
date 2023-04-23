use crate::{misc, ControllerNeed, Jank, Mode, UpOrDown, WatcherNeed};
use crossbeam_channel::{bounded, Receiver};
use std::time::Duration;

pub struct Watcher<'a> {
    controller: &'a mut dyn ControllerNeed,
    ft_rx: Receiver<usize>,
    fps_fn: fn(Duration) -> u64,
    target_fps_rx: Receiver<u64>,
    target_fps: u64,
    last_mode: Mode,
    last_do: UpOrDown,
    last_count: i32,
}

impl Watcher<'_> {
    fn get_current() -> Mode {
        match misc::ask_is_game() {
            true => Mode::GameMode,
            false => Mode::DailyMode(misc::get_refresh_rate()),
        }
    }

    fn get_target_fps(&mut self) -> u64 {
        match Watcher::get_current() {
            Mode::DailyMode(f) => f,
            Mode::GameMode => match self.target_fps_rx.try_recv() {
                Ok(o) => {
                    self.target_fps = misc::next_multiple(o, 5);
                    self.target_fps
                }
                Err(_) => self.target_fps,
            },
            Mode::None => 0,
        }
    }

    fn game_freq(&mut self, u: UpOrDown) {
        if self.last_do != u {
            self.last_count = 0;
        } else {
            self.last_count += 1;
        }
        match u {
            UpOrDown::Up => {
                for _ in 1..(self.last_count + 1) {
                    self.controller.g_up();
                }
            }
            UpOrDown::Down => {
                if self.last_do != UpOrDown::Up {
                    for _ in 1..(self.last_count + 1) {
                        self.controller.g_down();
                    }
                }
            }
            UpOrDown::None => (),
        }
        self.last_do = u;
    }

    fn daily_freq(&mut self, u: UpOrDown) {
        self.last_count = 0;
        match u {
            UpOrDown::Up => {
                self.controller.d_up();
            }
            UpOrDown::Down => {
                if self.last_do != UpOrDown::Up {
                    self.controller.d_down();
                }
            }
            UpOrDown::None => (),
        }
        self.last_do = u;
    }

    fn game_reset(&mut self) {
        self.last_do = UpOrDown::None;
        self.last_count = 0;
        self.controller.g_reset();
    }

    fn daily_reset(&mut self) {
        self.last_do = UpOrDown::None;
        self.last_count = 0;
        self.controller.d_reset();
    }

    // 单个fas模块运行逻辑
    fn run(&mut self, t: Duration) {
        match Watcher::get_current() {
            Mode::DailyMode(a) => {
                if self.last_mode != Mode::DailyMode(a) {
                    self.last_mode = Mode::DailyMode(a);
                    self.daily_reset();
                }
                if !self.controller.d_support() {
                    return;
                }
                let target_fps = self.get_target_fps();
                let fps_janked = self.get_fps_jank(Duration::from_millis(300));
                let ft_janked = self.get_ft_jank(target_fps).unwrap_or(false);
                match fps_janked {
                    Jank::Janked => {
                        self.daily_freq(UpOrDown::Up);
                    }
                    Jank::UnJanked => {
                        if ft_janked {
                            self.daily_freq(UpOrDown::Up);
                        } else {
                            self.daily_freq(UpOrDown::Down);
                        }
                    }
                    Jank::Static => {
                        self.daily_freq(UpOrDown::Down);
                    }
                }
            }
            Mode::GameMode => {
                if self.last_mode != Mode::GameMode {
                    self.last_mode = Mode::GameMode;
                    self.game_reset();
                }
                let target_fps = self.get_target_fps();
                let fps_janked = self.get_fps_jank(t);
                let ft_janked = match self.get_ft_jank(target_fps / 12) {
                    Ok(o) => o,
                    Err(o) => o,
                };
                match fps_janked {
                    Jank::Janked => {
                        self.game_freq(UpOrDown::Up);
                    }
                    Jank::UnJanked => {
                        if ft_janked {
                            self.game_freq(UpOrDown::Up);
                        } else {
                            self.game_freq(UpOrDown::Down);
                        }
                    }
                    _ => (),
                }
            }
            Mode::None => (),
        }
    }

    // 传入具体实现的监视器列表，匹配第一个支持的
    pub fn start<'a>(w: &'a mut [Box<dyn WatcherNeed>], c: &'a mut [Box<dyn ControllerNeed>]) {
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
                    ft_rx: ft_rx.clone(),
                    controller: &mut **ci,
                    fps_fn,
                    target_fps_rx: target_fps_rx.clone(),
                    target_fps: 120,
                    last_mode: Mode::None,
                    last_do: UpOrDown::None,
                    last_count: 0,
                };
                w_vec.push(n);
            }
            // 处理错误
            if w_vec.is_empty() {
                eprintln!("没有支持的控制器!");
                std::process::exit(-1);
            }
            // 处理Watcher实例，最多4个，因为时间不够
            if w_vec.len() > 4 {
                w_vec.truncate(4);
            }
            // 分配给每个实例的时间
            let t = Duration::from_millis((400 / w_vec.len()).try_into().unwrap());
            // 控制多个实例
            loop {
                let timer = Instant::now();
                for w in &mut w_vec {
                    w.run(t);
                }
                // 防止没有一个控制器支持日用模式
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
        let mut ft_vec: Vec<usize> = Vec::new();
        let iter = self.ft_rx.try_iter().peekable();
        ft_vec.extend(iter);
        ft_vec.reverse();
        ft_vec.truncate(count.try_into().unwrap());
        let fresh_rate = misc::get_refresh_rate();
        let target_fps = self.get_target_fps();
        let jank_count = ft_vec
            .iter()
            .filter(|&v| {
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
                    Jank::Janked
                } else if fps <= f / 4 {
                    return Jank::Static;
                } else {
                    return Jank::UnJanked;
                }
            }
            Mode::GameMode => {
                if fps < target_fps - 2 {
                    Jank::Janked
                } else {
                    Jank::UnJanked
                }
            }
            Mode::None => Jank::UnJanked,
        }
    }
}
