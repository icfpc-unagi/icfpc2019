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

    pub fn apply(&mut self, state: &WorkerState) {
        let cells = state.visible_manipulators_on_empty_cells(&self.initial_square_map);
        for cell in cells {
            self.fill_count[cell.0][cell.1] += 1;
        }
    }

    pub fn cancel(&mut self, state: &WorkerState) {
        let cells = state.visible_manipulators_on_empty_cells(&self.initial_square_map);
        for cell in cells {
            self.fill_count[cell.0][cell.1] -= 1;
        }
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
        let mut dummy_booster_map = mat![None; xsize; ysize];

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

    pub fn deactivate(&mut self, begin: usize, end: usize) {
        assert!(begin < end);
        for state in &self.states[begin + 1..end] {
            self.dynamic_map.cancel(state);
        }
    }

    pub fn reactivate(&mut self, begin: usize, end: usize) {
        assert!(begin < end);
        for state in &self.states[begin + 1..end] {
            self.dynamic_map.apply(state);
        }
    }

    pub fn replace(&mut self, begin: usize, end: usize, new_actions: &[Action]) {
        // new_actionsは同じ場所にたどり着くこと！！

        assert!(begin < end);
        assert!(end <= self.actions.len());

        self.deactivate(begin, end);

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
            let new_end_state = new_states.last().unwrap();
            let original_end_state = &self.states[end];
            assert_eq!(new_end_state.x, original_end_state.x);
            assert_eq!(new_end_state.y, original_end_state.y);
            assert_eq!(new_end_state.dir, original_end_state.dir);
            assert_eq!(new_end_state.manipulators, original_end_state.manipulators);
        }

        let mut new_full_actions = vec![];
        new_full_actions.extend_from_slice(&self.actions[..begin]);
        new_full_actions.extend_from_slice(new_actions);
        new_full_actions.extend_from_slice(&self.actions[end..]);
        std::mem::swap(&mut self.actions, &mut new_full_actions);
        drop(new_full_actions);

        let mut new_full_states = vec![];
        new_full_states.extend_from_slice(&self.states[..begin + 1]);
        new_full_states.extend_from_slice(&new_states[..new_states.len() - 1]);
        new_full_states.extend_from_slice(&self.states[end..]);
        std::mem::swap(&mut self.states, &mut new_full_states);
        drop(new_full_states);

        self.reactivate(begin, begin + new_actions.len());
    }
}

pub fn optimize_local_tsp(task: &RasterizedTask, actions: &Vec<Action>) -> Vec<Action> {
    /*
    let mut dynamic_map = DynamicMap::new(task, actions);
    let mut dynamic_solution = DynamicSolution::new(task, actions);

    let k = 5;

    let i = 1;
    dynamic_map.cancel(&dynamic_solution.states[i]);
    */

    unimplemented!();
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

    #[test]
    fn it_works() {
        // タスクを準備。MoveとTurnしか入ってないやつ。
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

        let n_actions = actions.len();
        let mut rng = rand::thread_rng();
        let mut generate_random_range = || {
            loop {
                let (b, e) = (rng.gen_range(0, n_actions), rng.gen_range(0, n_actions));
                let (b, e) = (usize::min(b, e), usize::max(b, e));
                if b != e {
                    return (b, e)
                }
            }
        };

        // まずはdeactivvate単発
        for _ in 0..30 {
            let (b, e) = generate_random_range();

            let (_, sm_naive) = get_filled_square_map_naive(&task, actions, b, e - 1);
            print_map(&sm_naive);

            let mut dsol = DynamicSolution::new(&task, actions);
            dsol.deactivate(b, e);
            let sm_dynamic = dsol.dynamic_map.to_square_map();
            print_map(&sm_dynamic);

            assert_eq!(sm_naive, sm_dynamic);
        }

        // 次はdeactivate, activate連発
        {
            let mut dsol = DynamicSolution::new(&task, actions);

            for _ in 0..30 {
                let (b, e) = generate_random_range();

                let (_, sm_naive) = get_filled_square_map_naive(&task, actions, b, e - 1);
                print_map(&sm_naive);

                dsol.deactivate(b, e);
                let sm_dynamic = dsol.dynamic_map.to_square_map();
                dsol.reactivate(b, e);
                print_map(&sm_dynamic);

                assert_eq!(sm_naive, sm_dynamic);
            }
        }
    }
}
