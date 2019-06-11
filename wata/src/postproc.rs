#![allow(unused)]
use *;
use std::collections::*;
use std::iter::FromIterator;


pub fn fusion_all(matrix: &V3<bool>, mut positions: Vec<P>) -> Vec<Command> {
    {
        let position_set = BTreeSet::from_iter(positions.iter());
        assert_eq!(position_set.len(), positions.len());
    }
    let x_set = BTreeSet::from_iter(positions.iter().map(|p: &P| p.x));
    let y_set = BTreeSet::from_iter(positions.iter().map(|p: &P| p.y));
    let z_set = BTreeSet::from_iter(positions.iter().map(|p: &P| p.z));
    if positions.len() == 8 && [&x_set, &y_set, &z_set].iter().all(|s| is_good_coord_set(s)) {
        let x_max = *x_set.iter().max().unwrap();
        let y_max = *y_set.iter().max().unwrap();
        let z_max = *z_set.iter().max().unwrap();

        let mut orz = false;
        for x in 0..=x_max {
            for y in 0..=y_max {
                for z in 0..=z_max {
                    if ((x == 0 || x == x_max) as i32
                        + (y == 0 || y == y_max) as i32
                        + (z == 0 || z == z_max) as i32
                        >= 2 // edge
                    ) && matrix[P::new(x, y, z)] {
                        orz = true;
                    }
                }
            }
        }

        if !orz {
            eprintln!("hand-crafted (^_^)");

            let x_mid = (x_max + 1) / 2;
            let y_mid = (y_max + 1) / 2;
            let z_mid = (z_max + 1) / 2;

            use Command::*;
            let cmds = vec![
                vec![ // 0 0 0
                    Wait, Wait, FusionP(P::new(1, 0, 0)),
                    FusionP(P::new(0, 1, 0)), FusionP(P::new(1, 0, 0)),
                    FusionP(P::new(0, 0, 1)), FusionP(P::new(0, 1, 0)), Halt],
                vec![ // 0 0 z
                    SMove(P::new(0, 0, z_mid-z_max)), SMove(P::new(0, 0, 1-z_mid)),
                    FusionS(P::new(0, 1, -1))],
                vec![ // 0 y 0
                    SMove(P::new(0, y_mid-y_max, 0)), SMove(P::new(0, 1-y_mid, 0)),
                    FusionP(P::new(0, -1, 1)), FusionS(P::new(0, -1, 0))],
                vec![ // 0 y z
                    SMove(P::new(0, y_mid-y_max, 0)), SMove(P::new(0, -y_mid, 0)),
                    FusionP(P::new(1, 0, 0)),
                    SMove(P::new(0, 0, z_mid-z_max)), SMove(P::new(0, 0, 1-z_mid)),
                    FusionS(P::new(0, 0, -1))],
                vec![ // x 0 0
                    SMove(P::new(x_mid-x_max, 0, 0)), SMove(P::new(1-x_mid, 0, 0)),
                    FusionS(P::new(-1, 0, 0))],
                vec![ // x 0 z
                    SMove(P::new(x_mid-x_max, 0, 0)), SMove(P::new(1-x_mid, 0, 0)),
                    FusionS(P::new(-1, 0, 0))],
                vec![ // x y 0
                    SMove(P::new(0, y_mid-y_max, 0)), SMove(P::new(0, -y_mid, 0)),
                    SMove(P::new(x_mid-x_max, 0, 0)), SMove(P::new(1-x_mid, 0, 0)),
                    FusionS(P::new(-1, 0, 0))],
                vec![ // x y z
                    SMove(P::new(x_mid-x_max, 0, 0)), SMove(P::new(-x_mid, 0, 0)),
                    SMove(P::new(0, 0, z_mid-z_max)), SMove(P::new(0, 0, -z_mid)),
                    SMove(P::new(0, y_mid-y_max, 0)), SMove(P::new(0, 1-y_mid, 0)),
                    FusionS(P::new(0, -1, 0))],
            ];

            let mut ret = Vec::new();
            for t in 0..8 {
                for pos in positions.iter() {
                    let mut i = 0;
                    if pos.x != 0 {
                        i += 4;
                    }
                    if pos.y != 0 {
                        i += 2;
                    }
                    if pos.z != 0 {
                        i += 1;
                    }
                    if t < cmds[i].len() {
                        ret.push(cmds[i][t]);
                    }
                }
            }
            return ret;
        }
    }
    fusion_all_ver2(matrix, positions)
}

fn is_good_coord_set(set: &BTreeSet<i32>) -> bool {
    set.len() == 2
        && *set.iter().nth(0).unwrap() == 0
        && *set.iter().nth(1).unwrap() <= 30
        && *set.iter().nth(1).unwrap() >= 3
}


