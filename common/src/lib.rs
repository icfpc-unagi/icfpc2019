pub mod reach;
pub mod task;

pub use task::*;

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
    X,
}

impl std::str::FromStr for Booster {
    type Err = ();

    fn from_str(s: &str) -> Result<Booster, ()> {
        match s {
            "B" => Ok(Booster::Extension),
            "F" => Ok(Booster::Fast),
            "L" => Ok(Booster::Drill),
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
    Drill
}

pub fn actions_to_string(list: &Vec<Action>) -> String {
    let mut out = String::new();
    for mv in list {
        match mv {
            Action::Move(dir) => out += ["D", "S", "A", "W"][*dir],
            Action::Nothing => out += "Z",
            Action::TurnR => out += "E",
            Action::TurnL => out += "Q",
            Action::Extension(dx, dy) => out += &format!("B({},{})", dx, dy),
            Action::Fast => out += "F",
            Action::Drill => out += "L"
        }
    }
    out
}

pub struct PlayerState {
    x: usize,  //・今いる座標
    y: usize,
    dir: usize,  //・向いている向き
    unused_boosters: Vec<Booster>,  //・持っている
    active_boosters: Vec<(Booster, i32)>,  //・発動中の効果、残りターン
    manipulators: Vec<(i32, i32)>,  // マニピュレータたちの相対位置（方向0のときの）
}

/*
impl PlayerState {
    pub fn apply_action(&mut self, action: &Action) {
        match action {
            Action::Move(dir) =>
                // TODO: Fastがonだったら2マス移動したりする
                (self.x, self.y) = apply_move((self.x, self.y), *dir),
            Action::Nothing =>
                (),
            Action::TurnR => {
                self.dir += 1;
                self.dir %= 4;
            }
            Action::TurnL => {
                self.dir += 3;
                self.dir %= 4;
            }
            Action::Extension(dx, dy) =>
                self.manipulators.push((*dx, *dy)),
            Action::Fast =>
                // TODO: unused_boostersからfastを除いてactive_boostersに追加
                unimplemented!(),
            Action::Drill =>
                // TODO: マップに対する作用は一体？？？
                unimplemented!(),
        }

        // TODO: 発動中の効果の有効期限を減らしたりする
    }
}
*/
