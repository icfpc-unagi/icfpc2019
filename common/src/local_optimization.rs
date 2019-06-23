/*
注意：
* まずはfastとかを全く考えない。居場所だけがfillされると思って実装している。
*/

use crate::*;

//
// ユーティリティ（あとでしかるべき場所に移しても良い）
//

fn apply_actions(
    actions: &[Action],
    state: &mut WorkerState,
    square_map: &mut SquareMap,
    booster_map: &mut BoosterMap,
) -> Vec<Update> {
    actions
        .iter()
        .map(|action| apply_action(*action, state, square_map, booster_map))
        .collect()
}

fn print_map(square_map: &SquareMap) {
    let xsize = square_map.len();
    let ysize = square_map[0].len();

    for y in (0..ysize).rev() {
        eprint!("{:02}:", y);
        for x in 0..xsize {
            eprint!(
                "{}",
                match square_map[x][y] {
                    Square::Empty => ' ',
                    Square::Block => '#',
                    Square::Filled => '.',
                }
            );
        }
        eprintln!();
    }
}

fn get_initial_state(task: &RasterizedTask) -> WorkerState {
    WorkerState::new3(task.2, task.3, &mut task.0.clone(), &mut task.1.clone())
}

//
// 構造体いろいろ
//

pub struct DynamicMap {
    initial_square_map: SquareMap,
    fill_count: Vec<Vec<usize>>,
}

impl DynamicMap {
    pub fn new(task: &RasterizedTask) -> DynamicMap {
        let (xsize, ysize) = get_xysize(&task.0);

        DynamicMap {
            initial_square_map: task.0.clone(),
            fill_count: mat![0; xsize; ysize],
        }
    }

    pub fn apply(&mut self, state: &WorkerState) -> usize {
        let cells = state.visible_manipulators_on_empty_cells(&self.initial_square_map);
        let mut n = 0;
        for cell in cells {
            if self.fill_count[cell.0][cell.1] == 0 {
                n += 1;
            }
            self.fill_count[cell.0][cell.1] += 1;
        }
        n
    }

    pub fn cancel(&mut self, state: &WorkerState) -> usize {
        let cells = state.visible_manipulators_on_empty_cells(&self.initial_square_map);
        let mut n = 0;
        for cell in cells {
            self.fill_count[cell.0][cell.1] -= 1;
            if self.fill_count[cell.0][cell.1] == 0 {
                n += 1;
            }
        }
        n
    }

    pub fn to_square_map(&self) -> SquareMap {
        let (xsize, ysize) = get_xysize(&self.initial_square_map);
        let mut ret = self.initial_square_map.clone();
        for x in 0..xsize {
            for y in 0..ysize {
                let m = self.initial_square_map[x][y];
                let c = self.fill_count[x][y];
                if m == Square::Empty {
                    ret[x][y] = if c == 0 {
                        Square::Empty
                    } else {
                        Square::Filled
                    };
                } else {
                    assert_eq!(m, Square::Block);
                    assert_eq!(c, 0);
                }
            }
        }
        ret
    }

    pub fn num_filled_squares(&self) -> usize {
        let (xsize, ysize) = get_xysize(&self.initial_square_map);
        let mut n = 0;
        for x in 0..xsize {
            for y in 0..ysize {
                if self.fill_count[x][y] > 0 {
                    n += 1;
                }
            }
        }
        n
    }
}

pub struct DynamicSolution {
    actions: Vec<Action>,
    states: Vec<WorkerState>,
    dynamic_map: DynamicMap,
    dummy_square_map: SquareMap,   // めちゃくちゃになる
    dummy_booster_map: BoosterMap, // めちゃくちゃになる
}

