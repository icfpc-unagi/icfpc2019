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
    // shared.unused_boosters.push(Booster::X);
    for (action, worker) in actions.iter().zip(locals.iter()) {
    }
    unimplemented!()
}
