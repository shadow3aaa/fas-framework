use std::time::Duration;
use crossbeam_channel::{bounded, Receiver};
use crate::{WatcherNeed, Mode, ControllerNeed, Jank};

pub struct Watcher<'a> {
    controllers: &'a[Box<dyn ControllerNeed>],
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
                for i in self.controllers {
                    i.g_up();
                }
            },
            UpOrDown::Down => {
                if self.janked {
                    return;
                }
                for i in self.controllers {
                    i.g_down();
                }
            }
        }
    }
    fn daily_freq(&mut self, u: UpOrDown) {
        match u {
            UpOrDown::Up => {
                for i in self.controllers {
                    i.d_up();
                }
            },
            UpOrDown::Down => {
                if self.janked {
                    return;
                }
                for i in self.controllers {
                    i.d_down();
                }
            }
        }
    }
    fn game_reset(&mut self) {
        self.janked = false;
        for i in self.controllers {
            i.g_reset();
        }
    }
    fn daily_reset(&mut self) {
        self.janked = false;
        for i in self.controllers {
            i.d_reset();
        }
    }
    
    // fas运行逻辑
    pub fn start(&mut self) {
        self.daily_reset();
        loop {
            match Watcher::get_current() {
                Mode::DailyMode(a) => {
                    if self.inline_mode != Mode::DailyMode(a) {
                        self.inline_mode = Mode::DailyMode(a);
                        self.daily_reset();
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
                    let fps_janked = self.get_fps_jank(Duration::from_millis(400));
                    let ft_janked = match self.get_ft_jank(target_fps / 12) {
                        Ok(o) => o,
                        Err(_) => {
                            continue;
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
    }
    // 传入具体实现的监视器列表，匹配第一个支持的
    pub fn new<'a>(w: &'a[Box<dyn WatcherNeed>], c: &'a[Box<dyn ControllerNeed>]) -> Watcher<'a> {
        use std::{thread, time::Instant};
        for i in w {
            if i.support() {
                let ft_rx = i.get_ft();
                let fps_fn = i.get_fps();
                let (sender, receiver) = bounded(1);
                let target_fps_rx = receiver;
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
                return Watcher {
                    ft_rx,
                    controllers : c,
                    fps_fn,
                    target_fps_rx,
                    target_fps : 120,
                    inline_mode : Mode::DailyMode(120),
                    janked : false
                }
            }
        }
        eprint!("似乎该程序不支持你的设备");
        std::process::exit(-1);
    }
    /* 消耗frametime消息管道所有数据
       返回指定最近帧内是否有超时 */
    fn get_ft_jank(&mut self, count: u64) -> Result<bool, &'static str> {
        use crate::misc;
        let mut ft_vec: Vec<usize> = Vec::new();
        let iter = self.ft_rx.try_iter().peekable();
        ft_vec.extend(iter.map(|x| x.clone()));
        if ft_vec.len() < count.try_into().unwrap() {
            return Err("data too less");
        }
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