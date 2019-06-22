use common::*;

fn print_partition(map: &Vec<Vec<Square>>, ps: &Vec<(usize, usize)>) {
    let n = map.len();
    let m = map[0].len();
    let ds = bfs_multi(map, ps);
    let mut cs = mat!['.'; n; m];
    for i in 0..n {
        for j in 0..m {
            if map[i][j] == Square::Empty {
                cs[i][j] = (b'a' + ds[i][j].1 as u8) as char;
            }
        }
    }
    for &(i, j) in ps {
        cs[i][j] = '@';
    }
    for i in 0..n {
        for j in 0..m {
            eprint!("{}", cs[i][j]);
        }
        eprintln!();
    }
    eprintln!();
}

fn k_means(map: &Vec<Vec<Square>>, k: usize) -> Vec<(usize, usize)> {
    let n = map.len();
    let m = map[0].len();
    use rand::seq::SliceRandom;
    let mut opt_ps = vec![];
    let mut opt_score = !0;
    for _ in 0..10 {
        let mut ps = vec![];
        for i in 0..n {
            for j in 0..m {
                if map[i][j] == Square::Empty {
                    ps.push((i, j));
                }
            }
        }
        ps.shuffle(&mut rand::thread_rng());
        ps.truncate(k);
        for _ in 0..100 {
            let ds = bfs_multi(map, &ps);
            let mut fur = vec![(0, 0); ps.len()];
            let mut sum = 0;
            for i in 0..n {
                for j in 0..m {
                    if ds[i][j].0 != !0 {
                        let (d, p, init_dir) = ds[i][j];
                        sum += d * d;
                        if fur[p].0 < d {
                            fur[p] = (d, init_dir);
                        } else if fur[p].0 == d {
                            fur[p].1 &= init_dir;
                        }
                    }
                }
            }
            if opt_score.setmin(sum) {
                opt_ps = ps.clone();
            }
            for i in 0..ps.len() {
                for d in 0..4 {
                    if fur[i].1 >> d & 1 != 0 {
                        ps[i] = apply_move(ps[i], d);
                        break;
                    }
                }
            }
        }
    }
    print_partition(map, &opt_ps);
    opt_ps
}

fn bfs_multi(map: &Vec<Vec<Square>>, ps: &[(usize, usize)]) -> Vec<Vec<(usize, usize, i32)>> {
    let n = map.len();
    let m = map[0].len();
    let mut ds = mat![(!0, !0, 0); n; m];
    let mut que = std::collections::VecDeque::new();
    for i in 0..ps.len() {
        que.push_back(ps[i]);
        ds[ps[i].0][ps[i].1] = (0, i, 0);
    }
    while let Some(p) = que.pop_front() {
        let (d, i, init_dir) = ds[p.0][p.1];
        for dir in 0..4 {
            let q = apply_move(p, dir);
            let init_dir = if d == 0 {
                1 << dir
            } else {
                init_dir
            };
            if ds[q.0][q.1].0 == !0 && map[q.0][q.1] != Square::Block {
                ds[q.0][q.1] = (d + 1, i, init_dir);
                que.push_back(q);
            }
            if ds[q.0][q.1].0 == d + 1 && ds[q.0][q.1].1 == i {
                ds[q.0][q.1].2 |= init_dir;
            }
        }
    }
    ds
}

fn tsp(map: &Vec<Vec<Square>>, ps: &Vec<(usize, usize)>, s: usize) -> Vec<usize> {
    let k = ps.len();
    let mut g = mat![0; k; k];
    for i in 0..k {
        let ds = bfs_multi(map, &[ps[i]]);
        for j in 0..k {
            g[i][j] = ds[ps[j].0][ps[j].1].0;
        }
    }
    let mut dp = mat![(!0, !0); 1 << k; k];
    dp[1 << s][s] = (0, !0);
    for i in 0..1 << k {
        for u in 0..k {
            let d = dp[i][u].0;
            if d != !0 {
                for v in 0..k {
                    if i >> v & 1 == 0 {
                        dp[i | 1 << v][v].setmin((d + g[u][v], u));
                    }
                }
            }
        }
    }
    let mut t = 0;
    for i in 0..k {
        if dp[(1 << k) - 1][t] > dp[(1 << k) - 1][i] {
            t = i;
        }
    }
    let mut us = vec![];
    let mut i = (1 << k) - 1;
    while t != s {
        us.push(t);
        let x = t;
        t = dp[i][t].1;
        i ^= 1 << x;
    }
    us.push(s);
    us.reverse();
    us
}

