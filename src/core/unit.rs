use crate::ControllerNeed;

pub struct Unit<'a> {
    controller : &'a mut dyn ControllerNeed,
    need_review : bool,
}

impl<'a> Unit<'a> {
    pub fn new(controller : &'a mut dyn ControllerNeed) -> Box<Self> {
        Box::new(Self {
            controller,
            need_review : false,
        })
    }
    pub fn reset(&mut self, game : bool) {
        self.need_review = false;
        if game {
            self.controller.g_reset();
        } else {
            self.controller.d_reset();
        }
    }
    
}