impl DynamicSolution {
    pub fn new(task: &RasterizedTask, actions: &Vec<Action>) -> DynamicSolution {
        let (xsize, ysize) = get_xysize(&task.0);

        let mut dummy_square_map = task.0.clone();
        let mut dummy_booster_map = task.1.clone();

        let mut state = get_initial_state(&task);
        let mut states = vec![state.clone()];
        for action in actions {
            apply_action(
                *action,
                &mut state,
                &mut dummy_square_map,
                &mut dummy_booster_map,
            );
            states.push(state.clone());
        }

        let mut dynamic_map = DynamicMap::new(task);
        for state in &states {
            dynamic_map.apply(state);
        }

        DynamicSolution {
            actions: actions.clone(),
            dynamic_map,
            states,
            dummy_square_map,
            dummy_booster_map,
        }
    }

    //
    // こいつらはstate側の数え方であることに注意
    //
    pub fn deactivate_step(&mut self, step: usize) -> usize {
        self.dynamic_map.cancel(&self.states[step])
    }

    pub fn reactivate_step(&mut self, step: usize) -> usize {
        self.dynamic_map.apply(&self.states[step])
    }

    pub fn deactivate_range(&mut self, begin: usize, end: usize) -> usize {
        // step begin, step endは踏む。その間を除く。
        assert!(begin < end);
        let mut n = 0;
        for step in begin + 1..end {
            n += self.deactivate_step(step);
        }
        n
    }

    pub fn reactivate_range(&mut self, begin: usize, end: usize) -> usize {
        assert!(begin < end);
        let mut n = 0;
        for step in begin + 1..end {
            n += self.reactivate_step(step);
        }
        n
    }

    pub fn replace(&mut self, begin: usize, end: usize, new_actions: &[Action]) {
        // step beginとstep endは踏む。つまり、stepは(begin, end)が置き換わる。
        // actionでいうと[begin, end)が置き換わる。
        // (begin, end) は既にdeactivateされていること！

        assert!(begin < end);
        assert!(end <= self.actions.len());

        let mut new_states = vec![];
        let mut state = self.states[begin].clone();
        for action in new_actions {
            apply_action(
                *action,
                &mut state,
                &mut self.dummy_square_map,
                &mut self.dummy_booster_map,
            );
            new_states.push(state.clone());
        }

        {
            let new_end_state = new_states.pop().unwrap(); // 注意！POPしてるよ！！
            let original_end_state = &self.states[end];
            assert_eq!(new_end_state.x, original_end_state.x);
            assert_eq!(new_end_state.y, original_end_state.y);
            assert_eq!(new_end_state.dir, original_end_state.dir);
            assert_eq!(new_end_state.manipulators, original_end_state.manipulators);
        }

        {
            let mut new_full_states = vec![];
            new_full_states.extend_from_slice(&self.states[..begin + 1]);
            new_full_states.extend_from_slice(&new_states);
            new_full_states.extend_from_slice(&self.states[end..]);
            std::mem::swap(&mut self.states, &mut new_full_states);
        }

        {
            eprintln!("{:?} {:?}", &self.actions[begin..end], new_actions);

            let mut new_full_actions = vec![];
            new_full_actions.extend_from_slice(&self.actions[..begin]);
            new_full_actions.extend_from_slice(new_actions);
            new_full_actions.extend_from_slice(&self.actions[end..]);
            std::mem::swap(&mut self.actions, &mut new_full_actions);
        }

        self.reactivate_range(begin, begin + new_actions.len());
    }
}

