use crate::*;

pub fn bootstrap_clone(task: &RasterizedTask, k: usize, buy_clone: usize) -> Vec<((usize, usize), usize, Vec<Action>)> {
	let &(ref map, ref boosters, sx, sy) = task;
	if k + buy_clone == 0 {
		return vec![((sx, sy), 0, vec![])];
	}
	let n = map.len();
	let m = map[0].len();
	let mut clones = vec![];
	let mut xs = vec![];
	for i in 0..n {
		for j in 0..m {
			if boosters[i][j] == Some(Booster::CloneWorker) {
				clones.push((i, j));
			} else if boosters[i][j] == Some(Booster::X) {
				xs.push((i, j));
			}
		}
	}
	let mut ret = vec![];
	let mut opt = !0;
	let mut bfs = BFS::new(n, m);
	for (xi, xj) in xs {
		if buy_clone > 0 {
			let mut acts = vec![bfs.search_fewest_actions_to_move(map, &PlayerState::new(sx, sy), xi, xj)];
			let mut ts = vec![acts[0].len()];
			let mut used = vec![false; clones.len()];
			// for _ in 0..buy_clone {
			// 	acts[0].extend();
			// }
		} else {
			for first_p in 0..clones.len() {
				let mut used = vec![false; clones.len()];
				
				let mut acts = vec![bfs.search_fewest_actions_to_move(map, &PlayerState::new(sx, sy), clones[first_p].0, clones[first_p].1)];
				acts[0].extend(bfs.search_fewest_actions_to_move(map, &PlayerState::new(clones[first_p].0, clones[first_p].1), xi, xj));
				used[first_p] = true;
				
				let mut ps = vec![(xi, xj), (xi, xj)];
				acts[0].push(Action::CloneWorker);
				acts.push(vec![]);
				let mut ts = vec![acts[0].len(), acts[0].len()];
				for _ in 0..k-1 {
					let mut min_jp = (!0, !0, vec![]);
					let mut min_t = !0;
					for j in 1..ts.len() {
						for p in 0..clones.len() {
							if !used[p] {
								let act = bfs.search_fewest_actions_to_move(map, &PlayerState::new(ps[j].0, ps[j].1), clones[p].0, clones[p].1);
								if min_t.setmin(ts[j] + act.len()) {
									min_jp = (j, p, act);
								}
							}
						}
					}
					let (j, p, act) = min_jp;
					ts[j] += act.len();
					used[p] = true;
					ps[j] = clones[p];
					acts[j].extend(act);
					while acts[0].len() <= ts[j] { // why <= ?
						acts[0].push(Action::Nothing);
					}
					acts[0].push(Action::CloneWorker);
					ts[0] = acts[0].len();
					acts.push(vec![]);
					ps.push(ps[0]);
					ts.push(ts[0]);
				}
				if opt.setmin(acts[0].len()) {
					ret = (0..ps.len()).map(|i| (ps[i], ts[i], acts[i].clone())).collect();
				}
			}
		}
	}
	ret
}
