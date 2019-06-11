#![allow(unused)]

use super::super::*;
use super::super::bfs::*;
use std::collections::*;

#[derive(Clone, Debug)]
pub struct Bot {
    pub bid: usize,
    pub p: P,
    pub commands: Vec<Command>,
}

pub fn target_bottom_up(target: &V3<bool>) -> V3<bool> {
    let r = target.len();
    let mut target2 = target.clone();
    let mut ground = mat![false; r; r; r];
    for x in 0..r {
        for z in 0..r {
            if target2[x][0][z] {
                ground[x][0][z] = true;
            }
        }
    }
    for y in 1..r {
        let mut stack = vec![];
        for x in 0..r {
            for z in 0..r {
                if target2[x][y][z] && ground[x][y - 1][z] {
                    ground[x][y][z] = true;
                    stack.push(P::new(x as i32, y as i32, z as i32));
                }
            }
        }
        loop {
            while let Some(p) = stack.pop() {
                for q in p.adj(r) {
                    if q.y == y as i32 && target2[q] && !ground[q] {
                        ground[q] = true;
                        stack.push(q);
                    }
                }
            }
            let mut max_d = -2;
            let mut q = P::new(0, 0, 0);
            for x in 0..r {
                for z in 0..r {
                    if target2[x][y][z] && !ground[x][y][z] {
                        let mut d = -1;
                        for y2 in (0..y).rev() {
                            if target2[x][y2][z] {
                                d = y2 as i32;
                                break;
                            }
                        }
                        if max_d.setmax(d) {
                            q = P::new(x as i32, y as i32, z as i32);
                        }
                    }
                }
            }
            if max_d == -2 {
                break;
            }
            eprintln!("support: {:?} : {}", q, y as i32 - max_d);
            ground[q] = true;
            stack.push(q);
            for y2 in (0..y).rev() {
                if target2[q.x as usize][y2][q.z as usize] {
                    break;
                }
                target2[q.x as usize][y2][q.z as usize] = true;
            }
        }
    }
    target2
}

