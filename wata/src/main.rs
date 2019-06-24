use common::*;
use chokudai;

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

pub fn split_solve_sub(map: &Vec<Vec<Square>>, boosters: &Vec<Vec<Option<Booster>>>, (sx, sy): (usize, usize),
                        c: usize, ex: usize, buy_c: usize, buy_ex: usize, optimize: bool, many_starts: bool, ops: &[chokudai::ChokudaiOptions])
                         -> (usize, Vec<Vec<Action>>, chokudai::ChokudaiOptions) {
    let n = map.len();
    let m = map[0].len();
    let mut p_t_as = bootstrap_clone(&(map.clone(), boosters.clone(), sx, sy), c, buy_c);
    
    let mut bfs = BFS::new(n, m);
    let mut boosters = boosters.clone();
    let mut get_time = vec![];
    for _ in 0..buy_ex {
        get_time.push((0, 0));
    }
    for _ in buy_ex..ex * (c + 1) {
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
    for e in 0..ex {
        manipulators.push((1, -2 - e as i32));
    }
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
    
    let mut pre_map = map.clone();
    let mut pre_boosters = boosters.clone();
    let pre_actions: Vec<_> = p_t_as.iter().map(|(_, _, acts)| acts.clone()).collect();
    let buy_boosters: Vec<_> = (0..100).map(|_| Booster::CloneWorker).chain((0..100).map(|_| Booster::Extension)).collect();
    // let buy_boosters: Vec<_> = (0..buy_c).map(|_| Booster::CloneWorker).chain((0..buy_ex).map(|_| Booster::Extension)).collect();
    let mut pre_state = WorkersState::new_t0_with_options(sx, sy, &mut pre_map, buy_boosters);
    sim2::apply_multi_actions(&mut pre_map, &mut pre_boosters, &mut pre_state, &pre_actions);
    
    let mut starts = vec![];
    if many_starts {
        for s in 0..=c {
            starts.push(p_t_as[s].0);
        }
        let mut dists = vec![];
        for i in 0..=c {
            dists.push(compute_dist(&map, p_t_as[i].0));
        }
        let mut nearest = vec![(!0, (!0, !0)); 4];
        for i in 0..n {
            for j in 0..m {
                if map[i][j] != Square::Block {
                    let mut min_dist = !0;
                    for a in 0..=c {
                        min_dist.setmin(dists[a][i][j]);
                    }
                    let mut num = 0;
                    for d in 0..4 {
                        let (x, y) = apply_move((i, j), d);
                        if map[x][y] == Square::Block {
                            num += 1;
                        }
                    }
                    if num > 0 && num < 4 {
                        nearest[num].setmin((min_dist, (i, j)));
                    }
                }
            }
        }
        for d in 1..4 {
            if nearest[d].0 != !0 {
                starts.push(nearest[d].1);
            }
        }
    } else {
        starts.push(p_t_as[0].0);
    }
    starts.sort();
    starts.dedup();
    dbg!(&starts);
    
    let mut best = vec![];
    let mut best_op = chokudai::ChokudaiOptions::default();
    let mut best_start = (!0, !0);
    for op in ops {
        for &(sx0, sy0) in &starts {
            let mut state = chokudai::get_first_state(map.clone(), boosters.clone(), sx0, sy0);
            state.p.manipulators = manipulators.clone();
            let act = chokudai::make_action_by_state(&state, &op);
            let mut state = PlayerState::new(sx0, sy0);
            state.manipulators = manipulators.clone();
            let act = optimize_pure_move(&pre_map.clone(), &boosters.clone(), &state, &act);
            let mut state = chokudai::get_first_state(map.clone(), boosters.clone(), sx0, sy0);
            state.p.manipulators = manipulators.clone();
            let act = chokudai::optimization_actions(&state, &act, 6, &op).1;
            // let act = chokudai::optimization_actions(&state, &act, 6, &chokudai::ChokudaiOptions::chokudai()).1;
            if best.len() == 0 || best.len() > act.len() {
                best = act;
                best_op = op.clone();
                best_start = (sx0, sy0);
            }
        }
    }
    dbg!(best_start);
    let (sx0, sy0) = best_start;
    if optimize {
        let mut state = PlayerState::new(sx0, sy0);
        state.manipulators = manipulators.clone();
        best = optimize_pure_move(&pre_map.clone(), &boosters.clone(), &state, &best);
        let mut state = chokudai::get_first_state(map.clone(), boosters.clone(), sx0, sy0);
        state.p.manipulators = manipulators.clone();
        best = chokudai::optimization_actions(&state, &best, 120, &best_op).1;
        // best = chokudai::optimization_actions(&state, &best, 120, &chokudai::ChokudaiOptions::chokudai()).1;
        // best = chokudai::optimization_actions(&state, &best, 60, &best_op).1;
        
        for w in 6..=9 {
            dbg!(w);
            let mut state = PlayerState::new(sx0, sy0);
            state.manipulators = manipulators.clone();
            best = optimize_pure_move2(&pre_map.clone(), &boosters.clone(), &state, &best, w, 30);
        }
        
        let mut state = PlayerState::new(sx0, sy0);
        state.manipulators = manipulators.clone();
        best = optimize_pure_move(&pre_map.clone(), &boosters.clone(), &state, &best);
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
    (ub, best_moves, best_op)
}

pub fn split_solve(map: &Vec<Vec<Square>>, boosters: &Vec<Vec<Option<Booster>>>, (sx, sy): (usize, usize), buy: &str, all: i32) -> Vec<Vec<Action>> {
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
    let mut buy_clone = 0;
    let mut buy_extend = 0;
    for c in buy.chars() {
        if c == 'C' {
            buy_clone += 1;
        } else if c == 'B' {
            buy_extend += 1;
        }
    }
    if count_x == 0 && count_clone + buy_clone > 0 {
        eprintln!("no X");
        count_clone = 0;
        buy_clone = 0;
    }
    dbg!((count_x, count_clone, count_extend));
    let mut ret = vec![];
    let mut min_t = !0;
    let mut results = vec![];
    let mut best_c = !0;
    let mut best_ex = !0;
    let mut best_op = chokudai::ChokudaiOptions::default();
    for c in 0..=count_clone {
        if c != count_clone && all == 0 {
            continue;
        }
        if c != 0 && c != count_clone && all == 1 {
            continue;
        }
        results.push(vec![]);
        let ex_max = (count_extend + buy_extend) / (c + buy_clone + 1);
        for ex in 0..=ex_max {
            if ex != ex_max && all == 0 {
                continue;
            }
            let (max_t, moves, op) = split_solve_sub(map, boosters, (sx, sy), c + buy_clone, ex, buy_clone, buy_extend, false, false, &chokudai::ChokudaiOptions::small());
            if min_t.setmin(max_t) {
                ret = moves;
                best_c = c;
                best_ex = ex;
                best_op = op;
            }
            results.last_mut().unwrap().push(max_t);
            eprintln!("c = {}, ex = {}: {}", c, ex, max_t);
        }
    }
    dbg!(results);
    dbg!(&best_op);
    best_op = chokudai::ChokudaiOptions::small()[3].clone();//
    let (max_t, moves, _) = split_solve_sub(map, boosters, (sx, sy), best_c + buy_clone, best_ex, buy_clone, buy_extend, true, true, &[best_op]);
    if min_t.setmin(max_t) {
        ret = moves;
    }
    eprintln!("result: {}", min_t);
    ret
}

fn flip<T: Copy>(a: &mut Vec<Vec<T>>) {
    let n = a.len();
    let m = a[0].len();
    let b = a.clone();
    for i in 0..n {
        for j in 0..m {
            a[i][j] = b[i][m - j - 1];
        }
    }
}

fn flip_task(map: &mut SquareMap, boosters: &mut BoosterMap, sy: &mut usize) {
    let m = map[0].len();
    flip(map);
    flip(boosters);
    *sy = m - *sy - 1;
}

fn main() {
    let taskfile = std::env::args().nth(1).expect("usage: args[1] = taskfile");
    let buy = std::env::args().nth(2).expect("usage: args[2] = buy");
    let all: i32 = std::env::args().nth(3).unwrap_or("0".to_owned()).parse().unwrap();
    let (mut map, mut boosters, sx, mut sy) = read_task(&taskfile);
    flip_task(&mut map, &mut boosters, &mut sy);
    let mut moves = split_solve(&map, &boosters, (sx, sy), &buy, all);
    flip_solution(&mut moves);
    let moves = solution_to_string(&moves);
    println!("{}", moves);
}
