use crate::*;

// startから開始して、targetをぜんぶ回収して、is_goalを満たすマスまで移動する
pub fn tsp<F: Fn(usize, usize) -> bool>(
    map: &Vec<Vec<Square>>,
    start: (usize, usize),
    targets: &Vec<(usize, usize)>,
    is_goal_func: F,
) -> (Vec<Action>, usize, usize) {
    let (xsize, ysize) = get_xysize(map);
    let mut bfs = BFS::new(xsize, ysize);

    let n = targets.len();
    let cost_to_goal: Vec<_> = (0..n)
        .map(|i| {
            bfs.search_fewest_actions_to_satisfy(
                map,
                &WorkerState::new(targets[i].0, targets[i].1),
                |x, y| is_goal_func(x, y),
            )
            .0
            .len()
        })
        .collect();
    let cost_mat: Vec<Vec<_>> = (0..n)
        .map(|u| {
            (0..n)
                .map(|v| {
                    bfs.search_fewest_actions_to_move(
                        map,
                        &WorkerState::new(targets[u].0, targets[u].1),
                        targets[v].0,
                        targets[v].1,
                    )
                    .len()
                })
                .collect()
        })
        .collect();

    if n == 0 {
        return bfs.search_fewest_actions_to_satisfy(
            map,
            &WorkerState::new(start.0, start.1),
            |x, y| is_goal_func(x, y),
        );
    }

    let mut dp = vec![vec![(!0, !0); 1 << n]; n];
    for v in 0..n {
        dp[v][1 << v] = (
            bfs.search_fewest_actions_to_move(
                map,
                &WorkerState::new(start.0, start.1),
                targets[v].0,
                targets[v].1,
            )
            .len(),
            !0,
        );
    }

    for b in 0..1 << n {
        for v in 0..n {
            if (b & (1 << v)) == 0 {
                continue;
            }
            for u in 0..n {
                if (b & (1 << u)) != 0 {
                    continue;
                }
                let t = (dp[v][b].0 + cost_mat[v][u], v);
                dp[u][b | (1 << u)].setmin(t);
            }
        }
    }

    // 頂点順を復元
    let mut b = (1 << n) - 1;
    let mut v = (0..n)
        .max_by_key(|v| dp[*v][b].0 + cost_to_goal[*v])
        .unwrap();
    let mut ord = vec![];

    loop {
        ord.push(v);

        let tv = dp[v][b].1;
        let tb = b ^ (1 << v);
        v = tv;
        b = tb;

        if b == 0 {
            break;
        }
    }
    ord.reverse();
    assert_eq!(ord.len(), n);

    // actionを復元
    let mut actions = vec![];
    actions.extend(bfs.search_fewest_actions_to_move(
        map,
        &WorkerState::new(start.0, start.1),
        targets[ord[0]].0,
        targets[ord[0]].1,
    ));
    for i in 0..(ord.len() - 1) {
        let (u, v) = (ord[i], ord[i + 1]);
        actions.extend(bfs.search_fewest_actions_to_move(
            map,
            &WorkerState::new(targets[u].0, targets[u].1),
            targets[v].0,
            targets[v].1,
        ))
    }
    let v = ord[n - 1];
    let (a, x, y) = bfs.search_fewest_actions_to_satisfy(
        map,
        &WorkerState::new(targets[v].0, targets[v].1),
        |x, y| is_goal_func(x, y),
    );
    actions.extend(a);

    return (actions, x, y);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn it_works() {
        use rand::Rng;
        let mut rng = rand::thread_rng(); // デフォルトの乱数生成器を初期化します

        let tasks = [load_task_001(), load_task_002()];
        for task in tasks.iter() {
            for _ in 0..100 {
                let mut map = task.0.clone();
                let (xsize, ysize) = get_xysize(&map);

                let random_empty_cell = |rng: &mut rand::rngs::ThreadRng| loop {
                    let x: usize = rng.gen::<usize>() % xsize;
                    let y: usize = rng.gen::<usize>() % ysize;
                    if map[x][y] == Square::Empty {
                        return (x, y);
                    }
                };

                let n_targets = rng.gen::<usize>() % 10;
                let start = random_empty_cell(&mut rng);
                let targets = (0..n_targets)
                    .map(|_| random_empty_cell(&mut rng))
                    .collect();

                let (mut actions, goal_x, goal_y) = tsp(&map, start, &targets, |_x, _y| true);

                // 検証
                let mut touched = vec![false; n_targets];
                let mut ps = PlayerState::new(start.0, start.1);
                actions.push(Action::Nothing);

                for action in actions.iter() {
                    for i in 0..n_targets {
                        if (ps.x, ps.y) == targets[i] {
                            touched[i] = true;
                        }
                    }
                    apply_action(*action, &mut ps, &mut map, &mut task.1.clone());
                }

                assert_eq!((ps.x, ps.y), (goal_x, goal_y));
                assert_eq!(touched, vec![true; n_targets]);
            }
        }
    }
}
