use common::*;
use chokudai;

pub fn print_partition(map: &Vec<Vec<Square>>, ps: &Vec<(usize, usize)>) {
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

pub fn k_means(map: &Vec<Vec<Square>>, k: usize) -> Vec<(usize, usize)> {
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

fn compute_dist(map: &Vec<Vec<Square>>, (sx, sy): (usize, usize)) -> Vec<Vec<usize>> {
    let n = map.len();
    let m = map[0].len();
    let mut dist = mat![!0; n; m];
    let mut que = std::collections::VecDeque::new();
    que.push_back((sx, sy));
    dist[sx][sy] = 0;
    while let Some(p) = que.pop_front() {
        for d in 0..4 {
            let (x, y) = apply_move(p, d);
            if dist[x][y] == !0 && map[x][y] != Square::Block {
                dist[x][y] = dist[p.0][p.1] + 1;
                que.push_back((x, y));
            }
        }
    }
    dist
}

pub fn tsp(map: &Vec<Vec<Square>>, ps: &Vec<(usize, usize)>, s: usize) -> Vec<usize> {
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

pub fn at_most_k_step(map: &Vec<Vec<Square>>, target: &Vec<Vec<bool>>, boosters: &Vec<Vec<Option<Booster>>>, state: &PlayerState, k: usize) -> (Vec<Action>, usize) {
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

pub fn optimize(map: &Vec<Vec<Square>>, target: &Vec<Vec<bool>>, boosters: &Vec<Vec<Option<Booster>>>, state: &PlayerState, goal: Option<(usize, usize)>) -> Vec<Action> {
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



pub fn split_solve_sub(map: &Vec<Vec<Square>>, boosters: &Vec<Vec<Option<Booster>>>, (sx, sy): (usize, usize), c: usize, ex: usize, optimize: bool) -> (usize, Vec<Vec<Action>>) {
    let n = map.len();
    let m = map[0].len();
    let mut p_t_as = bootstrap_clone(&(map.clone(), boosters.clone(), sx, sy), c);
    let mut bfs = BFS::new(n, m);
    let mut boosters = boosters.clone();
    let mut get_time = vec![];
    for _ in 0..ex * (c + 1) {
        let mut min_t = !0;
        let mut min_i = !0;
        let mut min_mv = vec![];
        let mut min_to = (!0, !0);
        for i in 0..=c {
            let (x, y) = p_t_as[i].0;
            let (mv, tx, ty) = bfs.search_fewest_actions_to_satisfy(&map, &PlayerState::new(x, y), |i, j| boosters[i][j] == Some(Booster::Extension));
            if min_t.setmin(p_t_as[i].1 + mv.len()) {
                min_i = i;
                min_mv = mv;
                min_to = (tx, ty);
            }
        }
        p_t_as[min_i].0 = min_to;
        p_t_as[min_i].1 = min_t;
        p_t_as[min_i].2.extend(min_mv);
        boosters[min_to.0][min_to.1] = None;
        get_time.push((min_t, min_i));
    }
    get_time.sort();
    let mut manipulators = PlayerState::new(sx, sy).manipulators;
    let mut extended = vec![0; c + 1];
    for e in 0..ex * (c + 1) {
        let mut next = !0;
        let mut min_t = !0;
        for i in 0..=c {
            if extended[i] < ex {
                if min_t.setmin(p_t_as[i].1) {
                    next = i;
                }
            }
        }
        let (t, j) = get_time[e];
        while p_t_as[next].1 < t + if next == j { 0 } else if next > j { 1 } else { 2 } {
            p_t_as[next].1 += 1;
            p_t_as[next].2.push(Action::Nothing);
        }
        p_t_as[next].1 += 1;
        p_t_as[next].2.push(Action::Extension(1, -2 - extended[next] as i32));
        extended[next] += 1;
    }
    for e in 0..ex {
        manipulators.push((1, -2 - e as i32));
    }
    let mut best = vec![];
    let (sx0, sy0) = p_t_as[0].0;
    for op in 0..2 {
        let mut state = chokudai::get_first_state(map.clone(), boosters.clone(), sx0, sy0);
        state.p.manipulators = manipulators.clone();
        let act = chokudai::make_action_by_state(&state, op);
        let act = optimize_pure_move(&(map.clone(), boosters.clone(), sx0, sy0), &act);
        if op == 0 || best.len() > act.len() {
            best = act;
        }
    }
    if optimize {
        let mut state = chokudai::get_first_state(map.clone(), boosters.clone(), sx0, sy0);
        state.p.manipulators = manipulators.clone();
        best = chokudai::optimization_actions(&state, &best, 60).1;
        best = optimize_pure_move(&(map.clone(), boosters.clone(), sx0, sy0), &best);
    }
    let mut max_t = 0;
    let mut best_moves = vec![];
    let mut bfs = BFS::new(n, m);
    for i in 0..=c {
        let from = best.len() * i / (c + 1);
        let to = best.len() * (i + 1) / (c + 1);
        let mut state = PlayerState::new(sx0, sy0);
        let mut map = map.clone();
        let mut boosters = boosters.clone();
        for a in 0..from {
            apply_action(best[a], &mut state, &mut map, &mut boosters);
        }
        let ((sx, sy), mut t, mut pre_mv) = p_t_as[i].clone();
        let mut mv = bfs.search_fewest_actions_to_move(&map, &PlayerState::new(sx, sy), state.x, state.y);
        if state.dir == 1 {
            mv.push(Action::TurnR);
        } else if state.dir == 2 {
            mv.push(Action::TurnR);
            mv.push(Action::TurnR);
        } else if state.dir == 3 {
            mv.push(Action::TurnL);
        }
        mv.extend(best[from..to].into_iter());
        t += mv.len();
        pre_mv.extend(mv);
        max_t.setmax(t);
        best_moves.push(pre_mv);
    }
    let mut dists = vec![];
    for i in 0..=c {
        dists.push(compute_dist(&map, p_t_as[i].0));
    }
    eprintln!("turn: {}", max_t);
    let mut lb = p_t_as.iter().map(|&(_, t, _)| t).max().unwrap();
    let mut ub = max_t;
    let mut ps = vec![(sx0, sy0)];
    let mut dirs = vec![0];
    {
        let mut state = PlayerState::new(sx0, sy0);
        let mut map = map.clone();
        let mut boosters = boosters.clone();
        for &a in &best {
            apply_action(a, &mut state, &mut map, &mut boosters);
            ps.push((state.x, state.y));
            dirs.push(state.dir);
        }
    }
    'a: while ub - lb > 1 {
        let mid = (lb + ub) / 2;
        let mut dp = vec![(0, !0, false); 1 << (c + 1)];
        for used in 0..(1 << (c + 1)) {
            for i in 0..=c {
                if (used >> i & 1) == 0 {
                    let from = dp[used].0;
                    let (tx, ty) = ps[from];
                    let add_d = match dirs[from] {
                        1 | 3 => 1,
                        2 => 2,
                        _ => 0
                    };
                    if p_t_as[i].1 + dists[i][tx][ty] + add_d <= mid {
                        let to = (from + mid - (p_t_as[i].1 + dists[i][tx][ty] + add_d)).min(best.len());
                        dp[used | 1 << i].setmax((to, i, false));
                    }
                    let mut lb2 = 0;
                    let mut ub2 = (mid - p_t_as[i].1 + 1).min(ps.len() - from);
                    while ub2 - lb2 > 1 {
                        let mid2 = (lb2 + ub2) / 2;
                        let (x, y) = ps[from + mid2];
                        let add_d = match dirs[from + mid2] {
                            1 | 3 => 1,
                            2 => 2,
                            _ => 0
                        };
                        if p_t_as[i].1 + dists[i][x][y] + mid2 + add_d <= mid {
                            lb2 = mid2;
                        } else {
                            ub2 = mid2;
                        }
                    }
                    dp[used | 1 << i].setmax((from + lb2, i, true));
                }
            }
        }
        let mut used = (1 << (c + 1)) - 1;
        if dp[used].0 == best.len() {
            let mut moves = vec![vec![]; c + 1];
            while used > 0 {
                let (to, i, rev) = dp[used];
                used ^= 1 << i;
                let mut from = dp[used].0;
                if rev {
                    let ((sx, sy), t, mut pre_mv) = p_t_as[i].clone();
                    let mut mv = bfs.search_fewest_actions_to_move(&map, &PlayerState::new(sx, sy), ps[to].0, ps[to].1);
                    if dirs[to] == 1 {
                        mv.push(Action::TurnR);
                    } else if dirs[to] == 2 {
                        mv.push(Action::TurnR);
                        mv.push(Action::TurnR);
                    } else if dirs[to] == 3 {
                        mv.push(Action::TurnL);
                    }
                    if t + mv.len() > mid {
                        lb = mid;
                        continue 'a;
                    }
                    pre_mv.extend(mv);
                    pre_mv.extend(common::reverse::reverse_actions(&best[from..to]));
                    moves[i] = pre_mv;
                } else {
                    let (tx, ty) = ps[from];
                    let ((sx, sy), t, mut pre_mv) = p_t_as[i].clone();
                    let mut mv = bfs.search_fewest_actions_to_move(&map, &PlayerState::new(sx, sy), tx, ty);
                    if dirs[from] == 1 {
                        mv.push(Action::TurnR);
                    } else if dirs[from] == 2 {
                        mv.push(Action::TurnR);
                        mv.push(Action::TurnR);
                    } else if dirs[from] == 3 {
                        mv.push(Action::TurnL);
                    }
                    if t + mv.len() > mid {
                        lb = mid;
                        continue 'a;
                    }
                    while t + mv.len() < mid && from < best.len() {
                        mv.push(best[from]);
                        from += 1;
                    }
                    pre_mv.extend(mv);
                    moves[i] = pre_mv;
                }
            }
            ub = mid;
            best_moves = moves;
            eprintln!("turn: {}", ub);
        } else {
            lb = mid;
        }
    }
    (ub, best_moves)
}

pub fn split_solve(map: &Vec<Vec<Square>>, boosters: &Vec<Vec<Option<Booster>>>, (sx, sy): (usize, usize), all: i32) -> Vec<Vec<Action>> {
    let n = map.len();
    let m = map[0].len();
    let mut count_x = 0;
    let mut count_clone = 0;
    let mut count_extend = 0;
    let mut pos_extend = vec![];
    for i in 0..n {
        for j in 0..m {
            if boosters[i][j] == Some(Booster::CloneWorker) {
                count_clone += 1;
            } else if boosters[i][j] == Some(Booster::X) {
                count_x += 1;
            } else if boosters[i][j] == Some(Booster::Extension) {
                count_extend += 1;
                pos_extend.push((i, j));
            }
        }
    }
    if count_x == 0 {
        count_clone = 0;
    }
    dbg!((count_x, count_clone, count_extend));
    let mut ret = vec![];
    let mut min_t = !0;
    let mut results = vec![];
    let mut best_c = !0;
    let mut best_ex = !0;
    for c in 0..=count_clone {
        if c != 0 && c != count_clone && all != 2 {
            continue;
        }
        if c != count_clone && all == 0 {
            continue;
        }
        results.push(vec![]);
        for ex in 0..=count_extend / (c + 1) {
            if ex != 0 && ex != count_extend / (c + 1) && all != 2 {
                continue;
            }
            if ex != count_extend / (c + 1) && all == 0 {
                continue;
            }
            let (max_t, moves) = split_solve_sub(map, boosters, (sx, sy), c, ex, false);
            if min_t.setmin(max_t) {
                ret = moves;
                best_c = c;
                best_ex = ex;
            }
            results.last_mut().unwrap().push(max_t);
            eprintln!("{}, {}: {}", c, ex, max_t);
        }
    }
    dbg!(results);
    let (max_t, moves) = split_solve_sub(map, boosters, (sx, sy), best_c, best_ex, true);
    if min_t.setmin(max_t) {
        ret = moves;
    }
    eprintln!("result: {}", min_t);
    ret
}

fn main() {
    let taskfile = std::env::args().nth(1).expect("usage: args[1] = taskfile");
    let all: i32 = std::env::args().nth(2).unwrap_or("0".to_owned()).parse().unwrap();
    let (map, boosters, sx, sy) = read_task(&taskfile);
    let moves = split_solve(&map, &boosters, (sx, sy), all);
    // let moves = clone_solve(&map, &boosters, (sx, sy));
    let moves = solution_to_string(&moves);
    println!("{}", moves);
}
