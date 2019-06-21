use crate::*;

use map::*;
use std::collections::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WorkerState {
    pub pos: (usize, usize),              //・今いる座標
    pub dir: usize,                       //・向いている向き
    pub manipulators: Vec<(i32, i32)>,    // マニピュレータたちの相対位置（方向0のときの）
    pub unused_boosters: Vec<Booster>,    //・持っている
    pub fast_remaining: usize,            // Fast効果残り時間
    pub drill_remaining: usize,           // Drill効果残り時間
    pub beacons: HashSet<(usize, usize)>, // Teleport Beacons
}

impl WorkerState {
    pub fn new(x: usize, y: usize) -> WorkerState {
        WorkerState {
            pos: (x, y),
            manipulators: vec![(1, 0), (1, 1), (1, -1)],
            unused_boosters: vec![],
            ..Default::default()
        }
    }
}

pub fn apply_action(action: Action, w: &mut WorkerState, m: &mut MapState) {
    match action {
        Action::Move(dir) => {
            let pos = apply_move(w.pos, dir);
            w.pos = if w.fast_remaining > 0 && m.is_enterable(pos) {
                apply_move(pos, dir)
            } else {
                // TODO: Validate
                pos
            };
        }
        Action::Nothing => (),
        Action::TurnR => {
            w.dir += 1;
            w.dir %= 4;
            for m in w.manipulators.iter_mut() {
                let p = *m;
                m.0 = p.1;
                m.1 = -p.0;
            }
        }
        Action::TurnL => {
            w.dir += 3;
            w.dir %= 4;
            for m in w.manipulators.iter_mut() {
                let p = *m;
                m.0 = -p.1;
                m.1 = p.0;
            }
        }
        Action::Extension(dx, dy) => w.manipulators.push((dx, dy)),
        Action::Fast => {
            w.fast_remaining = 50;
        }
        Action::Drill => {
            w.drill_remaining = 30;
        }
        Action::Reset => {
            w.beacons.insert(w.pos);
        }
        Action::Teleport(x, y) => {
            if !w.beacons.contains(&(x, y)) {
                panic!()
            }
            w.pos = (x, y)
        }
    }
    if w.fast_remaining > 0 {
        w.fast_remaining -= 1;
    }
    if w.drill_remaining > 0 {
        w.drill_remaining -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut m = MapState::empty(10, 10);
        let a = WorkerState::new(10, 20);
        let mut b = a.clone();
        apply_action(Action::Move(0), &mut b, &mut m);
        assert_eq!(
            b,
            WorkerState {
                pos: (11, 20),
                ..a.clone()
            }
        );

        let mut b = a.clone();
        apply_action(Action::TurnR, &mut b, &mut m);
        assert_eq!(
            b,
            WorkerState {
                dir: 1,
                manipulators: vec![(0, -1,), (1, -1,), (-1, -1,),],
                ..a.clone()
            }
        );
    }
}
