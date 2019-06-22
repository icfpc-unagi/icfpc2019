
use crate::sol;
use crate::*;
use reach::*;
use std::collections::*;

use std::mem;
use std::vec::*;
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WorkerState {
    pub x: usize,                         //・今いる座標
    pub y: usize,                         //
    pub dir: usize,                       //・向いている向き
    pub manipulators: Vec<(i32, i32)>,    // マニピュレータたちの位置
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
            manipulators: vec![(0, 0), (1, 0), (1, 1), (1, -1)],
            unused_boosters: vec![],
            ..Default::default()
        }
    }
    #[deprecated(note="これを使うと最初のturnでboosterを使えない可能性あり。 `new3` を使って。")]
    pub fn new2(x: usize, y: usize, map: &mut SquareMap) -> WorkerState {
        let w = WorkerState::new(x, y);
        w.fill(map);
        w
    }
    pub fn new3(x: usize, y: usize, map: &mut SquareMap, booster: &mut BoosterMap) -> WorkerState {
        let mut w = WorkerState::new(x, y);
        w.fill(map);
        if let Some(b) = booster[x][y].take() {
            w.unused_boosters.push(b);
        }
        w
    }
    // Returns updated squares
    pub fn fill(&self, map: &mut SquareMap) -> Vec<(usize, usize)> {
        let mut filled = vec![];
        for &manipulator in &self.manipulators {
            if is_visible(map, self.pos(), manipulator) {
                let x = (self.x as i32 + manipulator.0) as usize;
                let y = (self.y as i32 + manipulator.1) as usize;
                if map[x][y] != Square::Filled {
                    map[x][y] = Square::Filled;
                    filled.push((x, y));
                }
            }
        }
        filled
    }
    pub fn pos(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

// Map への影響も考慮して動く
// - 動くたびに Fill する
// - Drill 中は Block も Fill にする
// - Fast 中に壁にぶつかると 1 step で止まる
pub fn apply_action(
    action: Action,
    worker: &mut WorkerState,
    map: &mut SquareMap,
    booster: &mut BoosterMap,
) -> Update {
    let mut workers = mem::replace(worker, WorkerState::default()).into();
    let upd = apply_multi_action(&[action], &mut workers, map, booster);
    mem::replace(worker, workers.into());
    // 1 workerしかいないので先にboosterをとったことにして良い
    // これを渡さないと `has_expand` が壊れる
    if let Some(b) = booster[worker.x][worker.y].take() {
        worker.unused_boosters.push(b);
    }
    upd
}


// もとの実装
fn apply_action_old(
    action: Action,
    worker: &mut WorkerState,
    map: &mut SquareMap,
    booster: &mut BoosterMap,
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
                if let Some(b) = booster[pos.0][pos.1].take() {
                    worker.unused_boosters.push(b);
                }
            } else {
                panic!("bad move to {:?}", pos);
            }
            if worker.fast_remaining > 0 {
                filled.append(&mut worker.fill(map)); // in the middle of fast steps
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
        Action::Extension(dx, dy) => {
            swap_remove_one_from_vec(&mut worker.unused_boosters, &Booster::Extension)
                .expect("no Extension remaining");
            worker.manipulators.push((dx, dy));
        }
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
            swap_remove_one_from_vec(&mut worker.unused_boosters, &Booster::Teleport);
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
            worker.x = x + 1;
            worker.y = y + 1;
        }
        Action::CloneWorker => unimplemented!(),
    }
    filled.append(&mut worker.fill(map));
    if worker.fast_remaining > 0 {
        worker.fast_remaining -= 1;
    }
    if worker.drill_remaining > 0 {
        worker.drill_remaining -= 1;
    }
    Update {
        filled,
        ..Update::default()
    }
}

pub fn within_mine((x, y): (usize, usize), (w, h): (usize, usize)) -> bool {
    0 < x && x < w - 1 && 0 < y && y < h
}

pub fn swap_remove_one_from_vec<T: Eq>(v: &mut Vec<T>, t: &T) -> Option<T> {
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
                manipulators: vec![(0, 0), (0, -1,), (1, -1,), (-1, -1,),],
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

    #[test]

    fn test_example_part1() {
        sim_golden(
            "../data/part-1-examples/example-01.desc",
            "../data/part-1-examples/example-01-1.sol",
        );
        sim_golden(
            "../data/part-1-examples/example-01.desc",
            "../data/part-1-examples/example-01-2.sol",
        );
        sim_golden(
            "../data/part-1-examples/example-01.desc",
            "../data/part-1-examples/example-01-3.sol",
        );
    }
    #[test]

    fn test_example_part2() {
        sim_golden(
            "../data/part-2-teleports-examples/example-02.desc",
            "../data/part-2-teleports-examples/example-02-1.sol",
        );
    }
    // 実装したらいれる
    // #[test]

    // fn test_example_part3() {
    //     sim_golden(
    //         "../data/part-3-clones-examples/example-03.desc",
    //         "../data/part-3-clones-examples/example-03-1.sol",
    //     );
    // }

    fn sim_golden(task_path: &str, sol_path: &str) {
        let (mut map, mut booster, init_x, init_y) = read_task(task_path);
        let sol = read_sol1(sol_path);
        let mut worker = WorkerState::new3(init_x, init_y, &mut map, &mut booster);
        for action in sol {
            apply_action(action, &mut worker, &mut map, &mut booster);
        }
        eprintln!("{:?}", worker);
        print_task(&(map.clone(), booster.clone(), worker.x, worker.y));
        assert!(map.iter().all(|v| v.iter().all(|&s| s != Square::Empty)));
    }
}
