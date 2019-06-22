use crate::*;

// 更新されたタスク、ここまでのアクション列、プレイヤーの状態
pub type BootstrapResult = (RasterizedTask, Vec<Action>, PlayerState);

impl PlayerState {
    fn has_expand(&self) -> bool {
        self.unused_boosters.iter().any(|&b| b == Booster::Extension)
    }
}

pub fn bootstrap_expand<F: Fn(&PlayerState) -> Option<Action>>(
    task: &RasterizedTask,
    expand_callback: F,
    max_expands: usize,
) -> BootstrapResult {
    // TODO: max_expandsをちゃんと使う！！！！！！！

    let (square_map, booster_map, start_x, start_y) = task;
    let start = (*start_x, *start_y);
    let (xsize, ysize) = get_xysize(square_map);
    // unimplemented!();

    let mut targets = vec![];
    for x in 0..xsize {
        for y in 0..ysize {
            if booster_map[x][y] == Some(Booster::Extension) {
                targets.push((x, y));
            }
        }
    }

    if targets.len() > 20 {
        eprintln!("awoeifjpaowejfpoiawejfpoiawejfpoawiejfopawejfpawoeifjapwoefjipae")
        eprintln!("awoeifjpaowejfpoiawejfpoiawejfpoawiejfopawejfpawoeifjapwoefjipae")
        eprintln!("TOO MANY EXPANDS: {}", targets.len());
        eprintln!("awoeifjpaowejfpoiawejfpoiawejfpoawiejfopawejfpawoeifjapwoefjipae")
        eprintln!("awoeifjpaowejfpoiawejfpoiawejfpoawiejfopawejfpawoeifjapwoefjipae")
    }

    let (actions, x, y) = tsp(square_map, start, &targets, |_, _| true);

    let mut square_map = square_map.clone();
    let mut booster_map = booster_map.clone();
    let mut player_state = PlayerState::new(*start_x, *start_y);

    let mut actions2: Vec<Action> = vec![];
    for move_action in actions.iter() {
        apply_action(
            *move_action,
            &mut player_state,
            &mut square_map,
            &mut booster_map,
        );
        //assert!(move_action == Action::Move());

        actions2.push(*move_action);

        if player_state.has_expand() {
            if let Some(expand_action) = expand_callback(&player_state) {
                apply_action(
                    expand_action,
                    &mut player_state,
                    &mut square_map,
                    &mut booster_map,
                );
                actions2.push(expand_action);
            }
        }
    }

    let (x, y) = (player_state.x, player_state.y);
    (
        (square_map, booster_map, x, y),
        actions,
        player_state.clone(),
    )
}

pub fn bootstrap_expand_0_atsumerudake(
    task: &RasterizedTask,
    max_expands: usize,
) -> BootstrapResult {
    let f = |p: &PlayerState| {
        None
    };

    bootstrap_expand(
        task,
        f,
        max_expands)
}

pub fn bootstrap_expand_1_migimae(
    task: &RasterizedTask,
    max_expands: usize,
) -> BootstrapResult {
    let f = |p: &PlayerState| {
        Some(Action::Extension(1, -((p.manipulators.len() - 2) as i32)))
    };

    bootstrap_expand(
        task,
        f,
        max_expands)
}

pub fn bootstrap_expand_2_migi(
    task: &RasterizedTask,
    max_expands: usize,
) -> BootstrapResult {
    let f = |p: &PlayerState| {
        Some(Action::Extension(0, -((p.manipulators.len() - 3) as i32)))
    };

    bootstrap_expand(
        task,
        f,
        max_expands)
}

pub fn bootstrap_expand_3_mae(
    task: &RasterizedTask,
    max_expands: usize,
) -> BootstrapResult {
    let f = |p: &PlayerState| {
        Some(Action::Extension((p.manipulators.len() - 2) as i32, 0))
    };

    bootstrap_expand(
        task,
        f,
        max_expands)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        {
            let task = load_example_01();
            let (task, actions, state) = bootstrap_expand_1_migimae(
                &task,
                100,
            );
            dbg!(&actions);
            dbg!(&state);
        }

        {
            let task = load_example_01();
            let (task, actions, state) = bootstrap_expand_2_migi(
                &task,
                100,
            );
            dbg!(&actions);
            dbg!(&state);
        }

        {
            let task = load_example_01();
            let (task, actions, state) = bootstrap_expand_3_mae(
                &task,
                100,
            );
            dbg!(&actions);
            dbg!(&state);
        }

        {
            let task = load_example_01();
            let (task, actions, state) = bootstrap_expand_0_atsumerudake(
                &task,
                100,
            );
            dbg!(&actions);
            dbg!(&state);
        }
    }
}