pub fn destruct_support(target: &V3<bool>, filled: &mut V3<bool>, bots: &mut Vec<Bot>) -> i64 {
    let r = target.len();
    let mut supports = vec![];
    for x in 0..r {
        for z in 0..r {
            let mut y = 0;
            while y < r {
                if !target[x][y][z] && filled[x][y][z] {
                    let mut s = y;
                    while !target[x][y][z] {
                        y += 1;
                    }
                    let t = y - 1;
                    while t - s > 30 {
                        supports.push([P::new(x as i32, s as i32, z as i32), P::new(x as i32, (s + 30) as i32, z as i32)]);
                        s += 31;
                    }
                    supports.push([P::new(x as i32, s as i32, z as i32), P::new(x as i32, t as i32, z as i32)]);
                }
                y += 1;
            }
        }
    }
    eprintln!("support = {:?}", supports);
    /// bs[i][j] := bid of the bot targetting supports[i][j]
    let mut bs: Vec<[Option<usize>; 2]> = vec![[None; 2]; supports.len()];
    /// working[i] := the target of the bod i
    let mut working: Vec<Option<(usize, usize)>> = vec![None; bots.len()];
    let mut finished = vec![false; supports.len()];
    let mut rem = supports.len();
    let mut t = bots[0].commands.len();
    let mut occupied = InitV3::new(false, r);
    let mut bpos = InitV3::new(!0, r);
    let mut ws = InitV3::new(false, r);
    let mut bfs = BFS::new(r);
    let mut energy = 0;
    while rem > 0 {
        eprintln!("rem: {}", rem);
        let mut free = BTreeSet::new();
        occupied.init();
        for b in bots.iter() {
            occupied[b.p] = true;
            if working[b.bid].is_none() {
                free.insert(b.bid);
            }
        }
        let mut moved = vec![false; bots.len()];
        for b in bots.iter_mut() {
            if b.commands.len() > t {
                if check_occupied(b.p, b.commands[t], &occupied) {
                    set_occupied(b.p, b.commands[t], &mut occupied);
                    moved[b.bid] = true;
                } else {
                    if let Some((i, j)) = working[b.bid] {
                        bs[i][j] = None;
                        working[b.bid] = None;
                        if supports[i][0] == supports[i][1] {
                            bs[i][j] = None;
                        }
                        b.commands.truncate(t);
                    } else {
                        assert!(false);
                    }
                }
            }
        }
        if !free.is_empty() {
            for i in 0..bs.len() {
                if finished[i] {
                    continue;
                }
                for j in 0..2 {
                    if !free.is_empty() && bs[i][j].is_none() {
                        bpos.init();
                        ws.init();
                        for b in bots.iter() {
                            ws[b.p] = true;
                        }
                        for &bid in &free {
                            bpos[bots[bid].p] = bid;
                            ws[bots[bid].p] = false;
                        }
                        let mut starts = vec![];
                        for t in supports[i][j].near(r) {
                            if !filled[t] {
                                starts.push(t);
                            }
                        }
                        bfs.clear();
                        if let Some(s) = bfs.bfs(|p| filled[p] || ws[p], &starts, |p| bpos[p] != !0) {
                            let bid = bpos[s];
                            assert!(bid != !0);
                            free.remove(&bid);
                            bs[i][j] = Some(bid);
                            working[bid] = Some((i, j));
                            bots[bid].commands.extend(bfs.restore_backward(s));
                            if supports[i][0] == supports[i][1] {
                                bs[i][j] = Some(bid);
                            }
                            break;
                        }
                    }
                }
            }
        }
        eprintln!("{:?}", working);
        for i in 0..bs.len() {
            if finished[i] {
                continue;
            }
            match bs[i] {
                [Some(a), Some(b)] if bots[a].commands.len() == t && bots[b].commands.len() == t => {
                    if supports[i][0] == supports[i][1] {
                        let ca = Command::Void(supports[i][0] - bots[a].p);
                        bots[a].commands.push(ca);
                    } else {
                        let ca = Command::GVoid(supports[i][0] - bots[a].p, supports[i][1] - supports[i][0]);
                        bots[a].commands.push(ca);
                        let cb = Command::GVoid(supports[i][1] - bots[b].p, supports[i][0] - supports[i][1]);
                        bots[b].commands.push(cb);
                    }
                    finished[i] = true;
                    for y in supports[i][0].y..=supports[i][1].y {
                        let mut p = supports[i][0];
                        p.y = y;
                        filled[p] = false;
                    }
                    working[a] = None;
                    working[b] = None;
                    bs[i] = [None; 2];
                    rem -= 1;
                },
                _ => {
                }
            }
        }
        for b in bots.iter_mut() {
            if moved[b.bid] {
                continue;
            }
            if b.commands.len() > t {
                if check_occupied(b.p, b.commands[t], &occupied) {
                    set_occupied(b.p, b.commands[t], &mut occupied);
                } else {
                    if let Some((i, j)) = working[b.bid] {
                        bs[i][j] = None;
                        working[b.bid] = None;
                        if supports[i][0] == supports[i][1] {
                            bs[i][j] = None;
                        }
                        b.commands.truncate(t);
                        b.commands.push(Command::Wait);
                    } else {
                        assert!(false);
                    }
                }
            } else {
                b.commands.push(Command::Wait);
            }
        }
        for b in bots.iter_mut() {
            match b.commands[t] {
                Command::SMove(d) => {
                    b.p += d;
                },
                Command::LMove(d1, d2) => {
                    b.p += d1 + d2;
                },
                _ => {
                }
            }
        }
        t += 1;
        energy += 1;
    }
    for b in bots.iter_mut() {
        b.commands.truncate(t);
    }
    energy
}