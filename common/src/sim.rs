use crate::*;

use map::*;

use reach::*;
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
    pub fn fill(&self, map: &mut Vec<Vec<Square>>) {
        for &manipurator in &self.manipulators {
            if is_visible(map, self.pos, manipurator) {
                map[self.pos.0][self.pos.1] = Square::Filled;
            }
        }
    }
    pub fn x(&self) -> usize {self.pos.0}
    pub fn y(&self) -> usize {self.pos.1}
}

// Map への影響も考慮して動く
// - 動くたびに Fill する
// - Drill 中は Block も Fill にする
// - Fast 中に壁にぶつかると 1 step で止まる
pub fn apply_action(action: Action, w: &mut WorkerState, m: &mut MapState) {
    match action {
        Action::Move(dir) => {
            let pos = apply_move(w.pos, dir);
            if m.is_enterable(pos, w.drill_remaining > 0) {
                w.pos = pos;
                m.map[pos.0][pos.1] = Square::Filled;
                if let Some(b) = m.booster[pos.0][pos.1] {
                    w.unused_boosters.push(b);
                }
            } else {
                panic!("bad move");
            }
            if w.fast_remaining > 0 {
                let pos = apply_move(w.pos, dir);
                if m.is_enterable(pos, w.drill_remaining > 0) {
                    w.pos = pos;
                    m.map[pos.0][pos.1] = Square::Filled;
                    if let Some(b) = m.booster[pos.0][pos.1] {
                        w.unused_boosters.push(b);
                    }
                }
            }
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
            let i = w
                .unused_boosters
                .iter()
                .position(|&b| b == Booster::Fast)
                .expect("no Fast remaining");
            let j = w.unused_boosters.len() - 1;
            w.unused_boosters.swap(i, j);
            w.unused_boosters.pop();
            w.fast_remaining = 51;
        }
        Action::Drill => {
            let i = w
                .unused_boosters
                .iter()
                .position(|&b| b == Booster::Drill)
                .expect("no Drill remaining");
            let j = w.unused_boosters.len() - 1;
            w.unused_boosters.swap(i, j);
            w.unused_boosters.pop();
            w.drill_remaining = 31;
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
    fn move_0() {
        let mut m = MapState::empty(20, 20);
        let a = WorkerState::new(10, 10);
        let mut b = a.clone();
        apply_action(Action::Move(0), &mut b, &mut m);
        assert_eq!(
            b,
            WorkerState {
                pos: (11, 10),
                ..a.clone()
            }
        );
    }

    #[test]
    fn turn_r() {
        let mut m = MapState::empty(10, 10);
        let a = WorkerState::new(5, 5);
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

    #[test]
    fn fast() {
        let mut m = MapState::empty(10, 10);
        let a = WorkerState::new(5, 5);
        let mut b = a.clone();
        m.booster[5][4] = Some(Booster::Fast);
        apply_action(Action::Move(1), &mut b, &mut m);
        apply_action(Action::Fast, &mut b, &mut m);
        apply_action(Action::Move(0), &mut b, &mut m);
        assert_eq!(
            b,
            WorkerState {
                pos: (7, 4),
                fast_remaining: 49,
                ..a.clone()
            }
        );
    }

    #[test]
    fn drill() {
        let mut m = MapState::empty(10, 10);
        let a = WorkerState::new(5, 5);
        let mut b = a.clone();
        m.booster[5][4] = Some(Booster::Drill);
        apply_action(Action::Move(1), &mut b, &mut m);
        apply_action(Action::Drill, &mut b, &mut m);
        m.map[6][4] = Square::Block;
        apply_action(Action::Move(0), &mut b, &mut m);
        assert_eq!(
            b,
            WorkerState {
                pos: (6, 4),
                drill_remaining: 29,
                ..a.clone()
            }
        );
        assert_ne!(m.map[6][4], Square::Block);
    }
}