fn at_most_k_step(map: &Vec<Vec<Square>>, target: &Vec<Vec<bool>>, boosters: &Vec<Vec<Option<Booster>>>, state: &PlayerState, k: usize) -> (Vec<Action>, usize) {
    if k == 0 {
        (vec![], 0)
    } else {
        let mut opt = (vec![], 0);
        for &mv in &[Action::Move(0), Action::Move(1), Action::Move(2), Action::Move(3), Action::TurnL, Action::TurnR] {
            let mut map = map.clone();
            let mut boosters = boosters.clone();
            let mut state = state.clone();
            let mut count = 0;
            if let Action::Move(d) = mv {
                let (cx, cy) = apply_move((state.x, state.y), d);
                if map[cx][cy] == Square::Block {
                    continue;
                }
            }
            for (x, y) in apply_action(mv, &mut state, &mut map, &mut boosters).filled {
                if target[x][y] {
                    count += 1;
                }
            }
            let (act, c) = at_most_k_step(&map, target, &boosters, &state, k - 1);
            if (opt.1, !opt.0.len()) < (count + c, !(act.len() + 1)) {
                let mut a = vec![mv];
                a.extend(act);
                opt = (a, count + c);
            }
        }
        opt
    }
}

fn optimize(map: &Vec<Vec<Square>>, target: &Vec<Vec<bool>>, boosters: &Vec<Vec<Option<Booster>>>, state: &PlayerState, goal: Option<(usize, usize)>) -> Vec<Action> {
    assert!(goal.is_none());
    let n = map.len();
    let m = map[0].len();
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (n, 0, m, 0);
    for i in 0..n {
        for j in 0..m {
            if target[i][j] {
                min_x.setmin(i);
                max_x.setmin(i + 1);
                min_y.setmin(j);
                max_y.setmin(j + 1);
            }
        }
    }
    // if max_x - min_x < n - 2 || max_y - min_y < m - 2 {
    //     let map2 = map[min_x..max_x]
    // }
    let mut actions = vec![];
    let mut map = map.clone();
    let mut boosters = boosters.clone();
    let mut state = state.clone();
    let mut num_empty = 0;
    for i in 0..n {
        for j in 0..m {
            if target[i][j] {
                num_empty += 1;
            }
        }
    }
    let mut bfs = BFS::new(n, m);
    while num_empty > 0 {
        let (mut moves, _) = at_most_k_step(&map, &target, &boosters, &state, 5);
        if moves.len() == 0 {
            moves = bfs.search_fewest_actions_to_satisfy(&map, &state, |x, y| {
                if target[x][y] && map[x][y] == Square::Empty {
                    return true;
                }
                false
            }).0;
        }
        for mv in moves {
            actions.push(mv);
            let update = apply_action(mv, &mut state, &mut map, &mut boosters);
            let mut br = false;
            for (x, y) in update.filled {
                if target[x][y] {
                    num_empty -= 1;
                    br = true;
                }
            }
            if br {
                break;
            }
        }
    }
    actions
}

fn solve(map: &Vec<Vec<Square>>, boosters: &Vec<Vec<Option<Booster>>>, (sx, sy): (usize, usize)) -> Vec<Action> {
    let mut map = map.clone();
    let mut boosters = boosters.clone();
    let n = map.len();
    let m = map[0].len();
    let ps = k_means(&map, 1);
    let ids = bfs_multi(&map, &ps).into_iter().map(|d| d.into_iter().map(|(_, a, _)| a).collect()).collect::<Vec<Vec<_>>>();
    let tsp = tsp(&map, &ps, ids[sx][sy]);
    let mut state = PlayerState::new2(sx, sy, &mut map);
    let mut actions = vec![];
    for t in tsp {
        let mut target = mat![false; n; m];
        for i in 0..n {
            for j in 0..m {
                if map[i][j] == Square::Empty && ids[i][j] == t {
                    target[i][j] = true;
                }
            }
        }
        let act = optimize(&map, &target, &boosters, &state, None);
        for a in act {
            actions.push(a);
            apply_action(a, &mut state, &mut map, &mut boosters);
        }
    }
    actions
}

fn clone_solve(map: &Vec<Vec<Square>>, boosters: &Vec<Vec<Option<Booster>>>, (sx, sy): (usize, usize)) -> Vec<Vec<Action>> {
    let n = map.len();
    let m = map[0].len();
    let mut count_x = 0;
    let mut count_clone = 0;
    for i in 0..n {
        for j in 0..m {
            if boosters[i][j] == Some(Booster::CloneWorker) {
                count_clone += 1;
            } else if boosters[i][j] == Some(Booster::X) {
                count_x += 1;
            }
        }
    }
    if count_x == 0 {
        count_clone = 0;
    }
    dbg!((count_x, count_clone));
    let mut ret = vec![];
    for c in 0..=count_clone {
        let pas = bootstrap_clone(&(map.clone(), boosters.clone(), sx, sy), count_clone);
        let mut acts = vec![];
        for (_, _, act) in pas {
            acts.push(act);
        }
    }
    ret
}

fn main() {
    let taskfile = std::env::args().nth(1).expect("usage: args[1] = taskfile");
    let (map, boosters, sx, sy) = read_task(&taskfile);
    // let moves = vec![solve(&map, &boosters, (sx, sy))];
    let moves = clone_solve(&map, &boosters, (sx, sy));
    let moves = solution_to_string(&moves);
    eprintln!("turns: {}", moves.len());
    println!("{}", moves);
}