pub fn fusion_all_ver2(matrix: &V3<bool>, mut positions: Vec<P>) -> Vec<Command> {
    let mut return_cmds = Vec::new();
    let r = matrix.len();
    let mut bfs = bfs::BFS::new(r);
    let filled_func = |p: P| { matrix[p] };
    /*
    let goal_func = |p: P| { true };
    bfs.bfs(filled_func, &vec![P::new(0, 0, 0)], goal_func);
    */
    {
        let mut unreached_position_set = BTreeSet::from_iter(positions.iter());
        {
            let goal_func = |p: P| {
                if unreached_position_set.remove(&p) {
                    eprintln!("Fusion BFS: {} / {} remaining", unreached_position_set.len(), positions.len());
                }
                return unreached_position_set.len() == 0;
            };
            bfs.bfs(filled_func, &vec![P::new(0, 0, 0)], goal_func);
        }
        assert_eq!(unreached_position_set.len(), 0);  // Otherwise, some positions were unreachable
        // eprintln!("Fusion BFS: done");
    }

    let mut occupied = InitV3::new(false, r);
    loop {
        occupied.init();
        for &pos in positions.iter() {
            occupied[pos] = true;
        }

        let n = positions.len();
        let mut step_cmds = vec![Command::Wait; n];

        let mut remove_bids = Vec::new();
        {  // fusion
            let mut waiting_pos = BTreeSet::from_iter(positions.iter().cloned());
            while let Some((p1, p2)) = pop_near_pair(&mut waiting_pos) {
                waiting_pos.remove(&p1);
                waiting_pos.remove(&p2);
                // these bid* are not true but positions are sorted by true bid
                let bid1 = positions.iter().position(|&p| p == p1).unwrap();
                let bid2 = positions.iter().position(|&p| p == p2).unwrap();
                step_cmds[bid1] = Command::FusionP(p2 - p1);
                step_cmds[bid2] = Command::FusionS(p1 - p2);
                remove_bids.push(bid2);
            }
        }

        // eprintln!("{:?}", positions);
        for (i, pos) in positions.iter_mut().enumerate() {
            if step_cmds[i] != Command::Wait {
                continue;
            }
            let cmd;
            {
                let mut pos_cands = BTreeMap::new();
                for (new_pos, cmd) in one_step(*pos, r, |p: P| matrix[p] || occupied[p]) {
                    pos_cands.insert(new_pos, cmd);
                }
                // let goal_func = |p: P| { pos_cands.contains_key(&p) };
                let goal_set: BTreeSet<P> = pos_cands.keys().cloned().collect();
                let new_pos = bfs.bfs_continue(filled_func, &goal_set).unwrap();
                cmd = pos_cands[&new_pos];
                // eprintln!("{:?} {:?}", pos, cmd);
                set_occupied(*pos, cmd, &mut occupied);
                *pos = new_pos;
            }
            step_cmds[i] = cmd;
        }

        if step_cmds.iter().all(|&cmd| cmd == Command::Wait) {
            break;
        }

        return_cmds.append(&mut step_cmds);

        remove_bids.sort();
        for bid in remove_bids.into_iter().rev() {
            positions.remove(bid);
        }
    }
    return_cmds.push(Command::Halt);

    return_cmds
}


fn one_step<F: Fn(P) -> bool>(p: P, r: usize, is_bad: F) -> Vec<(P, Command)> {
    // always push Wait
    // don't check p (occupied by self)
    let mut ret = vec![(p, Command::Wait)];
    for &v1 in ADJ.iter() {
        let mut p1 = p;
        for d1 in 1..=15 {
            p1 += v1;
            if !p1.is_valid(r) || is_bad(p1) {
                break;
            }
            ret.push((p1, Command::SMove(v1 * d1)));
            if d1 <= 5 {
                for &v2 in ADJ.iter() {
                    if v2 == v1 || v2 == -v1 {
                        continue;
                    }
                    let mut p2 = p1;
                    for d2 in 1..=5 {
                        p2 += v2;
                        if !p2.is_valid(r) || is_bad(p2) {
                            break;
                        }
                        ret.push((p2, Command::LMove(v1 * d1, v2 * d2)));
                    }
                }
            }
        }
    }
    ret
}


