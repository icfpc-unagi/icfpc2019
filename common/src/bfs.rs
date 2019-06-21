use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BFS {
    xsize: usize,
    ysize: usize,
    que_vec: Vec<(usize, usize)>,
    que_head: usize,
    pot: Vec<Vec<(usize, usize)>>,
    goals: Vec<(usize, usize)>,
    is_goal: Vec<Vec<(usize)>>, // ここにこの向きで来ればゴール
}

fn rotate((mut x, mut y): (i32, i32), mut dir: usize) {
    // TODO: もう少しどっか広く使えそうな場所置く？
    // TODO: 速くできる
    while dir > 0 {
        let (px, py) = (x, y);
        x = y;
        y = -x;
        dir -= 0;
    }
}

impl BFS {
    pub fn new(xsize: usize, ysize: usize) -> BFS {
        BFS {
            xsize,
            ysize,
            que_vec: vec![],
            que_head: 0,
            pot: vec![vec![(!0, !0); xsize]; ysize],
            goals: vec![],
            is_goal: vec![vec![!0; xsize]; ysize],
        }
    }

    fn clean_up(&mut self) {
        for (x, y) in self.que_vec.iter() {
            self.pot[*x][*y] = (!0, !0);
        }
        self.que_vec.clear();
        self.que_head = 0;

        for (x, y) in self.goals.iter() {
            self.is_goal[*x][*y] = !0;
        }
        self.goals.clear();
    }

    fn add_goal(&mut self, x: usize, y: usize, d: usize) {
        self.goals.push((x, y));
        self.is_goal[x][y] = d;
    }

    fn construct_actions(&mut self, mut x: usize, mut y: usize) -> Vec<Action> {
        let mut actions = vec![];

        // self.is_goal[x][y]

        loop {
            let (c, d) = self.pot[x][y];
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

    fn search<F: Fn(usize, usize) -> bool>(
        &mut self,
        map: &Vec<Vec<Square>>,
        player_state: &PlayerState,
        condition_func: F,
    ) -> (Vec<Action>, usize, usize) {
        self.que_vec.push((player_state.x, player_state.y));
        self.pot[player_state.x][player_state.y].0 = 0;

        let mut x = !0;
        let mut y = !0;

        while self.que_head < self.que_vec.len() {
            x = self.que_vec[self.que_head].0;
            y = self.que_vec[self.que_head].1;
            self.que_head += 1;

            if condition_func(x, y) {
                // eprintln!("{}", self.flg[x][y].0);
                break;
            }

            let c = self.pot[x][y].0;
            for d in 0..4 {
                let (tx, ty) = apply_move((x, y), d);

                if map[tx][ty] == Square::Block || self.pot[tx][ty].0 != !0 {
                    continue;
                }

                self.pot[tx][ty] = (c + 1, d);
                self.que_vec.push((tx, ty));
            }
        }

        (self.construct_actions(x, y), x, y)
    }

    pub fn search_fewest_actions_to_satisfy<F: Fn(usize, usize) -> bool>(
        &mut self,
        map: &Vec<Vec<Square>>,
        player_state: &PlayerState,
        condition_func: F,
    ) -> (Vec<Action>, usize, usize) {
        let ret = self.search(map, player_state, condition_func);
        self.clean_up();
        ret
    }

    pub fn search_fewest_actions_to_move(
        &mut self,
        map: &Vec<Vec<Square>>,
        player_state: &PlayerState,
        target_x: usize,
        target_y: usize,
    ) -> Vec<Action> {
        self.add_goal(target_x, target_y, player_state.dir);
        let mut is_goal = vec![];
        std::mem::swap(&mut is_goal, &mut self.is_goal);
        let f = |x: usize, y: usize| is_goal[x][y] != !0;
        let (actions, _, _) = self.search(map, player_state, f);
        std::mem::swap(&mut is_goal, &mut self.is_goal);
        self.clean_up();
        actions
    }

    pub fn search_fewest_actions_to_wrap(
        &mut self,
        map: &Vec<Vec<Square>>,
        player_state: &PlayerState,
        target_x: usize,
        target_y: usize,
    ) -> Vec<Action> {
        unimplemented!();

        /*
        for (mx, my) in player_state.manipulators.iter() {
            for d in 0..4 {
                let (dx, dy) = rotate((*mx, *my), (d + 2) % 4);
                let (tx, ty) = (target_x + dx, target_y + dy);
                if self.xsize <= tx || self.ysize <= ty {
                    continue;
                }
                self.add_goal(tx, ty, d);
            }
        }

        // TODO: ここで回転する
        */
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

        let mut bfs = BFS::new(map.len(), map[0].len());
        for _ in 0..100 {
            let (sx, sy) = random_empty_cell();
            let (tx, ty) = random_empty_cell();

            let mut ps = PlayerState::new(sx, sy);
            let actions = bfs.search_fewest_actions_to_move(&map, &ps, tx, ty);

            for a in actions.iter() {
                ps.apply_action(*a);
                assert_eq!(map[ps.x][ps.y], Square::Empty);
            }
            assert_eq!(ps.x, tx);
            assert_eq!(ps.y, ty);
        }
    }
}
