use crate::*;

fn get_worker_spawn_steps(task: &RasterizedTask, sol: &Vec<Vec<Action>>) -> Vec<usize> {
    let mut square_map = task.0.clone();
    let mut booster_map = task.1.clone();
    let mut state = WorkersState::new_t0(task.2, task.3, &mut square_map);

    let mut sol_iters = sol.iter().map(|actions| actions.iter()).collect::<Vec<_>>();
    let mut ans = vec![!0; sol.len()];
    let mut n_workers = 0;
    let mut step = 0;
    loop {
        while n_workers < state.locals.len() {
            ans[n_workers] = step;
            n_workers += 1;
        }

        let num_workers = state.locals.len();
        let mut actions: Vec<Option<&Action>> = vec![];
        for i in 0..num_workers {
            actions.push(sol_iters[i].next());
        }
        if actions.iter().all(|a| a.is_none()) {
            break;
        }
        let actions = actions.into_iter().map(|a| *a.unwrap_or(&Action::Nothing)).collect::<Vec<_>>();
        let upd = apply_multi_action(&actions, &mut state, &mut square_map, &mut booster_map);
        eprintln!("{:?}", state);

        step += 1;
    }

    ans
}

fn get_worker_spawn_halt_steps(task: &RasterizedTask, sol: &Vec<Vec<Action>>) -> Vec<(usize, usize)> {
    let ss = get_worker_spawn_steps(task, sol);
    (0..sol.len()).map(|i| (ss[i], ss[i] + sol[i].len())).collect()
}

pub fn insert_fast(task: &RasterizedTask, sol: &Vec<Vec<Action>>) -> Vec<Vec<Action>> {
    let times = get_worker_spawn_halt_steps(task, sol);
    dbg!(&times);
    unimplemented!();
}
