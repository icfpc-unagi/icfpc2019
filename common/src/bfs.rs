use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BFS<'a> {
    map: &'a Vec<Vec<Square>>,
    que_vec: Vec<(usize, usize)>,
    que_head: usize,
    flg: Vec<Vec<(usize, usize)>>,
}

impl<'a> BFS<'a> {
    pub fn new(map: &Vec<Vec<Square>>) -> BFS {
        BFS {
            map,
            que_vec: vec![],
            que_head: 0,
            flg: vec![vec![(!0, !0); map[0].len()]; map.len()],
        }
    }

    pub fn clean_up(&mut self) {
        for (x, y) in self.que_vec.iter() {
            self.flg[*x][*y] = (!0, !0);
        }
        self.que_vec.clear();
        self.que_head = 0;
    }

    pub fn search_fewest_actions_to_move(
        &mut self,
        player_state: &PlayerState,
        target_x: usize,
        target_y: usize,
    ) -> Vec<Action> {
        self.que_vec.push((player_state.x, player_state.y));
        self.flg[player_state.x][player_state.y].0 = 0;

        while self.que_head < self.que_vec.len() {
            let (x, y) = self.que_vec[self.que_head];
            self.que_head += 1;

            if (x, y) == (target_x, target_y) {
                eprintln!("{}", self.flg[x][y].0);
                break;
            }

            let c = self.flg[x][y].0;
            for d in 0..4 {
                let (tx, ty) = apply_move((x, y), d);

                if self.map[tx][ty] == Square::Block || self.flg[tx][ty].0 != !0 {
                    continue;
                }

                self.flg[tx][ty] = (c + 1, d);
                self.que_vec.push((tx, ty));
            }
        }

        // TODO: 復元

        self.clean_up();

        return vec![];
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
