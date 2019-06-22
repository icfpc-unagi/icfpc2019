use crate::*;

// 更新されたタスク、ここまでのアクション列、プレイヤーの状態
pub type BootstrapResult = (RasterizedTask, Vec<Action>, PlayerState);

pub fn bootstrap_expand<F: Fn(PlayerState) -> Option<Action>>(
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

    let (actions, x, y) = tsp(square_map, start, &targets, |_, _| true);

    let mut square_map = square_map.clone();
    let mut booster_map = booster_map.clone();
    let mut player_state = PlayerState::new(*start_x, *start_y);
    for action in actions.iter() {
        apply_action(
            *action,
            &mut player_state,
            &mut square_map,
            &mut booster_map,
        );

        // if player_state.unused_boosters.
    }

    (
        (square_map, booster_map, player_state.x, player_state.y),
        actions,
        player_state,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let task = load_example_01();
        let (task, actions, state) = bootstrap_expand(
            &task,
            |p| None,
            100,
        );
        dbg!(&actions);
        dbg!(&state);
    }
}
