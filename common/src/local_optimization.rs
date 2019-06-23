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

pub struct DynamicMap {}

impl DynamicMap {
    pub fn new(task: &RasterizedTask, actions: &Vec<Action>) -> DynamicMap {
        //let square_map
        //let state = WorkerState::new3;
        unimplemented!();
    }

    pub fn cancel(&mut self, state: &WorkerState) {
        unimplemented!();
    }
}

pub struct DynamicSolution {
    actions: Vec<Action>,
    states: Vec<WorkerState>,
}

impl DynamicSolution {
    pub fn new(task: &RasterizedTask, actions: &Vec<Action>) -> DynamicSolution {
        unimplemented!();
    }
}

pub fn create_fill_map_naive(
    task: &RasterizedTask,
    actions: &Vec<Action>,
    exclude_begin: usize,
    exclude_end: usize,
) -> (WorkerState, SquareMap) {
    // chokudaiさんのshortening関数からパクってきてリファクタリングした感じ
    let mut square_map = task.0.clone();
    let mut booster_map = task.1.clone();
    let (xsize, ysize) = get_xysize(&square_map);

    let mut current_state = WorkerState::new3(task.2, task.3, &mut square_map, &mut booster_map);
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

pub fn optimize_local_tsp(task: &RasterizedTask, actions: &Vec<Action>) -> Vec<Action> {
    let mut dynamic_map = DynamicMap::new(task, actions);
    let mut dynamic_solution = DynamicSolution::new(task, actions);

    let k = 5;

    let i = 1;
    dynamic_map.cancel(&dynamic_solution.states[i]);

    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let task = load_task_002();
        let sol = parse_sol("DQWWWWWWWWEDDDESSSSSSSSQDSDDDDDDDDDDDWWWAAWWWWWDDWWWWWWDDESSASAASWWWWEEWWWWWWWEDDDDDDESSSSSSWWWAAEAEDDDDWWDDWSDSSSSSSSSQAADDQSSSSSSSSSSSSSSSQDSDDDQWWWWEDSSSDDDDQWWAWWWWQAWAEWWEDDDDQWWWWAWWSEDSDDSSSSSSDDDESEAAAAWWAAASSDSSSSSSSASAAAAAAAAAWWAAEWWWWWWWSAAAAWWSSDQQSSSSSSSSSSSEAAAQSASSSSSSSSSSEAAAEWWWWWWWDWWWWSSSSSSSSSSSASAAAAAAAWWWWWWWWWEDESSSSSSAAASSWWWWWWWAWWWWQDDAAQWWDWWWWEDDDDDDDDDDDDDESSSWAAAAAAWAAAASAS");
        assert_eq!(sol.len(), 1);
        let sol = sol[0].clone();
        eprintln!("{}", sol.len());

        let (_, m1) = create_fill_map_naive(&task, &sol, 0, 0);
        print_map(&m1);

        let (_, m1) = create_fill_map_naive(&task, &sol, 5, 10);
        print_map(&m1);
    }
}
