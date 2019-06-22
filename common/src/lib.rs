pub mod reach;
pub mod task;
pub mod player_state;
pub mod sol;
pub mod bfs;
pub mod sim;
pub mod sim2;
pub mod tsp;
pub mod bootstrap;

pub use reach::*;
pub use task::*;
pub use player_state::*;
pub use bfs::*;
pub use sim::{WorkerState, apply_action};
pub use sim2::*;
pub use sol::*;
pub use tsp::*;
pub use bootstrap::*;

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

pub trait SetMinMax {
	fn setmin(&mut self, v: Self) -> bool;
	fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T where T: PartialOrd {
	fn setmin(&mut self, v: T) -> bool {
		*self > v && { *self = v; true }
	}
	fn setmax(&mut self, v: T) -> bool {
		*self < v && { *self = v; true }
	}
}

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum Square {
    Empty,
    Block,
    Filled,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum Booster {
    Extension,
    Fast,
    Drill,
    Teleport,
    X,
    CloneWorker,
}

impl std::str::FromStr for Booster {
    type Err = ();

    fn from_str(s: &str) -> Result<Booster, ()> {
        match s {
            "B" => Ok(Booster::Extension),
            "F" => Ok(Booster::Fast),
            "L" => Ok(Booster::Drill),
            "R" => Ok(Booster::Teleport),
            "X" => Ok(Booster::X),
            _ => Err(()),
        }
    }
}

pub fn apply_move((x, y): (usize, usize), dir: usize) -> (usize, usize) {
    match dir {
        0 => (x + 1, y),
        1 => (x, y - 1),
        2 => (x - 1, y),
        3 => (x, y + 1),
        _ => panic!("illegal dir: {}", dir)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Move(usize),
    Nothing,
    TurnR,
    TurnL,
    Extension(i32, i32),
    Fast,
    Drill,
    Reset,
    Teleport(usize, usize),
    CloneWorker,
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Action::Move(dir) => f.write_str(["D", "S", "A", "W"][*dir]),
            Action::Nothing => f.write_str("Z"),
            Action::TurnR => f.write_str("E"),
            Action::TurnL => f.write_str("Q"),
            Action::Extension(dx, dy) => f.write_fmt(format_args!("B({},{})", dx, dy)),
            Action::Fast => f.write_str("F"),
            Action::Drill => f.write_str("L"),
            Action::Reset => f.write_str("R"),
            Action::Teleport(x, y) => f.write_fmt(format_args!("T({},{})", x, y)),
            Action::CloneWorker => f.write_str("C"),
        }
    }
}

pub fn actions_to_string(list: &Vec<Action>) -> String {
    let mut out = String::new();
    for mv in list {
        out += &mv.to_string();
    }
    out
}

pub fn solution_to_string(lists: &[Vec<Action>]) -> String {
    let mut out = String::new();
    for (i, list) in lists.iter().enumerate() {
        if i != 0 {
            out += &"#";
        }
        out += &actions_to_string(list);
    }
    out
}
