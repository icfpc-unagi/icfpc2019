use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BFS<'a> {
    map: &'a Vec<Vec<Square>>,
}

impl<'a> BFS<'a> {
    pub fn new(map: &Vec<Vec<Square>>) -> BFS {
        BFS { map }
    }

    pub fn search_fewest_actions_to_move(
        &mut self,
        player_state: &PlayerState,
        x: usize,
        y: usize,
    ) -> Vec<Action> {
        unimplemented!();
    }

    pub fn search_fewest_actions_to_wrap(
        &mut self,
        player_state: &PlayerState,
        x: usize,
        y: usize,
    ) -> Vec<Action> {
        unimplemented!();
    }
}
