#![allow(unused)]
extern crate wata;

use wata::*;
use wata::bfs::*;
use std::collections::*;

#[derive(Clone, Debug)]
enum State {
	Free,
	Moving {
		to: P,
		commands: Vec<Command>
	},
	Filling {
		dir: P
	},
	Halt,
}

#[derive(Clone, Debug)]
struct Bot {
	bid: usize,
	/// current position
	p: P,
	state: State,
}

fn split() -> Vec<Bot> {
	for t in 0..19 {
		for _ in 0..t {
			println!("{}", Command::Wait.to_string());
		}
		println!("{}", Command::Fission(P::new(1, 0, 0), 18 - t as usize).to_string());
	}
	(0..20).map(|i| Bot { bid: i, p: P::new(i as i32, 0, 0), state: State::Free }).collect()
}

fn main() {
	let file = std::env::args().nth(1).unwrap();
	let model = wata::read(&file);
	let r = model.r;
	let target = model.filled;
	let mut filled = mat![false; r; r; r];
	let mut ground = mat![false; r; r; r];
	let mut reserved = mat![!0; r; r; r];
	let mut bots = split();
	let mut reserved_list = vec![BTreeSet::<P>::new(); bots.len()];
	for x in 0..r {
		for z in 0..r {
			if target[x][0][z] {
				ground[x][0][z] = true;
			}
		}
	}
	let mut bfs = bfs::BFS::new(r);
	let mut occupied = InitV3::new(false, r);
	let mut cache = InitV3::new(false, r);
	let mut ground2 = InitV3::new(false, r);
	loop {
		occupied.init();
		eprintln!("{:?}\n{:?}", bots[1], bots[11]);
		eprintln!("{:?}\n{:?}", reserved_list[1], reserved_list[11]);
		let mut halt = true;
		for b in &mut bots {
			occupied[b.p] = true;
			match b.state {
				State::Halt => {
					b.state = State::Free;
				},
				_ => {
					halt = false;
				}
			}
		}
		if halt {
			break;
		}
		let mut moves = vec![];
		// fill
		for b in &mut bots {
			match b.state {
				State::Filling { dir } => {
					let mut q = None;
					for p in b.p.near(r) {
						if p != b.p + dir && !filled[p] && ground[p] && reserved[p] == b.bid {
							q = Some(p);
							break;
						}
					}
					if let Some(q) = q {
						if occupied[q] {
							moves.push((b.bid, Command::Wait));
						} else {
							moves.push((b.bid, Command::Fill(q - b.p)));
							occupied[q] = true;
							reserved_list[b.bid].remove(&q);
						}
					} else if reserved_list[b.bid].is_empty() {
						b.state = State::Free;
					} else {
						if occupied[b.p + dir] {
							moves.push((b.bid, Command::Wait));
						} else {
							moves.push((b.bid, Command::SMove(dir)));
							occupied[b.p + dir] = true;
						}
					}
				},
				_ => {
				}
			}
		}
		// reserve
		for b in &mut bots {
			cache.init();
			match b.state {
				State::Free => {
					bfs.clear();
					if let Some(q) = bfs.bfs(
						|p| filled[p] || reserved[p] != !0 || (p != b.p && occupied[p]),
						&vec![b.p],
						|p| {
							if p.x % 3 != 1 || p.z % 3 != 1 || cache[p] {
								return false;
							}
							cache[p] = true;
							let mut ok = false;
							for q in p.near(r) {
								if !filled[q] && ground[q] && reserved[q] == !0 {
									ok = true;
									break;
								}
							}
							if !ok {
								return false;
							}
							if p.y > 0 {
								for q in (p + P::new(0, -1, 0)).near(r) {
									if !filled[q] && ground[q] && reserved[q] == !0 {
										ok = false;
										break;
									}
								}
							}
							if !ok {
								return false;
							}
							cache[p] = false;
							true
						}
					) {
						let commands = bfs.restore(q);
						b.state = State::Moving { to: q, commands };
						let mut p = q;
						let dir = P::new(0, 1, 0);
						ground2.init();
						while p.is_valid(r) {
							loop {
								let mut ok = false;
								for q in p.near(r) {
									if q != p + dir && !filled[q] && (ground[q] || ground2[q]) && reserved[q] == !0 {
										reserved[q] = b.bid;
										reserved_list[b.bid].insert(q);
										for w in q.adj(r) {
											if target[w] && !ground2[w] {
												ground2[w] = true;
												ok = true;
											}
										}
									}
								}
								if !ok {
									break;
								}
							}
							p += dir;
						}
					} else {
						b.state = State::Halt;
						moves.push((b.bid, Command::Wait));
					}
				},
				_ => {
				}
			}
		}
		// move
		for b in &mut bots {
			let mut orz = false;
			let mut finished = false;
			match b.state {
				State::Moving { to, ref mut commands } => {
					if commands.len() == 0 {
						orz = true;
					} else {
						let c = commands[0];
						let mut ok = true;
						match c {
							Command::SMove(d) => {
								let len = d.mlen();
								let d = d / len;
								for i in 1..=len {
									let p = b.p + d * i;
									if filled[p] {
										orz = true;
										break;
									}
									if occupied[p] {
										ok = false;
										break;
									}
								}
								if orz || !ok {
									moves.push((b.bid, Command::Wait));
								} else {
									moves.push((b.bid, Command::SMove(d * len)));
									commands.remove(0);
									if commands.len() == 0 {
										finished = true;
									}
									for i in 1..=len {
										let p = b.p + d * i;
										occupied[p] = true;
									}
								}
							},
							Command::LMove(d1, d2) => {
								let len1 = d1.mlen();
								let d1 = d1 / len1;
								let len2 = d2.mlen();
								let d2 = d2 / len2;
								for i in 1..=len1 {
									let p = b.p + d1 * i;
									if filled[p] {
										orz = true;
										break;
									}
									if occupied[p] {
										ok = false;
										break;
									}
								}
								if ok && !orz {
									for i in 1..=len2 {
										let p = b.p + d1 * len1 + d2 * i;
										if filled[p] {
											orz = true;
											break;
										}
										if occupied[p] {
											ok = false;
											break;
										}
									}
								}
								if orz || !ok {
									moves.push((b.bid, Command::Wait));
								} else {
									moves.push((b.bid, Command::LMove(d1 * len1, d2 * len2)));
									commands.remove(0);
									if commands.len() == 0 {
										finished = true;
									}
									for i in 1..=len1 {
										let p = b.p + d1 * i;
										occupied[p] = true;
									}
									for i in 1..=len2 {
										let p = b.p + d1 * len1 + d2 * i;
										occupied[p] = true;
									}
								}
							},
							_ => {
								unreachable!();
							}
						}
						if !ok {
							orz = true;
						}
					}
				},
				_ => {
				}
			}
			if orz {
				eprintln!("orz");
				b.state = State::Free;
				for r in reserved_list[b.bid].clone() {
					reserved[r] = !0;
				}
				reserved_list[b.bid].clear();
			} else if finished {
				b.state = State::Filling { dir: P::new(0, 1, 0) };
			}
		}
		// eprintln!("{:?}", moves);
		moves.sort();
		assert!(moves.len() == bots.len());
		let mut allwait = true;
		for (_, command) in &moves {
			if let Command::Wait = command {
			} else {
				allwait = false;
			}
		}
		if allwait {
			break;
		}
		for (bid, command) in moves {
			println!("{}", command.to_string());
			match command {
				Command::Fill(p) => {
					let p = bots[bid].p + p;
					filled[p] = true;
					for q in p.adj(r) {
						if target[q] {
							ground[q] = true;
						}
					}
				},
				Command::SMove(d) => {
					bots[bid].p += d;
				},
				Command::LMove(d1, d2) => {
					bots[bid].p += d1 + d2;
				}
				_ => {
				}
			}
		}
	}
	let mut rem = 0;
	let mut total = 0;
	let mut grounded = 0;
	for x in 0..r {
		for y in 0..r {
			for z in 0..r {
				if target[x][y][z] {
					total += 1;
				}
				if target[x][y][z] && !filled[x][y][z] {
					rem += 1;
					if ground[x][y][z] {
						grounded += 1;
					}
				}
			}
		}
	}
	eprintln!("remaining: {} ({}) / {}", rem, grounded, total);

    if rem == 0 {
        let mut positions = Vec::new();
        for bot in bots.iter() {
            positions.push(bot.p);
        }
        let cmds = postproc::fusion_all(&filled, positions);
        for cmd in cmds {
            println!("{}", cmd.to_string());
        }
    }
}
