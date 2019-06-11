#![allow(unused)]

pub mod strategy_small;
pub mod strategy_large;

pub mod structs;
pub mod harmonizer;
pub mod util;
pub mod from_wata;

use super::{Model, Command};

pub fn destroy(model: Model) -> Vec<Command> {
    if model.r <= 30 {
        eprintln!("Using the small strategy (R: {})", model.r);
        strategy_small::destroy_small(model)
    } else {
        eprintln!("Using the large strategy (R: {})", model.r);
        strategy_large::destroy_large(model)
    }
}
