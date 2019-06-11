#![allow(unused)]
extern crate wata;

use wata::*;
use wata::bfs::*;
use std::collections::*;

const FISSION: bool = true;//false;

#[derive(Clone, Debug)]
struct Bot {
	bid: usize,
	p: P,
	commands: Vec<Command>,
}

fn one_step<F: Fn(i32, i32) -> bool>(x: i32, z: i32, r: usize, filled: F) -> Vec<(i32, i32, Command)> {
	let mut ps = vec![];
	for &(dx, dz) in &[(-1, 0), (0, -1), (0, 1), (1, 0)] {
		for d in 1..16 {
			let x2 = x + dx * d;
			let z2 = z + dz * d;
			if x2 < 0 || x2 >= r as i32 || z2 < 0 || z2 >= r as i32 || filled(x2, z2) {
				break;
			}
			ps.push((x2 - x, z2 - z, Command::SMove(P::new(dx * d, 0, dz * d))));
			if d <= 5 {
				for &(dx2, dz2) in &[(-dz, dx), (dz, -dx)] {
					for d2 in 1..6 {
						let x3 = x2 + dx2 * d2;
						let z3 = z2 + dz2 * d2;
						if x3 < 0 || x3 >= r as i32 || z3 < 0 || z3 >= r as i32 || filled(x3, z3) {
							break;
						}
						ps.push((x3 - x, z3 - z, Command::LMove(P::new(dx * d, 0, dz * d), P::new(dx2 * d2, 0, dz2 * d2))));
					}
				}
			}
		}
	}
	ps
}

fn output_layer(target: &V3<bool>, filled: &V3<bool>, ground: &Vec<Vec<bool>>, bots: &Vec<Bot>, y0: i32) {
	let r = target.len();
	let mut out = mat!['.'; r; r];
	for x in 0..r {
		for z in 0..r {
			out[x][z] = if filled[x][y0 as usize][z] {
				'f'
			} else if ground[x][z] {
				'g'
			} else if target[x][y0 as usize][z] {
				't'
			} else {
				'.'
			};
		}
	}
	for b in bots {
		let c = &mut out[b.p.x as usize][b.p.z as usize];
		if *c == '.' {
			*c = 'B'
		} else {
			*c = c.to_uppercase().to_string().chars().nth(0).unwrap();
		}
	}
	for x in 0..r {
		for z in 0..r {
			eprint!("{}", out[x][z]);
		}
		eprintln!();
	}
}

