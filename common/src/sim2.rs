use crate::*;
use crate::sim::{swap_remove_one_from_vec, within_mine};

use std::collections::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LocalState {
    pub x: usize,                         //・今いる座標
    pub y: usize,                         //
    pub dir: usize,                       // (deprecated?) ・向いている向き
    pub manipulators: Vec<(i32, i32)>,    // マニピュレータたちの位置
    pub fast_remaining: usize,            // Fast効果残り時間
    pub drill_remaining: usize,           // Drill効果残り時間
}

impl LocalState {
    pub fn new(x: usize, y: usize) -> LocalState {
        LocalState {
            x,
            y,
            manipulators: vec![(0, 0), (1, 0), (1, 1), (1, -1)],
            ..Default::default()
        }
    }

    pub fn clone_worker(&self) -> LocalState {
        LocalState::new(self.x, self.y)
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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SharedState {
    pub unused_boosters: Vec<Booster>,    //・持っている
    pub beacons: HashSet<(usize, usize)>, // Teleport Beacons
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WorkersState {
    pub locals: Vec<LocalState>,
    pub shared: SharedState,
}

impl WorkersState {
    pub fn new_t0(x: usize, y: usize, map: &mut SquareMap) -> WorkersState {
        // WorkerState::new2
        let locals = vec![LocalState::new(x, y)];
        locals[0].fill(map);
        WorkersState {
            locals,
            shared: SharedState::default(),
        }
    }
}

// from v1
impl From<WorkerState> for WorkersState {
    fn from(state: WorkerState) -> WorkersState {
        let WorkerState {x, y, dir, manipulators, unused_boosters, fast_remaining, drill_remaining, beacons} = state;
        WorkersState {
            locals: vec![LocalState {
                x, y, dir, manipulators, fast_remaining, drill_remaining
            }],
            shared: SharedState {
                unused_boosters, beacons
            },
        }
    }
}

// to v1
impl From<WorkersState> for WorkerState {
    fn from(state: WorkersState) -> WorkerState {
        let WorkersState { mut locals, shared } = state;
        let SharedState { unused_boosters, beacons } = shared;
        if locals.len() != 1 {
            panic!("v1 does not support cloned workers");
        }
        let local = locals.pop().unwrap();
        let LocalState {
            x, y, dir, manipulators, fast_remaining, drill_remaining
        } = local;
        WorkerState {
            x, y, dir, manipulators, unused_boosters, fast_remaining, drill_remaining, beacons
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Update {
    pub filled: Vec<(usize, usize)>,
    pub num_cloned: usize,
}

pub fn apply_multi_action(
    actions: &[Action],
    workers: &mut WorkersState,
    map: &mut SquareMap,
    booster: &mut BoosterMap,
) -> Update {
    let WorkersState { locals, shared } = workers;
    assert_eq!(actions.len(), locals.len());
    let n = actions.len();

    let size = (map.len(), map[0].len());
    let mut filled = vec![];
    let mut new_workers = vec![];

    for i in 0..n {
        let action = actions[i];
        let mut worker = locals.get_mut(i).unwrap();
        match action {
            Action::Move(dir) => {
                let drilling = worker.drill_remaining > 0;
                let pos = apply_move(worker.pos(), dir);
                if within_mine(pos, size) && (drilling || map[pos.0][pos.1] != Square::Block) {
                    worker.x = pos.0;
                    worker.y = pos.1;
                    if let Some(b) = booster[pos.0][pos.1].take() {
                        shared.unused_boosters.push(b);
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
                            shared.unused_boosters.push(b);
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
                swap_remove_one_from_vec(&mut shared.unused_boosters, &Booster::Extension)
                    .expect("no Extension remaining");
                worker.manipulators.push((dx, dy));
            },
            Action::Fast => {
                swap_remove_one_from_vec(&mut shared.unused_boosters, &Booster::Fast)
                    .expect("no Fast remaining");
                worker.fast_remaining = 51;
            }
            Action::Drill => {
                swap_remove_one_from_vec(&mut shared.unused_boosters, &Booster::Drill)
                    .expect("no Drill remaining");
                worker.drill_remaining = 31;
            }
            Action::Reset => {
                swap_remove_one_from_vec(&mut shared.unused_boosters, &Booster::Teleport);
                shared.beacons.insert(worker.pos());
            }
            Action::Teleport(x, y) => {
                let to = (x + 1, y + 1);
                if !shared.beacons.contains(&to) {
                    panic!(
                        "teleporting to invalid beacon {:?} out of {:?}",
                        to, shared.beacons
                    )
                }
                worker.x = x + 1;
                worker.y = y + 1;
            }
            Action::CloneWorker => {
                swap_remove_one_from_vec(&mut shared.unused_boosters, &Booster::CloneWorker)
                    .expect("no Clone remaining");
                new_workers.push(worker.clone_worker());
            }
        }
        filled.append(&mut worker.fill(map));
        if worker.fast_remaining > 0 {
            worker.fast_remaining -= 1;
        }
        if worker.drill_remaining > 0 {
            worker.drill_remaining -= 1;
        }
    }
    let num_cloned = new_workers.len();
    locals.append(&mut new_workers);
    Update { filled, num_cloned }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part3() {
        sim_golden(
            "../data/part-3-clones-examples/example-03.desc",
            "../data/part-3-clones-examples/example-03-1.sol",
        );
    }

    fn sim_golden(task_path: &str, sol_path: &str) {
        let (mut map, mut booster, init_x, init_y) = read_task(task_path);
        let solution = read_sol(sol_path);
        let mut state = WorkersState::new_t0(init_x, init_y, &mut map);

        let mut solution_iters = solution.iter().map(|actions| actions.iter()).collect::<Vec<_>>();
        loop {
            let num_workers = state.locals.len();
            let mut actions: Vec<Option<&Action>> = vec![];
            for i in 0..num_workers {
                actions.push(solution_iters[i].next());
            }
            if actions.iter().all(|a| a.is_none()) {
                break;
            }
            let actions = actions.into_iter().map(|a| *a.unwrap_or(&Action::Nothing)).collect::<Vec<_>>();
            let upd = apply_multi_action(&actions, &mut state, &mut map, &mut booster);
            eprintln!("{:?}", state);
        }
        // print_task(&(map.clone(), booster.clone(), worker.x, worker.y));
        assert!(map.iter().all(|v| v.iter().all(|&s| s != Square::Empty)));
    }
}
