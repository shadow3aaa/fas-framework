use crate::ControllerNeed;

type ControllerRef<'a> = &'a mut Box<dyn ControllerNeed + 'a>;

pub struct Unit<'a> {
    pub controller: &'a mut dyn ControllerNeed,
    need_review: bool,
}

impl<'a> Unit<'a> {
    pub fn new(controller: &'a mut dyn ControllerNeed) -> Self {
        Self {
            controller,
            need_review: false,
        }
    }
    pub fn trans(list: &mut [Box<dyn ControllerNeed>]) -> Vec<Unit<'_>> {
        let mut r: Vec<Unit<'_>> = Vec::new();
        for c in list {
            r.push(Unit::new(&mut **c));
        }
        r
    }
    pub fn reset(&mut self, game: bool) {
        self.need_review = false;
        if game {
            self.controller.g_reset();
        } else {
            self.controller.d_reset();
        }
    }
    pub fn up(&mut self, game: bool) {
        if game {
            self.controller.g_up();
        } else {
            self.controller.d_up();
        }
    }
    pub fn down(&mut self, game: bool) {
        if game {
            self.controller.g_down();
        } else {
            self.controller.d_down();
        }
    }
}
