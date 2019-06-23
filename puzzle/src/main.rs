use std::fs::File;
use std::io::prelude::*;

use rand::Rng;

use common::{parse_map, apply_move};
use common::task2::*;

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
enum Cell {
    Out,
    Unk,
    In,
}

use Cell::*;

fn main() -> std::io::Result<()> {
    let mut rng = rand::thread_rng(); // デフォルトの乱数生成器を初期化します
    // println!("Hello, world!");
    let ipath = std::env::args().nth(1).expect("usage: args[1] = condfile(input)");
    let pinput = puzzle::read(&ipath).expect("Unable to read data");
    let opath = std::env::args().nth(2).expect("usage: args[2] = descfile(output)");
    let puzzle::PazzleInput {tsize, vmin, vmax, isqs, osqs, ..} = pinput.clone();
    // dbg!(&osqs);
    let n = tsize + 2;
    let mut bool_map = vec![vec![false; n]; n];
    // let map = gen_polygon(tsize, &isqs, &osqs);
    loop { // repeat until success
        let mut map = vec![vec![Unk; n]; n];
        {
            // generate a polygon
            for i in 0..n {
                map[0][i] = Out;
                map[n-1][i] = Out;
                map[i][0] = Out;
                map[i][n-1] = Out;
            }
            for (x, y) in &isqs {
                map[x+1][y+1] = In;
            }
            /*
            for (x, y) in &osqs {
                map[x+1][y+1] = Out;
            }
            */
            let mut osqs_shuffled = osqs.clone();
            rng.shuffle(&mut osqs_shuffled);
            for (x, y) in &osqs_shuffled {
                let x = *x+1;
                let y = *y+1;
                let mut bfs = BFS::new(n, n);
                let (path, goalx, goaly) = bfs.search(
                    x, y,
                    |qx, qy| { map[qx][qy] == Out },
                    |qx, qy| { map[qx][qy] == In }
                    );
                for (px, py) in &path {
                    let px = *px;
                    let py = *py;
                    assert!(map[px][py] != In);
                    map[px][py] = Out;
                }
                dbg!(path);
            }
        }
        {
            // vertex wo fuyasu
            let mut n_vertex = 0;
            for x in 0..(n-1) {
                for y in 0..(n-1) {
                    if is_corner(&map, x, y) {
                        n_vertex += 1;
                    }
                }
            }
            assert!(n_vertex <= vmax);
            while n_vertex < vmin {
                dbg!((n_vertex, vmin));
                let x: usize = rng.gen::<usize>() % (n-2) + 1;
                let y: usize = rng.gen::<usize>() % (n-2) + 1;
                if map[x][y] != Unk {
                    continue;
                }
                let mut cnt = 0;
                for d in 0..4 {
                    let (tx, ty) = apply_move((x, y), d);
                    if map[tx][ty] == Out {
                        cnt += 1;
                        // こういうのも駄目なので除外
                        // ?.#
                        // #..
                        // ?.?
                        let (sx, sy) = apply_move(apply_move((x, y), (d+2)%4), (d+1)%4);
                        if map[sx][sy] == Out { cnt += 10; }
                        let (sx, sy) = apply_move(apply_move((x, y), (d+2)%4), (d+3)%4);
                        if map[sx][sy] == Out { cnt += 10; }
                    }
                }
                if cnt != 1 {
                    continue;
                }
                for dx in 0..2 { for dy in 0..2 {
                    if is_corner(&map, x-dx, y-dy) {
                        n_vertex -= 1;
                    }
                }}
                map[x][y] = Out;
                for dx in 0..2 { for dy in 0..2 {
                    if is_corner(&map, x-dx, y-dy) {
                        n_vertex += 1;
                    }
                }}
                // todo(tos)
            }
        }
        for x in 0..n {
            for y in 0..n {
                bool_map[x][y] = (map[x][y] != Out);
            }
        }

        for x in 0..n {
            for y in 0..n {
                eprint!("{}", if bool_map[x][y] { '.' } else { '#' });
            }
            eprintln!();
        }
        if puzzle::check(&pinput, &bool_map) {
            break;
        }
        eprintln!("check failed! retrying...");
    }
    let taskspec = raster_map_to_task_specification(
        &bool_map,
        pinput.mnum,
        pinput.fnum,
        pinput.dnum,
        pinput.rnum,
        pinput.cnum,
        pinput.xnum,
        );
    // print!("{}", taskspec);
    let mut f = File::create(opath).expect("Unable to create file");
    f.write_all(taskspec.as_bytes()).expect("Unable to write data");
    Ok(())
}

fn is_corner(map: &Vec<Vec<Cell>>, x: usize, y: usize) -> bool {
    let mut cnt = 0;
    for dx in 0..2 {
        for dy in 0..2 {
            if map[x+dx][y+dy] == Out {
                cnt += 1;
            }
        }
    }
    cnt % 2 == 1
}

// fn gen_polygon(tsize: usize, isqs:)


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BFS {
    xsize: usize,
    ysize: usize,
    que_vec: Vec<(usize, usize)>,
    que_head: usize,
    pot: Vec<Vec<(usize, usize)>>,
    goals: Vec<(usize, usize)>,
    is_goal: Vec<Vec<(usize)>>, // ここにこの向きで来ればゴール。ここではplayer_stateのdirは無視し、現在との相対的な方向を書く。
}

impl BFS {
    pub fn new(xsize: usize, ysize: usize) -> BFS {
        BFS {
            xsize,
            ysize,
            que_vec: vec![],
            que_head: 0,
            pot: vec![vec![(!0, !0); ysize]; xsize],
            goals: vec![],
            is_goal: vec![vec![!0; ysize]; xsize],
        }
    }

    pub fn search<F: Fn(usize, usize) -> bool, G: Fn(usize, usize) -> bool>(
        &mut self,
        x0: usize,
        y0: usize,
        goal_func: F,
        block_func: G,
    ) -> (Vec<(usize, usize)>, usize, usize) {
        self.que_vec.push((x0, y0));
        self.pot[x0][y0].0 = 0;

        let mut x = !0;
        let mut y = !0;

        while self.que_head < self.que_vec.len() {
            x = self.que_vec[self.que_head].0;
            y = self.que_vec[self.que_head].1;
            self.que_head += 1;

            if goal_func(x, y) {
                // eprintln!("{}", self.flg[x][y].0);
                break;
            }

            let c = self.pot[x][y].0;
            for d in 0..4 {
                let (tx, ty) = apply_move((x, y), d);

                if block_func(x, y) || self.pot[tx][ty].0 != !0 {
                    continue;
                }

                self.pot[tx][ty] = (c + 1, d);
                self.que_vec.push((tx, ty));
            }
        }

        (self.construct_path(x, y), x, y)
    }

    fn construct_path(&mut self, mut x: usize, mut y: usize) -> Vec<(usize, usize)> {
        let mut path = vec![];

        // self.is_goal[x][y]

        loop {
            let (c, d) = self.pot[x][y];
            if c == 0 {
                break;
            }

            let (tx, ty) = apply_move((x, y), (d + 2) % 4);
            x = tx;
            y = ty;
            path.push((x, y));
        }
        path.reverse();
        path
    }
}