pub fn fusion_all_ver1(matrix: &V3<bool>, positions: Vec<P>) -> Vec<Command> {
    let mut return_cmds = Vec::new();
    let r = matrix.len();
    eprintln!("{:?}", r);
    for x in 0..r {
        for y in 0..r {
            for z in 0..r {
                if x == 0 || x + 1 == r || y + 1 == r || z == 0 || z + 1 == r {
                    assert!(!matrix[x][y][z]);
                }
            }
        }
    }
    let mut cmdss: Vec<VecDeque<Command>> = Vec::new();
    let mut bfs = bfs::BFS::new(r);
    {
        let filled_func = |p: P| { matrix[p] };

        let mut unreached_position_set = BTreeSet::from_iter(positions.iter());
        {
            let goal_func = |p: P| {
                if unreached_position_set.remove(&p) {
                    eprintln!("Fusion BFS: {} / {} remaining", unreached_position_set.len(), positions.len());
                }
                return unreached_position_set.len() == 0;
            };
            bfs.bfs(filled_func, &vec![P::new(0, 0, 0)], goal_func);
        }
        assert_eq!(unreached_position_set.len(), 0);  // Otherwise, some positions were unreachable

        for &pos in positions.iter() {
            let cmds = bfs.restore_backward(pos);
            cmdss.push(cmds.into_iter().collect());
        }

        bfs.clear();
    }

    let mut positions = positions;

    let mut occupied = InitV3::new(false, r);
    // while positions.len() > 1 && positions[0].mlen() != 0 {
    loop {
        occupied.init();

        let mut step_cmds = Vec::new();

        for &pos in positions.iter() {
            occupied[pos] = true;
        }

        // eprintln!("{:?}", positions);
        for (pos, mut cmds) in positions.iter_mut().zip(cmdss.iter_mut()) {
            let cmd = cmds.pop_front().unwrap_or(Command::Wait);
            let mut orz = false;
            for (p, cmd_done, cmd_remain) in path(*pos, cmd) {
                if occupied[p] {
                    cmds.push_front(cmd_remain);
                    step_cmds.push(cmd_done);
                    orz = true;
                    break;
                }
                occupied[p] = true;
                *pos = p;
            }
            if !orz {
                step_cmds.push(cmd);
            }
        }

        let mut remove_bids = Vec::new();

        let mut waiting_pos = BTreeSet::new();
        for (i, &pos) in positions.iter().enumerate() {
            if step_cmds[i] == Command::Wait {
                waiting_pos.insert(pos);
            }
        }
        while let Some((p1, p2)) = pop_near_pair(&mut waiting_pos) {
            waiting_pos.remove(&p1);
            waiting_pos.remove(&p2);
            // these bid* are not true but positions are sorted by true bid
            let bid1 = positions.iter().position(|&p| p == p1).unwrap();
            let bid2 = positions.iter().position(|&p| p == p2).unwrap();
            step_cmds[bid1] = Command::FusionP(p2 - p1);
            step_cmds[bid2] = Command::FusionS(p1 - p2);
            remove_bids.push(bid2);
        }

        if step_cmds.iter().all(|&cmd| cmd == Command::Wait) {
            break;
        }
        return_cmds.append(&mut step_cmds);

        remove_bids.sort();
        for bid in remove_bids.into_iter().rev() {
            positions.remove(bid);
            cmdss.remove(bid);
        }
    }
    return_cmds.push(Command::Halt);

    return_cmds
}


fn pop_near_pair(mut poss: &mut BTreeSet<P>) -> Option<(P, P)> {
    match get_near_pair(&poss) {
        Some((p1, p2)) => {
            poss.remove(&p1);
            poss.remove(&p2);
            if p1.mlen() <= p2.mlen() {
                Some((p1, p2))
            } else {
                Some((p2, p1))
            }
        },
        None => None,
    }
}


fn get_near_pair(poss: &BTreeSet<P>) -> Option<(P, P)> {
    for &p1 in poss.iter() {
        for p2 in p1.near(9999) {
            if poss.contains(&p2) {
                return Some((p1, p2));
            }
        }
    }
    return None;
}


fn path(mut p: P, mut cmd: Command) -> Vec<(P, Command, Command)> {
    // (next_pos, cmd_done, cmd_remain)
    let mut ret = Vec::new();
    let mut cmd_done = Command::Wait;
    while let Command::LMove(d1, d2) = cmd {
        let v = d1 / d1.mlen();
        p += v;
        ret.push((p, cmd_done, cmd));
        let d1 = d1 - v;
        cmd = if d1.mlen() == 0 {
            Command::SMove(d2)
        } else {
            Command::LMove(d1, d2)
        };
        cmd_done = cmd_push(cmd_done, v);
    }
    while let Command::SMove(d1) = cmd {
        let v = d1 / d1.mlen();
        p += v;
        ret.push((p, cmd_done, cmd));
        let d1 = d1 - v;
        cmd = if d1.mlen() == 0 {
            Command::Wait
        } else {
            Command::SMove(d1)
        };
        cmd_done = cmd_push(cmd_done, v);
    }
    ret
}


fn cmd_push(cmd: Command, d: P) -> Command {
    match cmd {
        Command::Wait => Command::SMove(d),
        Command::SMove(d1) => if d == d1/d1.mlen() {
            Command::SMove(d1 + d)
        } else {
            Command::LMove(d1, d)
        },
        Command::LMove(d1, d2) => Command::LMove(d1, d2 + d),
        _ => panic!()
    }
}
