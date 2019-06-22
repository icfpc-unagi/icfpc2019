use crate::*;

use sim::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BFS {
    xsize: usize,
    ysize: usize,
    que_vec: Vec<(usize, usize)>,
    que_head: usize,
    pot: Vec<Vec<(usize, usize)>>,
    goals: Vec<(usize, usize)>,
    is_goal: Vec<Vec<(usize)>>, // ここにこの向きで来ればゴール。ここではplayer_stateのdirは無視し、現在との相対的な方向を書く。
}

fn rotate((mut x, mut y): (i32, i32), mut dir: usize) -> (i32, i32) {
    // TODO: もう少しどっか広く使えそうな場所置く？
    // TODO: 速くできる
    while dir > 0 {
        let (px, py) = (x, y);
        x = py;
        y = -px;
        dir -= 1;
    }
    (x, y)
}

impl BFS {
    pub fn new(xsize: usize, ysize: usize) -> BFS {
        BFS {
            xsize,
            ysize,
            que_vec: vec![],
            que_head: 0,
            pot: vec![vec![(!0, !0); ysize]; xsize],
            goals: vec![],
            is_goal: vec![vec![!0; ysize]; xsize],
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
        player_state: &WorkerState,
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

    pub fn search_with_goals(
        &mut self,
        map: &Vec<Vec<Square>>,
        player_state: &WorkerState,
    ) -> (Vec<Action>, usize, usize) {
        // Search
        let mut is_goal = vec![];
        std::mem::swap(&mut is_goal, &mut self.is_goal);
        let f = |x: usize, y: usize| is_goal[x][y] != !0;
        let (mut actions, x, y) = self.search(map, player_state, f);
        std::mem::swap(&mut is_goal, &mut self.is_goal);
        drop(is_goal);

        // Rotate
        if self.is_goal[x][y] == 3 {
            actions.push(Action::TurnL);
        } else {
            let mut d = self.is_goal[x][y];
            while d > 0 {
                actions.push(Action::TurnR);
                d -= 1;
            }
        }

        self.clean_up();
        (actions, x, y)
    }

    //
    // 外向け
    //

    pub fn search_fewest_actions_to_satisfy<F: Fn(usize, usize) -> bool>(
        &mut self,
        map: &Vec<Vec<Square>>,
        player_state: &WorkerState,
        condition_func: F,
    ) -> (Vec<Action>, usize, usize) {
        let ret = self.search(map, player_state, condition_func);
        self.clean_up();
        ret
    }

    pub fn search_fewest_actions_to_move(
        &mut self,
        map: &Vec<Vec<Square>>,
        player_state: &WorkerState,
        target_x: usize,
        target_y: usize,
    ) -> Vec<Action> {
        self.add_goal(target_x, target_y, player_state.dir);
        self.search_with_goals(map, player_state).0
    }

    // 現状の実相だと、1, 2ぐらいsuboptimalな可能性がある
    // 真面目な実装にするためにはコストがかかるがそれがペイするならやる
    pub fn search_fewest_actions_to_wrap(
        &mut self,
        map: &Vec<Vec<Square>>,
        player_state: &WorkerState,
        target_x: usize,
        target_y: usize,
    ) -> (Vec<Action>, usize, usize) {
        for (mx, my) in player_state.manipulators.iter() {
            for &d in [0, 1, 3, 2].iter() {
                let (dx, dy) = rotate((*mx, *my), (d + 2) % 4);
                let (tx, ty) = (target_x + (dx as usize), target_y + (dy as usize));
                // dbg!(&(dx, dy, tx, ty));
                if self.xsize <= tx || self.ysize <= ty {
                    continue;
                }
                if !is_visible(map, (target_x, target_y), (dx, dy)) {
                    continue;
                }
                if self.is_goal[tx][ty] != !0 {
                    continue;
                }
                self.add_goal(tx, ty, d);
            }
        }

        self.search_with_goals(map, player_state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_stress() {
        let tasks = [load_task_001(), load_task_002()];
        for task in tasks.iter() {
            use rand::Rng;
            let mut rng = rand::thread_rng(); // デフォルトの乱数生成器を初期化します

            let map = &task.0;
            let booster = &task.1;
            let xsize = map.len();
            let ysize = map[0].len();

            let mut random_empty_cell = |rng: &mut rand::rngs::ThreadRng| loop {
                let x: usize = rng.gen::<usize>() % xsize;
                let y: usize = rng.gen::<usize>() % ysize;
                if map[x][y] == Square::Empty {
                    return (x, y);
                }
            };

            let mut bfs = BFS::new(map.len(), map[0].len());
            for _ in 0..100 {
                let (sx, sy) = random_empty_cell(&mut rng);
                let (tx, ty) = random_empty_cell(&mut rng);

                let mut ps = WorkerState::new(sx, sy);
                ps.dir = rng.gen::<usize>() % 4;
                let actions = bfs.search_fewest_actions_to_move(&map, &ps, tx, ty);

                let mut m = map.clone();
                let mut b = booster.clone();
                for a in actions.iter() {
                    if let Action::Move(_) = a {
                    } else {
                        assert!(false);
                    }

                    apply_action(*a, &mut ps, &mut m, &mut b);
                    assert_eq!(map[ps.x][ps.y], Square::Empty);
                }
                assert_eq!(ps.x, tx);
                assert_eq!(ps.y, ty);
            }
        }
    }

    #[test]
    fn wrap_stress() {
        let tasks = [load_task_001(), load_task_002()];
        for task in tasks.iter() {
            use rand::Rng;
            let mut rng = rand::thread_rng(); // デフォルトの乱数生成器を初期化します

            let map = &task.0;
            let booster = &task.1;
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

                let mut ps = WorkerState::new(sx, sy);
                ps.manipulators.push((2, 5)); // MAJI YABAI DESU

                let (actions, gx, gy) = bfs.search_fewest_actions_to_wrap(&map, &ps, tx, ty);

                let mut m = map.clone();
                let mut b = booster.clone();
                for a in actions.iter() {
                    apply_action(*a, &mut ps, &mut m, &mut b);
                    assert_eq!(map[ps.x][ps.y], Square::Empty);
                }
                assert_eq!(ps.x, gx);
                assert_eq!(ps.y, gy);

                let mut f = false;
                for m in ps.manipulators.iter() {
                    if (ps.x + (m.0 as usize), ps.y + (m.1 as usize)) == (tx, ty) {
                        dbg!(m);
                        f = true;
                        assert!(is_visible(map, (ps.x, ps.y), *m));
                    }
                }
                assert_eq!(f, true);
            }
        }
    }
}
