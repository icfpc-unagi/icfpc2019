use crate::*;

use std::collections::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LocalState {
    pub x: usize,                         //・今いる座標
    pub y: usize,                         //
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

pub fn apply_multi_action(
    actions: &[Action],
    workers: &mut WorkersState,
    map: &mut SquareMap,
    booster: &mut BoosterMap,
) -> Update {
    unimplemented!()
}
