use common::*;

#[derive(Clone, Debug)]
pub struct PazzleInput {
	pub tsize: usize,
	pub vmin: usize,
	pub vmax: usize,
	pub mnum: usize,
	pub fnum: usize,
	pub dnum: usize,
	pub rnum: usize,
	pub cnum: usize,
	pub xnum: usize,
	pub isqs: Vec<(usize, usize)>,
	pub osqs: Vec<(usize, usize)>,
}

pub fn read(path: &str) -> std::io::Result<PazzleInput> {
    let s = std::fs::read_to_string(path).expect("cannot read cond file");
    let ss: Vec<_> = s.split('#').collect();
    assert_eq!(ss.len(), 3);
    let num: Vec<_> = ss[0].split(',').map(|n| n.parse::<usize>().unwrap()).collect();
	Ok(PazzleInput {
		tsize: num[2],
		vmin: num[3],
		vmax: num[4],
		mnum: num[5],
		fnum: num[6],
		dnum: num[7],
		rnum: num[8],
		cnum: num[9],
		xnum: num[10],
		isqs: parse_map(&ss[1]),
		osqs: parse_map(&ss[2]),
	})
}

pub fn is_connected(output: &Vec<Vec<bool>>, b: bool) -> bool {
	let n = output.len();
	let m = output[0].len();
	let mut visited = mat![false; n; m];
	let mut first = true;
	for si in 0..n {
		for sj in 0..m {
			if output[si][sj] == b && !visited[si][sj] {
				if !first {
					return false;
				}
				first = false;
				let mut stack = vec![];
				stack.push((si, sj));
				visited[si][sj] = true;
				while let Some(p) = stack.pop() {
					for d in 0..4 {
						let (i, j) = apply_move(p, d);
						if i < n && j < m && output[i][j] == b && !visited[i][j] {
							visited[i][j] = true;
							stack.push((i, j));
						}
					}
				}
			}
		}
	}
	true
}

pub fn count_vertices(output: &Vec<Vec<bool>>) -> usize {
	let n = output.len();
	let m = output[0].len();
	let mut num = 0;
	for i in 0..n-1 {
		for j in 0..m-1 {
			let mut count = 0;
			for di in 0..2 {
				for dj in 0..2 {
					if output[i + di][j + dj] {
						count += 1;
					}
				}
			}
			if count == 1 || count == 3 {
				num += 1;
			} else if count == 2 {
				if output[i][j] && output[i + 1][j + 1] || output[i][j + 1] && output[i + 1][j] {
					return !0;
				}
			}
		}
	}
	return num;
}

pub fn check(input: &PazzleInput, output: &Vec<Vec<bool>>) -> bool {
	let n = output.len();
	let m = output[0].len();
	if !is_connected(output, true) {
		eprintln!("not connected");
		return false;
	}
	if !is_connected(output, false) {
		eprintln!("has obstacle");
		return false;
	}
	if input.tsize < n - 2 || input.tsize < m - 2 {
		eprintln!("too large: {} * {}", n - 2, m - 2);
		return false;
	}
	let mut min_x = n;
	let mut max_x = 0;
	let mut min_y = m;
	let mut max_y = 0;
	for i in 0..n {
		for j in 0..m {
			if output[i][j] {
				min_x.setmin(i);
				min_y.setmin(j);
				max_x.setmin(i);
				max_y.setmin(j);
			}
		}
	}
	if max_x - min_x < input.tsize - input.tsize / 10 && max_y - min_y < input.tsize - input.tsize / 10 {
		eprintln!("too small: ({}) * ({})", max_x - min_x, max_y - min_y);
		return false;
	}
	let mut area = 0;
	for i in 0..n {
		for j in 0..m {
			if output[i][j] {
				area += 1;
			}
		}
	}
	if area < (input.tsize * input.tsize + 4) / 5 {
		eprintln!("area is too small: {}", area);
		return false;
	}
	let v = count_vertices(output);
	if v < input.vmin || v > input.vmax {
		eprintln!("the number of vertices is out of range: {}", v);
	}
	true
}
