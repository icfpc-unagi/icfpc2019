use crate::*;

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

pub fn apply_multi_action(
    actions: &[Action],
    workers: &mut WorkersState,
    map: &mut SquareMap,
    booster: &mut BoosterMap,
) -> Update {
    let WorkersState { locals, shared } = workers;
    assert_eq!(actions.len(), locals.len());

    let size = (map.len(), map[0].len());
    let mut filled = vec![];
    for (action, worker) in actions.iter().zip(locals.iter()) {
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
        filled.append(&mut worker.fill(map));
        if worker.fast_remaining > 0 {
            worker.fast_remaining -= 1;
        }
        if worker.drill_remaining > 0 {
            worker.drill_remaining -= 1;
        }
    }
    Update { filled }
}
