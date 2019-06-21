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

    pub fn construct_actions(&mut self, mut x: usize, mut y: usize) -> Vec<Action> {
        let mut actions = vec![];
        loop {
            let (c, d) = self.flg[x][y];
            if c == 0 {
                break;
            }
            actions.push(Action::Move(d));

            let (tx, ty) = apply_move((x, y), (d + 2) % 4);
            x = tx;
            y = ty;
        }
        actions.reverse();
        actions
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

        let actions = self.construct_actions(target_x, target_y);
        self.clean_up();
        actions
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use rand::Rng;
        let mut rng = rand::thread_rng(); // デフォルトの乱数生成器を初期化します

        let task = load_task_002();
        let map = task.0;
        let xsize = map.len();
        let ysize = map[0].len();

        let mut random_empty_cell = || loop {
            let x: usize = rng.gen::<usize>() % xsize;
            let y: usize = rng.gen::<usize>() % ysize;
            if map[x][y] == Square::Empty {
                return (x, y);
            }
        };

        let mut bfs = BFS::new(&map);
        for _ in 0..100 {
            let (sx, sy) = random_empty_cell();
            let (tx, ty) = random_empty_cell();

            let mut ps = PlayerState::new_initial(sx, sy);
            let actions = bfs.search_fewest_actions_to_move(&ps, tx, ty);

            for a in actions.iter() {
                ps.apply_action(*a);
                assert_eq!(map[ps.x][ps.y], Square::Empty);
            }
            assert_eq!(ps.x, tx);
            assert_eq!(ps.y, ty);
        }
    }
}
