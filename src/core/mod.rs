mod unit;
mod utils;
use self::{fas_unit::Unit, utils::*};

pub struct Status {
    fresh_rate : i32,
    top_app : String
}

pub enum Jank {
    Janked,
    UnJanked,
    Static,
}

pub fn run