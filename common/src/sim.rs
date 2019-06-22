use crate::*;

use reach::*;
use std::collections::*;
use std::vec::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WorkerState {
    pub x: usize,                         //・今いる座標
    pub y: usize,                         //
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
            x,
            y,
            manipulators: vec![(1, 0), (1, 1), (1, -1)],
            unused_boosters: vec![],
            ..Default::default()
        }
    }
    pub fn fill(&self, map: &mut Vec<Vec<Square>>) {
        for &manipurator in &self.manipulators {
            if is_visible(map, self.pos(), manipurator) {
                map[self.x][self.y] = Square::Filled;
            }
        }
    }
    pub fn pos(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

pub struct Update {
    filled: Vec<(usize, usize)>,
}

// Map への影響も考慮して動く
// - 動くたびに Fill する
// - Drill 中は Block も Fill にする
// - Fast 中に壁にぶつかると 1 step で止まる
pub fn apply_action(
    action: Action,
    worker: &mut WorkerState,
    map: &mut Vec<Vec<Square>>,
    booster: &mut Vec<Vec<Option<Booster>>>,
) -> Update {
    let size = (map.len(), map[0].len());
    let mut filled = vec![];
    match action {
        Action::Move(dir) => {
            let drilling = worker.drill_remaining > 0;
            let pos = apply_move(worker.pos(), dir);
            if within_mine(pos, size) && (drilling || map[pos.0][pos.1] != Square::Block) {
                worker.x = pos.0;
                worker.y = pos.1;
                if map[pos.0][pos.1] != Square::Filled {
                    map[pos.0][pos.1] = Square::Filled;
                    filled.push(pos);
                }
                if let Some(b) = booster[pos.0][pos.1].take() {
                    worker.unused_boosters.push(b);
                }
            } else {
                panic!("bad move to {:?}", pos);
            }
            if worker.fast_remaining > 0 {
                let pos = apply_move(worker.pos(), dir);
                if within_mine(pos, size) && (drilling || map[pos.0][pos.1] != Square::Block) {
                    worker.x = pos.0;
                    worker.y = pos.1;
                    if map[pos.0][pos.1] != Square::Filled {
                        map[pos.0][pos.1] = Square::Filled;
                        filled.push(pos);
                    }
                    if let Some(b) = booster[pos.0][pos.1].take() {
                        worker.unused_boosters.push(b);
                    }
                }
            }
        }
        Action::Nothing => (),
        Action::TurnR => {
            worker.dir += 1;
            worker.dir %= 4;
            for m in worker.manipulators.iter_mut() {
                let p = *m;
                m.0 = p.1;
                m.1 = -p.0;
            }
        }
        Action::TurnL => {
            worker.dir += 3;
            worker.dir %= 4;
            for m in worker.manipulators.iter_mut() {
                let p = *m;
                m.0 = -p.1;
                m.1 = p.0;
            }
        }
        Action::Extension(dx, dy) => worker.manipulators.push((dx, dy)),
        Action::Fast => {
            swap_remove_one_from_vec(&mut worker.unused_boosters, &Booster::Fast)
                .expect("no Fast remaining");
            worker.fast_remaining = 51;
        }
        Action::Drill => {
            swap_remove_one_from_vec(&mut worker.unused_boosters, &Booster::Drill)
                .expect("no Drill remaining");
            worker.drill_remaining = 31;
        }
        Action::Reset => {
            worker.beacons.insert(worker.pos());
        }
        Action::Teleport(x, y) => {
            let to = (x + 1, y + 1);
            if !worker.beacons.contains(&to) {
                panic!(
                    "teleporting to invalid beacon {:?} out of {:?}",
                    to, worker.beacons
                )
            }
            swap_remove_one_from_vec(&mut worker.unused_boosters, &Booster::Teleport);
            worker.x = x + 1;
            worker.y = y + 1;
        }
    }
    if worker.fast_remaining > 0 {
        worker.fast_remaining -= 1;
    }
    if worker.drill_remaining > 0 {
        worker.drill_remaining -= 1;
    }
    Update{
        filled,
    }
}

pub fn within_mine((x, y): (usize, usize), (w, h): (usize, usize)) -> bool {
    0 < x && x < w - 1 && 0 < y && y < h
}

fn swap_remove_one_from_vec<T: Eq>(v: &mut Vec<T>, t: &T) -> Option<T> {
    if let Some(i) = v.iter().position(|i| i == t) {
        let j = v.len() - 1;
        v.swap(i, j);
        v.pop()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_0() {
        let mut map = vec![vec![Square::Empty; 20]; 20];
        let mut booster = vec![vec![None; 20]; 20];
        let a = WorkerState::new(10, 10);
        let mut b = a.clone();
        apply_action(Action::Move(0), &mut b, &mut map, &mut booster);
        assert_eq!(
            b,
            WorkerState {
                x: 11,
                y: 10,
                ..a.clone()
            }
        );
    }

    #[test]
    fn turn_r() {
        let mut map = vec![vec![Square::Empty; 10]; 10];
        let mut booster = vec![vec![None; 10]; 10];
        let a = WorkerState::new(5, 5);
        let mut b = a.clone();
        apply_action(Action::TurnR, &mut b, &mut map, &mut booster);
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
        let mut map = vec![vec![Square::Empty; 10]; 10];
        let mut booster = vec![vec![None; 10]; 10];
        let a = WorkerState::new(5, 5);
        let mut b = a.clone();
        booster[5][4] = Some(Booster::Fast);
        apply_action(Action::Move(1), &mut b, &mut map, &mut booster);
        apply_action(Action::Fast, &mut b, &mut map, &mut booster);
        apply_action(Action::Move(0), &mut b, &mut map, &mut booster);
        assert_eq!(
            b,
            WorkerState {
                x: 7,
                y: 4,
                fast_remaining: 49,
                ..a.clone()
            }
        );
    }

    #[test]
    fn drill() {
        let mut map = vec![vec![Square::Empty; 10]; 10];
        let mut booster = vec![vec![None; 10]; 10];
        let a = WorkerState::new(5, 5);
        let mut b = a.clone();
        booster[5][4] = Some(Booster::Drill);
        apply_action(Action::Move(1), &mut b, &mut map, &mut booster);
        apply_action(Action::Drill, &mut b, &mut map, &mut booster);
        map[6][4] = Square::Block;
        apply_action(Action::Move(0), &mut b, &mut map, &mut booster);
        assert_eq!(
            b,
            WorkerState {
                x: 6,
                y: 4,
                drill_remaining: 29,
                ..a.clone()
            }
        );
        assert_ne!(map[6][4], Square::Block);
    }
}