pub fn optimize_pure_move(task: &RasterizedTask, actions: &Vec<Action>) -> Vec<Action> {
    // 全く塗ってない移動を最適化する
    let mut dsol = DynamicSolution::new(task, actions);
    let (xsize, ysize) = get_xysize(&task.0);
    let mut bfs = BFS::new(xsize, ysize);

    // 後ろからやっていって、extensionを踏んだらやめる。
    let mut begin = dsol.states.len() - 2;
    while begin != !0 {
        match dsol.actions[begin] {
            Action::TurnL => (),
            Action::TurnR => (),
            Action::Move(_) => (),
            Action::Nothing => (),
            _ => break,
        }

        // state beginは踏んだまま。endも踏んだまま。(begin, end) を消しても、大丈夫。というところを探す。
        let mut end = begin + 1;
        while end + 1 < dsol.states.len() {
            // endをふまない、というのを試してみて大丈夫だったら進む、endは踏むことにしてbreak
            let diff = dsol.deactivate_step(end);
            if diff > 0 {
                let diff2 = dsol.reactivate_step(end);
                assert_eq!(diff, diff2);
                break;
            }
            end += 1;
        }

        // より良い移動の仕方を入手する
        let begin_state = &dsol.states[begin];
        let end_state = &dsol.states[end];
        let mut new_actions = bfs.search_fewest_actions_to_move(
            &task.0,
            &dsol.states[begin],
            end_state.x,
            end_state.y,
        );
        let dir_diff = (4 + end_state.dir - begin_state.dir) % 4;
        new_actions.extend_from_slice(match dir_diff {
            0 => &[],
            1 => &[Action::TurnR],
            2 => &[Action::TurnR, Action::TurnR],
            3 => &[Action::TurnL],
            _ => panic!(),
        });

        let n_original_actions = end - begin;
        let n_new_actions = new_actions.len();

        if n_new_actions < n_original_actions {
            dbg!((begin, end, n_original_actions, n_new_actions));
            dsol.replace(begin, end, &new_actions);
        } else {
            let diff3 = dsol.reactivate_range(begin, end);
            assert_eq!(diff3, 0);
        }

        begin -= 1;
    }

    eprintln!("Optimization till: {}", begin);
    dbg!(dsol.actions.len());
    dsol.actions
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn get_filled_square_map_naive(
        task: &RasterizedTask,
        actions: &Vec<Action>,
        exclude_begin: usize,
        exclude_end: usize,
    ) -> (WorkerState, SquareMap) {
        // chokudaiさんのshortening関数からパクってきてリファクタリングした感じ
        let mut square_map = task.0.clone();
        let mut booster_map = task.1.clone();
        let (xsize, ysize) = get_xysize(&square_map);

        let mut current_state =
            WorkerState::new3(task.2, task.3, &mut square_map, &mut booster_map);
        let (mut current_square_map, mut current_booster_map) =
            (square_map.clone(), booster_map.clone());

        apply_actions(
            &actions[0..exclude_begin],
            &mut current_state,
            &mut current_square_map,
            &mut current_booster_map,
        );

        let (begin_state, begin_square_map, begin_booster_map) = (
            current_state.clone(),
            current_square_map.clone(),
            current_booster_map.clone(),
        );

        apply_actions(
            &actions[exclude_begin..exclude_end],
            &mut current_state,
            &mut current_square_map,
            &mut current_booster_map,
        );

        // print_map(&begin_square_map);

        current_square_map = begin_square_map.clone();
        current_booster_map = begin_booster_map.clone();
        let end_state = current_state.clone();

        apply_actions(
            &actions[exclude_end..],
            &mut current_state,
            &mut current_square_map,
            &mut current_booster_map,
        );

        (current_state, current_square_map)
    }

    fn prepare_task_and_actions() -> (RasterizedTask, Vec<Action>) {
        let task = load_task_002();
        let sol = parse_sol("DQWWWWWWWWEDDDESSSSSSSSQDSDDDDDDDDDDDWWWAAWWWWWDDWWWWWWDDESSASAASWWWWEEWWWWWWWEDDDDDDESSSSSSWWWAAEAEDDDDWWDDWSDSSSSSSSSQAADDQSSSSSSSSSSSSSSSQDSDDDQWWWWEDSSSDDDDQWWAWWWWQAWAEWWEDDDDQWWWWAWWSEDSDDSSSSSSDDDESEAAAAWWAAASSDSSSSSSSASAAAAAAAAAWWAAEWWWWWWWSAAAAWWSSDQQSSSSSSSSSSSEAAAQSASSSSSSSSSSEAAAEWWWWWWWDWWWWSSSSSSSSSSSASAAAAAAAWWWWWWWWWEDESSSSSSAAASSWWWWWWWAWWWWQDDAAQWWDWWWWEDDDDDDDDDDDDDESSSWAAAAAAWAAAASAS");
        assert_eq!(sol.len(), 1);
        let actions = &sol[0];
        eprintln!("{}", actions.len());

        for action in actions {
            match action {
                Action::Move(_) => (),
                Action::TurnR => (),
                Action::TurnL => (),
                _ => panic!(),
            }
        }

        (task, actions.clone())
    }

    #[test]
    fn test_deactivate_reactivate() {
        let (task, actions) = prepare_task_and_actions();

        let n_actions = actions.len();
        let mut rng = rand::thread_rng();
        let mut generate_random_range = || loop {
            let (b, e) = (rng.gen_range(0, n_actions), rng.gen_range(0, n_actions));
            let (b, e) = (usize::min(b, e), usize::max(b, e));
            if b != e {
                return (b, e);
            }
        };

        // まずはdeactivvate単発
        for _ in 0..30 {
            let (b, e) = generate_random_range();

            let (_, sm_naive) = get_filled_square_map_naive(&task, &actions, b, e);
            print_map(&sm_naive);

            let mut dsol = DynamicSolution::new(&task, &actions);
            dsol.deactivate_range(b, e + 1);
            let sm_dynamic = dsol.dynamic_map.to_square_map();
            print_map(&sm_dynamic);

            assert_eq!(sm_naive, sm_dynamic);
        }

        // 次はdeactivate, activate連発
        {
            let mut dsol = DynamicSolution::new(&task, &actions);

            for _ in 0..30 {
                let (b, e) = generate_random_range();

                let (_, sm_naive) = get_filled_square_map_naive(&task, &actions, b, e);
                print_map(&sm_naive);

                dsol.deactivate_range(b, e + 1);
                let sm_dynamic = dsol.dynamic_map.to_square_map();
                dsol.reactivate_range(b, e + 1);
                print_map(&sm_dynamic);

                assert_eq!(sm_naive, sm_dynamic);
            }
        }
    }

    #[test]
    fn test_optimize() {
        let (task, mut full_actions) = prepare_task_and_actions();

        let mut rng = rand::thread_rng();
        for _ in 0..30 {
            //let n_actions = rng.gen_range(5, full_actions.len());
            //let mut actions = full_actions[..n_actions].to_vec();
            let mut actions = full_actions.clone();

            for _ in 0..20 {
                let i = rng.gen_range(0, actions.len() - 1);
                actions.insert(i, Action::TurnR);
                actions.insert(i, Action::TurnL);
            }
            for _ in 0..20 {
                let i = rng.gen_range(0, actions.len() - 1);
                actions.insert(i, Action::TurnR);
                actions.insert(i, Action::TurnR);
                actions.insert(i, Action::TurnR);
                actions.insert(i, Action::TurnR);
            }

            let optimized_actions = optimize_pure_move(&task, &actions);

            //  dbg!(&actions);
            // dbg!(&optimized_actions);

            let (_, sm1) = get_filled_square_map_naive(&task, &actions, 0, 0);
            let (_, sm2) = get_filled_square_map_naive(&task, &optimized_actions, 0, 0);
            // print_map(&sm1);
            // print_map(&sm2);
            assert_eq!(sm1, sm2);
            eprintln!("{} {}", actions.len(), optimized_actions.len());
        }

        /*
        actions.insert(10, Action::TurnR);
        actions.insert(10, Action::TurnR);
        actions.insert(10, Action::TurnR);
        actions.insert(10, Action::TurnR);
        */
    }
}