fn destruct_support(target: &V3<bool>, filled: &mut V3<bool>, bots: &mut Vec<Bot>) -> i64 {
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

struct RegionIter {
	min_x: i32,
	max_x: i32,
	min_y: i32,
	max_y: i32,
	min_z: i32,
	max_z: i32,
	x: i32,
	y: i32,
	z: i32,
}

impl Iterator for RegionIter {
	type Item = P;
	fn next(&mut self) -> Option<P> {
		self.z += 1;
		if self.z > self.max_z {
			self.z = self.min_z;
			self.y += 1;
			if self.y > self.max_y {
				self.y = self.min_y;
				self.x += 1;
				if self.x > self.max_x {
					return None;
				}
			}
		}
		Some(P::new(self.x, self.y, self.z))
	}
}

fn region(p: P, q: P) -> RegionIter {
	RegionIter {
		min_x: p.x.min(q.x),
		max_x: p.x.max(q.x),
		min_y: p.y.min(q.y),
		max_y: p.y.max(q.y),
		min_z: p.z.min(q.z),
		max_z: p.z.max(q.z),
		x: p.x.min(q.x),
		y: p.y.min(q.y),
		z: p.z.min(q.z) - 1,
	}
}

fn fill_layer<I: Fn(i32, i32) -> P, X: Fn(P) -> usize, Y: Fn(P) -> usize, Z: Fn(P) -> usize, G: Fn(usize, usize) -> bool>
			(target: &V3<bool>, filled: &mut V3<bool>, occupied: &mut InitV3<bool>, bots: &mut Vec<Bot>, dir: P,
				pos: I, get_x: X, get_y: Y, get_z: Z, is_grounded: G, y0: usize) -> i64 {
	let r = target.len();
	let nbots = bots.len();
	let mut energy = 0;
	let mut rem = 0;
	for x in 0..r {
		for z in 0..r {
			if target[pos(x as i32, z as i32)] {
				rem += 1;
			}
		}
	}
	let first = get_y(bots[0].p) != y0;
	if !first {
		for b in bots.iter_mut() {
			b.p -= dir;
			b.commands.push(Command::SMove(-dir));
		}
		energy += 1;
	}
	let mut ground = mat![false; r; r];
	for x in 0..r {
		for z in 0..r {
			if target[pos(x as i32, z as i32)] && (is_grounded(x, z) || !first && filled[pos(x as i32, z as i32) + dir]) {
				ground[x][z] = true;
			}
		}
	}
	let mut near_vb = mat![vec![]; r; r];
	let mut t = bots[0].commands.len();
	let mut bpos = mat![!0; r; r];
	while rem > 0 {
		eprintln!("y = {}, rem = {}", y0, rem);
		// output_layer(target, filled, &ground, &bots, y0);
		occupied.init();
		for b in bots.iter() {
			occupied[b.p] = true;
		}
		for b in bots.iter() {
			bpos[get_x(b.p)][get_z(b.p)] = b.bid;
		}
		// gfill
		loop {
			let mut max_size = 2;
			let mut gfill = vec![];
			for i in 0..nbots {
				if bots[i].commands.len() > t {
					continue;
				}
				for j in 0..i {
					if bots[j].commands.len() > t {
						continue;
					}
					for p in bots[i].p.near(r) {
						if get_y(p) != y0 {
							continue;
						}
						for q in bots[j].p.near(r) {
							if get_y(q) != y0 {
								continue;
							}
							if p == q {
								continue;
							}
							if (get_x(p) == get_x(q) || get_z(p) == get_z(q)) && (p - q).mlen() + 1 > max_size && (p - q).mlen() <= 30 {
								let mut count = 0;
								let mut ok = false;
								let mut ng = false;
								for a in region(p, q) {
									if occupied[a] || !target[a] {
										ng = true;
										break;
									}
									if ground[get_x(a)][get_z(a)] {
										ok = true;
									}
									if !filled[a] {
										count += 1;
									}
								}
								if ok && !ng && max_size.setmax(count) {
									gfill = vec![(i, Command::GFill(p - bots[i].p, q - p)), (j, Command::GFill(q - bots[j].p, p - q))];
								}
							} else {
								let min_x = get_x(p).min(get_x(q)) as i32;
								let max_x = get_x(p).max(get_x(q)) as i32;
								let min_z = get_z(p).min(get_z(q)) as i32;
								let max_z = get_z(p).max(get_z(q)) as i32;
								if (max_x - min_x + 1) * (max_z - min_z + 1) <= 4.max(max_size) || max_x - min_x > 30 || max_z - min_z > 30 || (p != pos(min_x, min_z) && p != pos(max_x, max_z)) {
									continue;
								}
								let mut i2 = !0;
								for p2 in pos(min_x, max_z).near(r) {
									if get_y(p2 + dir) != y0 || bpos[get_x(p2)][get_z(p2)] == !0 {
										continue;
									}
									let id = bpos[get_x(p2)][get_z(p2)];
									if id == i || id == j || bots[id].commands.len() > t {
										continue;
									}
									i2 = id;
									break;
								}
								if i2 == !0 {
									continue;
								}
								let mut j2 = !0;
								for q2 in pos(max_x, min_z).near(r) {
									if get_y(q2 + dir) != y0 || bpos[get_x(q2)][get_z(q2)] == !0 {
										continue;
									}
									let id = bpos[get_x(q2)][get_z(q2)];
									if id == i || id == j || id == i2 || bots[id].commands.len() > t {
										continue;
									}
									j2 = id;
									break;
								}
								if j2 == !0 {
									continue;
								}
								let mut count = 0;
								let mut ok = false;
								let mut ng = false;
								for a in region(p, q) {
									if occupied[a] || !target[a] {
										ng = true;
										break;
									}
									if ground[get_x(a)][get_z(a)] {
										ok = true;
									}
									if !filled[a] {
										count += 1;
									}
								}
								if ok && !ng && count > 4 && max_size.setmax(count) {
									gfill = vec![
										(i, Command::GFill(p - bots[i].p, q - p)),
										(j, Command::GFill(q - bots[j].p, p - q)),
										(i2, Command::GFill(pos(min_x, max_z) - bots[i2].p, pos(max_x, min_z) - pos(min_x, max_z))),
										(j2, Command::GFill(pos(max_x, min_z) - bots[j2].p, pos(min_x, max_z) - pos(max_x, min_z)))
									];
									for &(_, c) in &gfill {
										if let Command::GFill(p, _) = c {
											assert!(p.mlen() <= 2);
										}
									}
								}
							}
						}
					}
				}
			}
			if max_size <= 2 {
				break;
			}
			for &(i, c) in &gfill {
				bots[i].commands.push(c);
			}
			if let (i, Command::GFill(d1, d2)) = gfill[0] {
				eprintln!("GFill: {} ({:?})", max_size, d2);
				for a in region(bots[i].p + d1, bots[i].p + d1 + d2) {
					occupied[a] = true;
				}
			} else {
				assert!(false);
			}
		}
		for b in bots.iter() {
			bpos[get_x(b.p)][get_z(b.p)] = !0;
		}
		let mut near_bv = vec![vec![]; nbots];
		for b in bots.iter() {
			for p in b.p.near(r) {
				if get_y(p) == y0 && target[p] && !filled[p] && !occupied[p] {
					near_bv[b.bid].push(p);
					near_vb[get_x(p)][get_z(p)].push(b.bid);
				}
			}
		}
		
		// fill
		for b in bots.iter_mut() {
			if b.commands.len() > t {
				continue;
			}
			let mut min_size = 100;
			let mut q = P::new(0, 0, 0);
			for p in b.p.near(r) {
				if get_y(p) == y0 && ground[get_x(p)][get_z(p)] && !filled[p] && !occupied[p] {
					if min_size.setmin(near_vb[get_x(p)][get_z(p)].len()) {
						q = p;
					}
				}
			}
			if min_size < 100 {
				b.commands.push(Command::Fill(q - b.p));
				occupied[q] = true;
			}
		}
		macro_rules! score {
			($p:expr) => { {
				let p = $p;
				let mut ok = false;
				for q in p.near(r) {
					if get_y(q) == y0 {
						if target[q] && !filled[q] && ground[get_x(q)][get_z(q)] && !occupied[q] {
							ok = true;
							break;
						}
					}
				}
				let mut score = 0.0;
				if !ok {
					score = -1.0;
				} else {
					for q in p.near(r) {
						if get_y(q) == y0 {
							if target[q] && !filled[q] && !occupied[q] {
								if near_vb[get_x(q)][get_z(q)].len() == 0 {
									score += 1.0;
								} else {
									let mut ok = true;
									for &bid in &near_vb[get_x(q)][get_z(q)] {
										if near_bv[bid].len() <= 2 {
											ok = false;
										}
									}
									if ok {
										score += 0.1;
									}
								}
							}
						}
					}
				}
				score
			} };
		}
		let dxz = |dx, dz| {
			let mut d = pos(dx, dz);
			if dir.x != 0 {
				d.x = 0;
			} else if dir.y != 0 {
				d.y = 0;
			} else {
				d.z = 0;
			}
			d
		};
		// move 1
		for b in bots.iter_mut() {
			if b.commands.len() > t {
				continue;
			}
			let mut best = -1.0;
			let mut to = P::new(0, 0, 0);
			let mut com = Command::Wait;
			for (dx, dz, command) in one_step(get_x(b.p) as i32, get_z(b.p) as i32, r, |x, z| occupied[pos(x, z) - dir]) {
				let p = b.p + dxz(dx, dz);
				let score = score!(p);
				if best.setmax(score) {
					to = p;
					com = match command {
						Command::SMove(d) => {
							Command::SMove(dxz(d.x, d.z))
						},
						Command::LMove(d1, d2) => {
							Command::LMove(dxz(d1.x, d1.z), dxz(d2.x, d2.z))
						},
						_ => {
							unreachable!()
						}
					};
				}
			}
			if best > 0.0 {
				for q in to.near(r) {
					if get_y(q) == y0 && target[q] && !filled[q] && !occupied[q] {
						occupied[q] = true;
						break;
					}
				}
				b.commands.push(com);
				set_occupied(b.p, com, occupied);
			}
		}
		// move many
		for d in 1..r as i32 {
			let mut ok = true;
			for b in bots.iter_mut() {
				if b.commands.len() > t {
					continue;
				}
				let mut best = -1.0;
				let mut to = P::new(0, 0, 0);
				for &dx in &[-d, d] {
					for dz in -d..=d {
						for r in 0..2 {
							let p = if r == 0 {
								b.p + dxz(dx, dz)
							} else {
								b.p + dxz(dz, dx)
							};
							let score = score!(p);
							if best.setmax(score) {
								to = p;
							}
						}
					}
				}
				if best > 0.0 {
					for q in to.near(r) {
						if get_y(q) == y0 && target[q] && !filled[q] && !occupied[q] {
							occupied[q] = true;
							break;
						}
					}
					let mut min_dist = (b.p - to).mlen();
					let mut com = Command::Wait;
					for (dx, dz, command) in one_step(get_x(b.p) as i32, get_z(b.p) as i32, r, |x, z| occupied[pos(x, z) - dir]) {
						let p = b.p + dxz(dx, dz);
						let dist = (p - to).mlen();
						if min_dist.setmin(dist) {
							com = match command {
								Command::SMove(d) => {
									Command::SMove(dxz(d.x, d.z))
								},
								Command::LMove(d1, d2) => {
									Command::LMove(dxz(d1.x, d1.z), dxz(d2.x, d2.z))
								},
								_ => {
									unreachable!()
								}
							};
						}
					}
					b.commands.push(com);
					set_occupied(b.p, com, occupied);
				} else {
					ok = false;
				}
			}
			if ok {
				break;
			}
		}
		// wait
		for b in bots.iter_mut() {
			if b.commands.len() == t {
				b.commands.push(Command::Wait);
			}
		}
		for bid in 0..nbots {
			for &p in &near_bv[bid] {
				near_vb[get_x(p)][get_z(p)].clear();
			}
		}
		eprintln!("{:?}", bots.iter().map(|b| b.commands.last().unwrap()).collect::<Vec<_>>());
		for b in bots.iter_mut() {
			match b.commands[t] {
				Command::SMove(d) => {
					b.p += d;
				},
				Command::LMove(d1, d2) => {
					b.p += d1 + d2;
				},
				Command::Fill(d) => {
					let p = b.p + d;
					assert!(!filled[p]);
					filled[p] = true;
					rem -= 1;
					for q in p.adj(r) {
						if get_y(q) == y0 && target[q] && !ground[get_x(q)][get_z(q)] {
							ground[get_x(q)][get_z(q)] = true;
						}
					}
				},
				Command::GFill(d1, d2) => {
					for p in region(b.p + d1, b.p + d1 + d2) {
						assert!(target[p]);
						if !filled[p] {
							filled[p] = true;
							ground[get_x(p)][get_z(p)] = true;
							rem -= 1;
							for q in p.adj(r) {
								if get_y(q) == y0 && target[q] && !ground[get_x(q)][get_z(q)] {
									ground[get_x(q)][get_z(q)] = true;
								}
							}
						}
					}
				}
				_ => {
				}
			}
		}
		energy += 1;
		t += 1;
	}
	// output_layer(target, filled, &ground, &bots, y0);
	energy
}

fn make_jobs<I: Fn(i32, i32) -> P, X: Fn(P) -> usize, Y: Fn(P) -> usize, Z: Fn(P) -> usize, D: Fn(i32, i32) -> P>
		(target: &V3<bool>, ground: &Vec<Vec<bool>>, pos: &I, get_x: &X, get_y: &Y, get_z: &Z, dxz: &D, y0: usize) -> Vec<Vec<P>> {
	let r = target.len();
	let mut ground = ground.clone();
	let mut jobs = vec![];
	let mut filled = mat![false; r; r];
	let mut que = VecDeque::new();
	for x in 0..r {
		for z in 0..r {
			if ground[x][z] {
				que.push_back(pos(x as i32, z as i32));
			}
		}
	}
	macro_rules! push_adj {
		($p:expr) => { {
			let p = $p;
			for q in p.adj(r) {
				if get_y(q) == y0 && target[q] && !filled[get_x(q)][get_z(q)] && !ground[get_x(q)][get_z(q)] {
					ground[get_x(q)][get_z(q)] = true;
					que.push_back(q);
				}
			}
		} };
	}
	while let Some(p) = que.pop_front() {
		if filled[get_x(p)][get_z(p)] {
			continue;
		}
		let mut lx = 0;
		while lx <= 30 && get_x(p) as i32 - lx >= 0 && target[p + dxz(-lx, 0)] && !filled[get_x(p) - lx as usize][get_z(p)] {
			lx += 1;
		}
		lx -= 1;
		let mut rx = 0;
		while lx + rx <= 30 && (get_x(p) + rx as usize) < r && target[p + dxz(rx, 0)] && !filled[get_x(p) + rx as usize][get_z(p)] {
			rx += 1;
		}
		rx -= 1;
		let mut lz = 0;
		while lz <= 30 && get_z(p) as i32 - lz >= 0 && target[p + dxz(0, -lz)] && !filled[get_x(p)][get_z(p) - lz as usize] {
			lz += 1;
		}
		lz -= 1;
		let mut rz = 0;
		while lz + rz <= 30 && (get_z(p) + rz as usize) < r && target[p + dxz(0, rz)] && !filled[get_x(p)][get_z(p) + rz as usize] {
			rz += 1;
		}
		rz -= 1;
		let mut num_x = 0;
		for dx in -lx..=rx {
			if !ground[(get_x(p) as i32 + dx) as usize][get_z(p)] {
				num_x += 1;
			}
		}
		let mut num_z = 0;
		for dz in -lz..=rz {
			if !ground[get_x(p)][(get_z(p) as i32 + dz) as usize] {
				num_z += 1;
			}
		}
		// if lx + rx + num_x <= 3 && lz + rz + num_z <= 3 {
		if lx + rx + num_x <= 2 && lz + rz + num_z <= 2 {
			filled[get_x(p)][get_z(p)] = true;
			jobs.push(vec![p]);
			push_adj!(p);
		} else if lx + rx + num_x > lz + rz + num_z {
			lz = 0;
			while lz <= 30 && get_z(p) as i32 - lz >= 0 {
				let z = get_z(p) as i32 - lz;
				let mut ok = true;
				for x in get_x(p) as i32 - lx ..= get_x(p) as i32 + rx {
					if !target[pos(x, z)] || filled[x as usize][z as usize] {
						ok = false;
						break;
					}
				}
				if !ok {
					break;
				}
				lz += 1;
			}
			lz -= 1;
			rz = 0;
			while lz + rz <= 30 && (get_z(p) + rz as usize) < r {
				let z = get_z(p) as i32 + rz;
				let mut ok = true;
				for x in get_x(p) as i32 - lx ..= get_x(p) as i32 + rx {
					if !target[pos(x, z)] || filled[x as usize][z as usize] {
						ok = false;
						break;
					}
				}
				if !ok {
					break;
				}
				rz += 1;
			}
			rz -= 1;
			if lz + rz <= 3 {
			// if lz + rz <= 2 {
				for x in get_x(p) as i32 - lx ..= get_x(p) as i32 + rx {
					filled[x as usize][get_z(p)] = true;
				}
				jobs.push(vec![p + dxz(-lx, 0), p + dxz(rx, 0)]);
				for dx in -lx..=rx {
					push_adj!(p + dxz(dx, 0));
				}
			} else {
				for x in get_x(p) as i32 - lx ..= get_x(p) as i32 + rx {
					for z in get_z(p) as i32 - lz ..= get_z(p) as i32 + rz {
						filled[x as usize][z as usize] = true;
					}
				}
				jobs.push(vec![p + dxz(-lx, -lz), p + dxz(-lx, rz), p + dxz(rx, -lz), p + dxz(rx, rz)]);
				for dx in -lx..=rx {
					for dz in -lz..=rz {
						push_adj!(p + dxz(dx, dz));
					}
				}
			}
		} else {
			lx = 0;
			while lx <= 30 && get_x(p) as i32 - lx >= 0 {
				let x = get_x(p) as i32 - lx;
				let mut ok = true;
				for z in get_z(p) as i32 - lz ..= get_z(p) as i32 + rz {
					if !target[pos(x, z)] || filled[x as usize][z as usize] {
						ok = false;
						break;
					}
				}
				if !ok {
					break;
				}
				lx += 1;
			}
			lx -= 1;
			rx = 0;
			while lx + rx <= 30 && (get_x(p) + rx as usize) < r {
				let x = get_x(p) as i32 + rx;
				let mut ok = true;
				for z in get_z(p) as i32 - lz ..= get_z(p) as i32 + rz {
					if !target[pos(x, z)] || filled[x as usize][z as usize] {
						ok = false;
						break;
					}
				}
				if !ok {
					break;
				}
				rx += 1;
			}
			rx -= 1;
			if lx + rx <= 3 {
				for z in get_z(p) as i32 - lz ..= get_z(p) as i32 + rz {
					filled[get_x(p)][z as usize] = true;
				}
				jobs.push(vec![p + dxz(0, -lz), p + dxz(0, rz)]);
				for dz in -lz..=rz {
					push_adj!(p + dxz(0, dz));
				}
			} else {
				for x in get_x(p) as i32 - lx ..= get_x(p) as i32 + rx {
					for z in get_z(p) as i32 - lz ..= get_z(p) as i32 + rz {
						filled[x as usize][z as usize] = true;
					}
				}
				jobs.push(vec![p + dxz(-lx, -lz), p + dxz(-lx, rz), p + dxz(rx, -lz), p + dxz(rx, rz)]);
				for dx in -lx..=rx {
					for dz in -lz..=rz {
						push_adj!(p + dxz(dx, dz));
					}
				}
			}
		}
	}
	for x in 0..r {
		for z in 0..r {
			if target[pos(x as i32, z as i32)] {
				assert!(filled[x][z]);
			}
		}
	}
	jobs
}

fn fill_layer2<I: Fn(i32, i32) -> P, X: Fn(P) -> usize, Y: Fn(P) -> usize, Z: Fn(P) -> usize, G: Fn(usize, usize) -> bool>
			(target: &V3<bool>, filled: &mut V3<bool>, occupied: &mut InitV3<bool>, bots: &mut Vec<Bot>, dir: P,
				pos: I, get_x: X, get_y: Y, get_z: Z, is_grounded: G, y0: usize) -> i64 {
	let r = target.len();
	let nbots = bots.len();
	let mut rem = 0;
	for x in 0..r {
		for z in 0..r {
			if target[pos(x as i32, z as i32)] {
				rem += 1;
			}
		}
	}
	let mut energy = 0;
	let first = get_y(bots[0].p) != y0;
	if !first {
		for b in bots.iter_mut() {
			b.p -= dir;
			b.commands.push(Command::SMove(-dir));
		}
		energy += 1;
	}
	let mut ground = mat![false; r; r];
	for x in 0..r {
		for z in 0..r {
			if target[pos(x as i32, z as i32)] && (is_grounded(x, z) || !first && filled[pos(x as i32, z as i32) + dir]) {
				ground[x][z] = true;
			}
		}
	}
	let dxz = |dx, dz| {
		let mut d = pos(dx, dz);
		if dir.x != 0 {
			d.x = 0;
		} else if dir.y != 0 {
			d.y = 0;
		} else {
			d.z = 0;
		}
		d
	};
	let mut jobs = make_jobs(target, &ground, &pos, &get_x, &get_y, &get_z, &dxz, y0);
	eprintln!("jobs = {:?}", jobs);
	let mut t = bots[0].commands.len();
	let mut endpoints = mat![(!0, !0); r; r];
	let mut vtoj = mat![!0; r; r];
	let mut job_grounded = vec![false; jobs.len()];
	let mut job_grounded2 = vec![false; jobs.len()];
	for j in 0..jobs.len() {
		for i in 0..jobs[j].len() {
			endpoints[get_x(jobs[j][i])][get_z(jobs[j][i])] = (j, i);
		}
		for p in region(jobs[j][0], jobs[j][jobs[j].len() - 1]) {
			vtoj[get_x(p)][get_z(p)] = j;
			if ground[get_x(p)][get_z(p)] {
				job_grounded[j] = true;
			}
		}
	}
	for x in 0..r {
		for z in 0..r {
			if target[pos(x as i32, z as i32)] {
				assert!(vtoj[x][z] != !0);
			}
		}
	}
	let mut finished = vec![false; jobs.len()];
	let mut reserved: Vec<_> = jobs.iter().map(|job| vec![!0; job.len()]).collect();
	let mut work = vec![(!0, !0); nbots];
	while rem > 0 {
		eprintln!("y = {}, rem = {}", y0, rem);
		occupied.init();
		for b in bots.iter() {
			occupied[b.p] = true;
		}
		let mut num_grounded = 0;
		for j in 0..jobs.len() {
			if !finished[j] && job_grounded[j] {
				num_grounded += 1;
			}
		}
		eprintln!("{} / {}", num_grounded, jobs.len());
		// job assign
		loop {
			let mut free = 0;
			for b in 0..nbots {
				if work[b].0 == !0 {
					free += 1;
				}
			}
			let mut min_d = i32::max_value();
			let mut from = !0;
			let mut to = (!0, !0);
			for b in 0..nbots {
				if work[b].0 != !0 {
					continue;
				}
				for j in 0..jobs.len() {
					if finished[j] || !job_grounded[j] || reserved[j][0] != !0 || free < jobs[j].len() {
						continue;
					}
					for k in 0..jobs[j].len() {
						if min_d.setmin((bots[b].p - jobs[j][k]).mlen()) {
							from = b;
							to = (j, k);
						}
					}
				}
			}
			if from == !0 {
				for b in 0..nbots {
					if work[b].0 != !0 {
						continue;
					}
					for j in 0..jobs.len() {
						if finished[j] || !job_grounded2[j] || reserved[j][0] != !0 || free < jobs[j].len() {
							continue;
						}
						for k in 0..jobs[j].len() {
							if min_d.setmin((bots[b].p - jobs[j][k]).mlen()) {
								from = b;
								to = (j, k);
							}
						}
					}
				}
				if from == !0 {
					break;
				}
			}
			work[from] = to;
			let j = to.0;
			reserved[j][to.1] = from;
			for k in 0..jobs[j].len() {
				if k == to.1 {
					continue;
				}
				let mut min_d = i32::max_value();
				let mut from2 = !0;
				for b in 0..nbots {
					if work[b].0 != !0 {
						continue;
					}
					if min_d.setmin((bots[b].p - jobs[j][k]).mlen()) {
						from2 = b;
					}
				}
				work[from2] = (j, k);
				reserved[j][k] = from2;
			}
			// if job_grounded[j] {
				for p in region(jobs[j][0], jobs[j][jobs[j].len() - 1]) {
					for q in p.adj(r) {
						if target[q] {
							job_grounded2[vtoj[get_x(q)][get_z(q)]] = true;
						}
					}
				}
			// }
		}
		// job change
		for b in 0..nbots {
			if work[b].0 == !0 {
				let mut best = 0;
				let mut to = !0;
				for b2 in 0..nbots {
					if b2 == b || work[b2].0 == !0 {
						continue;
					}
					let t = jobs[work[b2].0][work[b2].1];
					let mut improve = (t - bots[b2].p).mlen() - (t - bots[b].p).mlen();
					if best.setmax(improve) {
						to = b2;
					}
				}
				if to != !0 {
					work[b] = work[to];
					reserved[work[b].0][work[b].1] = b;
					work[to] = (!0, !0);
				}
			}
		}
		
		// fill
		loop {
			let mut modified = false;
			for j in 0..jobs.len() {
				if finished[j] || reserved[j][0] == !0 || !job_grounded[j] {
					continue;
				}
				let mut ok = true;
				for k in 0..jobs[j].len() {
					let b = reserved[j][k];
					if !(bots[b].p - jobs[j][k]).is_near() {
						ok = false;
						break;
					}
				}
				if !ok {
					continue;
				}
				for p in region(jobs[j][0], jobs[j][jobs[j].len() - 1]) {
					assert!(target[p]);
					filled[p] = true;
					for q in p.adj(r) {
						if target[q] {
							ground[get_x(q)][get_z(q)] = true;
							job_grounded[vtoj[get_x(q)][get_z(q)]] = true;
						}
					}
					rem -= 1;
				}
				if jobs[j].len() == 1 {
					let b = reserved[j][0];
					let bp = bots[b].p;
					bots[b].commands.push(Command::Fill(jobs[j][0] - bp));
				} else {
					for k in 0..jobs[j].len() {
						let b = reserved[j][k];
						let bp = bots[b].p;
						bots[b].commands.push(Command::GFill(jobs[j][k] - bp, jobs[j][jobs[j].len() - 1 - k] - jobs[j][k]));
					}
				}
				finished[j] = true;
				for k in 0..jobs[j].len() {
					let b = reserved[j][k];
					reserved[j][k] = !0;
					work[b] = (!0, !0);
				}
				modified = true;
			}
			if !modified {
				break;
			}
		}
		// move
		for b in 0..bots.len() {
			if bots[b].commands.len() > t {
				continue;
			} else if work[b].0 == !0 {
				bots[b].commands.push(Command::Wait);
				continue;
			}
			occupied[bots[b].p] = false;
			let t = jobs[work[b].0][work[b].1];
			let mut min_d = (bots[b].p - t).mlen();
			let mut command = Command::Wait;
			for (dx, dz, com) in one_step(get_x(bots[b].p) as i32, get_z(bots[b].p) as i32, r, |x, z| occupied[pos(x, z) - dir]) {
				let p = bots[b].p + dxz(dx, dz);
				if min_d.setmin((p - t).mlen()) {
					command = match com {
						Command::SMove(d) => {
							Command::SMove(dxz(d.x, d.z))
						},
						Command::LMove(d1, d2) => {
							Command::LMove(dxz(d1.x, d1.z), dxz(d2.x, d2.z))
						},
						_ => {
							unreachable!()
						}
					};
				}
			}
			occupied[bots[b].p] = true;
			bots[b].commands.push(command);
			set_occupied(bots[b].p, command, occupied);
			match command {
				Command::SMove(d) => {
					bots[b].p += d;
				},
				Command::LMove(d1, d2) => {
					bots[b].p += d1 + d2;
				},
				_ => {
				}
			}
		}
		energy += 1;
		t += 1;
	}
	energy
}

fn fill_layer_z(target: &V3<bool>, filled: &mut V3<bool>, occupied: &mut InitV3<bool>, bots: &mut Vec<Bot>, z0: i32, dir: i32) -> i64 {
	fill_layer2(target, filled, occupied, bots, P::new(0, 0, dir), |x, y| P::new(x, y, z0), |p| p.x as usize, |p| p.z as usize, |p| p.y as usize, |_, y| y == 0, z0 as usize)
}

fn fill_layer_bottom(target: &V3<bool>, filled: &mut V3<bool>, occupied: &mut InitV3<bool>, bots: &mut Vec<Bot>, y0: i32) -> i64 {
	fill_layer2(target, filled, occupied, bots, P::new(0, -1, 0), |x, z| P::new(x, y0, z), |p| p.x as usize, |p| p.y as usize, |p| p.z as usize, |_, _| y0 == 0, y0 as usize)
}

fn target_bottom_up(target: &V3<bool>) -> V3<bool> {
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

// [0, z0), [z0, r)
fn target_z(target: &V3<bool>, z0: usize) -> V3<bool> {
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
	for &dir in &[-1, 1] {
		for z in (if dir < 0 { (z0..r).collect::<Vec<_>>() } else { (0..z0).rev().collect() }) {
			if z != z0 && z != z0 - 1 {
				for x in 0..r {
					for y in 0..r {
						if target2[x][y][z] && ground[x][y][(z as i32 + dir) as usize] {
							ground[x][y][z] = true;
						}
					}
				}
			}
			let mut stack = vec![];
			for x in 0..r {
				for y in 0..r {
					if ground[x][y][z] {
						stack.push(P::new(x as i32, y as i32, z as i32));
					}
				}
			}
			loop {
				while let Some(p) = stack.pop() {
					for q in p.adj(r) {
						if q.z == z as i32 && target2[q] && !ground[q] {
							ground[q] = true;
							stack.push(q);
						}
					}
				}
				let mut max_d = -2;
				let mut q = P::new(0, 0, 0);
				for x in 0..r {
					for y in 0..r {
						if target2[x][y][z] && !ground[x][y][z] {
							let mut d = -1;
							for y2 in (0..y).rev() {
								if target2[x][y2][z] && ground[x][y2][z] {
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
				eprintln!("support: {:?} : {}", q, q.y as i32 - max_d);
				ground[q] = true;
				stack.push(q);
				for y2 in (0..q.y).rev() {
					if target2[q.x as usize][y2 as usize][q.z as usize] && ground[q.x as usize][y2 as usize][q.z as usize] {
						break;
					}
					target2[q.x as usize][y2 as usize][q.z as usize] = true;
				}
			}
		}
	}
	target2
}

fn solve_bottom_up(target: &V3<bool>, nbots: usize) -> (i64, Vec<Command>) {
	let r = target.len();
	let target2 = target_bottom_up(target);
	let mut energy = 0;
	let mut init = vec![];
	for x in 0..r {
		for z in 0..r {
			if target2[x][0][z] {
				init.push(P::new(x as i32, 1, z as i32));
			}
		}
	}
	if init.len() >= nbots {
		let mut init2 = vec![];
		for i in 0..nbots {
			init2.push(init[i * init.len() / nbots]);
		}
		init = init2;
	} else {
		// TODO
		for x in 0..r {
			for z in 0..r {
				if init.len() < nbots && !target2[x][0][z] {
					init.push(P::new(x as i32, 1, z as i32));
				}
			}
		}
	}
	
	let mut filled = mat![false; r; r; r];
	let (bids, mut commands) = if FISSION {
		fission_to(&filled, &init)
	} else {
		((0..nbots).collect(), vec![])
	};
	
	let mut bots = vec![];
	for i in 0..nbots {
		bots.push(Bot { bid: bids[i], p: init[i], commands: vec![] });
	}
	bots.sort_by_key(|b| b.bid);
	for i in 0..nbots {
		bots[i].bid = i;
	}
	let mut occupied = InitV3::new(false, r);
	let mut max_r = 0;
	for x in 0..r {
		for y in 0..r {
			for z in 0..r {
				if target2[x][y][z] {
					max_r.setmax(y);
				}
			}
		}
	}
	for y in 0..=max_r {
		energy += fill_layer_bottom(&target2, &mut filled, &mut occupied, &mut bots, y as i32);
	}
	energy += destruct_support(&target, &mut filled, &mut bots);
	let t_max = bots.iter().map(|b| b.commands.len()).max().unwrap();
	for t in 0..t_max {
		for b in &bots {
			if b.commands.len() <= t {
				commands.push(Command::Wait);
			} else {
				commands.push(b.commands[t]);
			}
		}
	}
	if FISSION {
		commands.extend(postproc::fusion_all(&target, bots.iter().map(|b| b.p).collect()));
	}
	(energy, commands)
}

fn choose_z0(target: &V3<bool>, nbots: usize) -> (usize, usize) {
	let r = target.len();
	let mut total = 0;
	for x in 0..r {
		for y in 0..r {
			for z in 0..r {
				if target[x][y][z] {
					total += 1;
				}
			}
		}
	}
	let mut sub = 0;
	for z in 0..r {
		let tmp = sub;
		for x in 0..r {
			for y in 0..r {
				if target[x][y][z] {
					sub += 1;
				}
			}
		}
		if sub * 2 > total {
			return (z, nbots - nbots * tmp / total);
			// return (z, nbots / 2);
		}
	}
	return (r - 1, 0);
}

fn solve_z(target: &V3<bool>, nbots: usize, z0: Option<usize>, nbots1: Option<usize>) -> (i64, Vec<Command>) {
	let r = target.len();
	let (z0, nbots1) = if let (Some(z0), Some(nbots1)) = (z0, nbots1) {
		(z0, nbots1)
	} else {
		choose_z0(target, nbots)
	};
	eprintln!("z0: {} / {}, {} : {}", z0, r, nbots1, nbots - nbots1);
	let target2 = target_z(target, z0);
	let mut init_all = vec![];
	let mut energy = 0;
	for &dir in &[-1, 1] {
		let mut init = vec![];
		let nbots = if dir < 0 {
			nbots1
		} else {
			nbots - nbots1
		};
		let z = if dir < 0 { z0 } else { z0 - 1 };
		for x in 0..r {
			for y in 0..r {
				if target2[x][y][z] {
					init.push(P::new(x as i32, y as i32, z as i32 - dir));
				}
			}
		}
		if init.len() >= nbots {
			let mut init2 = vec![];
			for i in 0..nbots {
				init2.push(init[i * init.len() / nbots]);
			}
			init = init2;
		} else {
			for x in 0..r {
				for y in 0..r {
					if init.len() < nbots && !target2[x][y][z] {
						init.push(P::new(x as i32, y as i32, z as i32 - dir));
					}
				}
			}
		}
		init_all.extend(init);
	}
	let mut filled = mat![false; r; r; r];
	let (bids, mut commands) = if FISSION {
		fission_to(&filled, &init_all)
	} else {
		((0..nbots).collect(), vec![])
	};
	let mut bots_all = vec![];
	for &dir in &[-1, 1] {
		let mut bots = vec![];
		if dir < 0 {
			for i in 0..nbots1 {
				bots.push(Bot { bid: bids[i], p: init_all[i], commands: vec![] });
			}
		} else {
			for i in nbots1 .. nbots {
				bots.push(Bot { bid: bids[i], p: init_all[i], commands: vec![] });
			}
		}
		bots.sort_by_key(|b| b.bid);
		let mut bids = vec![];
		for i in 0..bots.len() {
			bids.push(bots[i].bid);
			bots[i].bid = i;
		}
		let mut occupied = InitV3::new(false, r);
		let mut max_d = 0;
		for d in 0.. {
			let z = if dir < 0 {
				z0 as i32 - dir * d
			} else {
				z0 as i32 - 1 - dir * d
			};
			if z < 0 || z >= r as i32 {
				max_d = d;
				break;
			}
			let mut ok = false;
			for x in 0..r {
				for y in 0..r {
					if target2[x][y][z as usize] {
						ok = true;
					}
				}
			}
			if !ok {
				max_d = d;
				break;
			}
		}
		for d in 0..max_d {
			let z = if dir < 0 {
				z0 as i32 - dir * d
			} else {
				z0 as i32 - 1 - dir * d
			};
			energy += fill_layer_z(&target2, &mut filled, &mut occupied, &mut bots, z, dir);
		}
		for i in 0..bots.len() {
			bots[i].bid = bids[i];
		}
		bots_all.extend(bots);
	}
	bots_all.sort_by_key(|b| b.bid);
	for i in 0..nbots {
		bots_all[i].bid = i;
	}
	let t_max = bots_all.iter().map(|b| b.commands.len()).max().unwrap();
	for b in bots_all.iter_mut() {
		while b.commands.len() < t_max {
			b.commands.push(Command::Wait);
		}
	}
	energy += destruct_support(&target, &mut filled, &mut bots_all);
	let t_max = bots_all.iter().map(|b| b.commands.len()).max().unwrap();
	for t in 0..t_max {
		for b in &bots_all {
			if b.commands.len() <= t {
				commands.push(Command::Wait);
			} else {
				commands.push(b.commands[t]);
			}
		}
	}
	if FISSION {
		commands.extend(postproc::fusion_all(&target, bots_all.iter().map(|b| b.p).collect()));
	}
	(energy, commands)
}

fn solve(target: &V3<bool>, nbots: usize, dir: &str, z0: Option<usize>, nbots1: Option<usize>) -> (i64, Vec<Command>) {
	match dir {
		"y" => {
			solve_bottom_up(target, nbots)
		},
		"z" => {
			solve_z(target, nbots, z0, nbots1)
		},
		"x" => {
			let r = target.len();
			let mut target2 = mat![false; r; r; r];
			for x in 0..r {
				for y in 0..r {
					for z in 0..r {
						target2[x][y][z] = target[z][y][x];
					}
				}
			}
			let (score, mut commands) = solve_z(&target2, nbots, z0, nbots1);
			let f = |p: P| P::new(p.z, p.y, p.x);
			for c in &mut commands {
				*c = match *c {
					Command::SMove(p) => Command::SMove(f(p)),
					Command::LMove(p1, p2) => Command::LMove(f(p1), f(p2)),
					Command::FusionP(p) => Command::FusionP(f(p)),
					Command::FusionS(p) => Command::FusionS(f(p)),
					Command::Fission(p, m) =>Command::Fission(f(p), m),
					Command::Fill(p) => Command::Fill(f(p)),
					Command::Void(p) => Command::Void(f(p)),
					Command::GFill(p1, p2) => Command::GFill(f(p1), f(p2)),
					Command::GVoid(p1, p2) => Command::GVoid(f(p1), f(p2)),
					c => c
				}
			}
			(score, commands)
		},
		_ => {
			panic!("unknown dir: {}", dir);
		}
	}
}

fn main() {
	assert!(std::env::args().nth(1).unwrap().trim().len() > 0 && std::env::args().nth(2).unwrap().len() <= 1);
	let file = std::env::args().nth(1).unwrap();
	let model = wata::read(&file);
	let target = model.filled;
	let mut min_score = i64::max_value();
	let mut min_commands = vec![];
	let nbots_list = if let Some(s) = std::env::args().nth(3) {
		vec![s.parse().unwrap()]
	} else {
		vec![40]
	};
	let dir = if let Some(s) = std::env::args().nth(4) {
		s
	} else {
		"y".to_owned()
	};
	let z0 = if let Some(s) = std::env::args().nth(5) {
		Some(s.parse().unwrap())
	} else {
		None
	};
	let nbots1 = if let Some(s) = std::env::args().nth(6) {
		Some(s.parse().unwrap())
	} else {
		None
	};
	for &nbots in &nbots_list {
		let (score, commands) = solve(&target, nbots, &dir, z0, nbots1);
		if min_score.setmin(score) {
			min_commands = commands;
		}
	}
	for command in min_commands {
		println!("{}", command.to_string());
	}
}
