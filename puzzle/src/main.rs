use std::fs::File;
use std::io::prelude::*;

use rand::Rng;
use rand::seq::SliceRandom;

use common::{parse_map, apply_move};
use common::task2::*;

#[derive(Copy, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Cell {
    Out,
    UOut,
    // Unk,  // deprecated
    UIn,
    In,
}


impl Cell {
    pub fn as_bool(self) -> bool {
        // assert!(self != Unk);
        self >= UIn
    }
    pub fn as_unk(b: bool) -> Self {
        if b { UIn } else { UOut }
    }
    pub fn is_unk(self) -> bool {
        self >= UOut && self <= UIn
    }
    pub fn as_char(self) -> char {
        match self {
            Out => '#',
            UOut => '*',
            UIn => '.',
            In => ' ',
        }
    }
}


use Cell::*;

fn main() -> std::io::Result<()> {
    // println!("Hello, world!");
    let ipath = std::env::args().nth(1).expect("usage: args[1] = condfile(input)");
    let pinput = puzzle::read(&ipath).expect("Unable to read data");
    let opath = std::env::args().nth(2).expect("usage: args[2] = descfile(output)");
    let bool_map = generate_raster_v2(&pinput);
    let bool_map = bool_map.or_else(
        || generate_raster_marine_day(&pinput));
    let bool_map = Some(bool_map.unwrap()); // debug!!
    let bool_map = bool_map.unwrap_or_else(
        || generate_raster_v1(&pinput));
    // let bool_map = generate_raster_v1(&pinput); // debug!!

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


fn generate_raster_marine_day(pinput: &puzzle::PazzleInput) -> Option<Vec<Vec<bool>>> {
    let mut rng = rand::thread_rng(); // デフォルトの乱数生成器を初期化します
    let puzzle::PazzleInput {tsize, vmin, vmax, isqs, osqs, ..} = pinput.clone();
    // dbg!(&osqs);
    if tsize % 5 != 0 {
        eprintln!("tsize % 5"); return None;
    }
    let m = tsize / 5;
    let n = tsize + 2;

    let mut map = vec![vec![UOut; n]; n];
    for i in 0..n {
        map[0][i] = Out;
        map[n-1][i] = Out;
        map[i][0] = Out;
        map[i][n-1] = Out;
    }
    for &(x, y) in &osqs {
        let x = x + 1;
        let y = y + 1;
        map[x][y] = Out;
    }
    for k in 0..m {
        let x = 5*k+3;
        if k != 0 {
            let mut ok = false;
            'h: for h in 0..n-2 {
                let y = if k % 2 == 0 {
                    1 + h
                } else {
                    n-2 - h
                };
                for dx in 1..=4 {
                    if map[x-dx][y] == Out {
                        continue 'h
                    }
                }
                for dx in 1..=4 {
                    map[x-dx][y] = In;
                }
                ok = true;
                break;
            }
            if !ok {
                eprintln!("horizontal conn"); return None;
            }
        }
        for y in 1..n-1 {
            if map[x][y] != Out {
                map[x][y] = In;
                continue;
            }
            if map[x-1][y-1] != Out && map[x-1][y] != Out && map[x-1][y+1] != Out {
                map[x-1][y-1] = In;
                map[x-1][y] = In;
                map[x-1][y+1] = In;
                continue;
            }
            if map[x+1][y-1] != Out && map[x+1][y] != Out && map[x+1][y+1] != Out {
                map[x+1][y-1] = In;
                map[x+1][y] = In;
                map[x+1][y+1] = In;
                continue;
            }
            eprintln!("cannot avoid out"); return None;
        }
    }
    for &(x, y) in &isqs {
        let mut x = x + 1;
        let y = y + 1;
        while map[x][y] != In {
            if map[x][y] == Out {
                eprintln!("in: equal x with out?"); return None;
            }
            map[x][y] = In;
            if x % 5 == 1 || x % 5 == 2 {
                x += 1;
            } else {
                x -= 1;
            }
        }
    }
    adjust_vnum(&mut map, vmin, vmax);

    let mut bool_map = vec![vec![false; n]; n];
    for x in 0..n {
        for y in 0..n {
            bool_map[x][y] = map[x][y].as_bool();
        }
    }
        for x in 0..n {
            for y in 0..n {
                eprint!("{}", if bool_map[x][y] { '.' } else { '#' });
            }
            eprintln!();
        }
        if puzzle::check(&pinput, &bool_map) {
            return Some(bool_map);
        }
        eprintln!("check failed!"); return None;
}

fn generate_raster_v2(pinput: &puzzle::PazzleInput) -> Option<Vec<Vec<bool>>> {
    let mut rng = rand::thread_rng(); // デフォルトの乱数生成器を初期化します
    let puzzle::PazzleInput {tsize, vmin, vmax, isqs, osqs, ..} = pinput.clone();
    // dbg!(&osqs);
    let n = tsize + 2;
    let mut map = vec![vec![UOut; n]; n];
    let img = [
        b"*****",
        b"*.*.*",
        b"*.*.*",
        b"*....",
        b"*****",
    ];
    for x in 0..tsize {
        for y in 0..tsize {
            let imgx = x*5/tsize;
            let imgy = y*5/tsize;
            if img[4-imgy][imgx] == b'.' {
                map[x][y] = UIn;
            }
        }
    }

    // generate a polygon
    for i in 0..n {
        map[0][i] = Out;
        map[n-1][i] = Out;
        map[i][0] = Out;
        map[i][n-1] = Out;
    }
    for &(x, y) in &isqs {
        map[x+1][y+1] = In;
    }
    /*
    for &(x, y) in &osqs {
        map[x+1][y+1] = Out;
    }
    */
    let mut osqs_shuffled = osqs.clone();
    osqs_shuffled.shuffle(&mut rng);
    for &(x, y) in &osqs_shuffled {
        let x = x+1;
        let y = y+1;
        let mut bfs = BFS::new(n, n);
        let (mut path, goalx, goaly) = bfs.search(
            x, y,
            |qx, qy| { !map[qx][qy].as_bool() },  // goal = out
            |qx, qy| { map[qx][qy] == In }  // block = isqs
        );
        path.push((goalx, goaly));
        for &(px, py) in &path {
            map[px][py] = Out;
        }
        assert_eq!(map[x][y], Out);
    }

    for &(x, y) in &isqs {
        let x = x+1;
        let y = y+1;
        assert!(map[x][y] != Out);
        map[x][y] = UOut;
    }
    let mut isqs_shuffled = isqs.clone();
    isqs_shuffled.shuffle(&mut rng);
    for &(x, y) in &isqs_shuffled {
        let x = x+1;
        let y = y+1;
        let mut bfs = BFS::new(n, n);
        let (mut path, goalx, goaly) = bfs.search(
            x, y,
            |qx, qy| { map[qx][qy].as_bool() },  // goal = in
            |qx, qy| { map[qx][qy] == Out }  // block = osqs
        );
        path.push((goalx, goaly));
        for &(px, py) in &path {
            map[px][py] = In;
        }
        assert_eq!(map[x][y], In);
    }

    adjust_vnum(&mut map, vmin, vmax);

    let mut bool_map = vec![vec![false; n]; n];
    for i in 0..n {
        for j in 0..n {
            eprint!("{}", map[j][n-1-i].as_char());
        }
        eprintln!();
    }
    for x in 0..n {
        for y in 0..n {
            bool_map[x][y] = map[x][y].as_bool();
        }
    }
    if puzzle::check(&pinput, &bool_map) {
        return Some(bool_map);
    }
    eprintln!("check failed!"); return None;
}

fn generate_raster_v1(pinput: &puzzle::PazzleInput) -> Vec<Vec<bool>> {
    let mut rng = rand::thread_rng(); // デフォルトの乱数生成器を初期化します
    let puzzle::PazzleInput {tsize, vmin, vmax, isqs, osqs, ..} = pinput.clone();
    // dbg!(&osqs);
    let n = tsize + 2;
    let mut bool_map = vec![vec![false; n]; n];
    // let map = gen_polygon(tsize, &isqs, &osqs);
    loop { // repeat until success
        let mut map = vec![vec![UIn; n]; n];
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
                    |qx, qy| { !map[qx][qy].as_bool() },  // goal = out
                    |qx, qy| { map[qx][qy] == In }  // block = isqs
                    );
                for (px, py) in &path {
                    let px = *px;
                    let py = *py;
                    assert!(map[px][py] != In);
                    map[px][py] = UOut;
                }
                dbg!(path);
            }
        }
        adjust_vnum(&mut map, vmin, vmax);
        for x in 0..n {
            for y in 0..n {
                bool_map[x][y] = map[x][y].as_bool();
            }
        }

        for x in 0..n {
            for y in 0..n {
                eprint!("{}", if bool_map[x][y] { '.' } else { '#' });
            }
            eprintln!();
        }
        if puzzle::check(&pinput, &bool_map) {
            return bool_map;
        }
        eprintln!("check failed! retrying...");
    }
}


fn adjust_vnum(map: &mut Vec<Vec<Cell>>, vmin: usize, vmax: usize) {
    let mut rng = rand::thread_rng();
    let n = map.len();
    assert_eq!(n, map[0].len());
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
    'search_v: while n_vertex < vmin {
        let x: usize = rng.gen::<usize>() % (n-2) + 1;
        let y: usize = rng.gen::<usize>() % (n-2) + 1;
        if !map[x][y].is_unk() {
            continue;
        }
        eprintln!("{} < {}", n_vertex, vmin);
        // dbg!((n_vertex, vmin));
        let orig = map[x][y].as_bool();
        let mut cnt = 0;
        for d in 0..4 {
            let (tx, ty) = apply_move((x, y), d);
            if map[tx][ty].as_bool() != orig {
                cnt += 1;
                // こういうのも駄目なので除外
                // ?.#
                // #..
                // ?.?
                let (sx, sy) = apply_move(apply_move((x, y), (d+2)%4), (d+1)%4);
                if map[sx][sy].as_bool() != orig {
                    continue 'search_v;
                }
                let (sx, sy) = apply_move(apply_move((x, y), (d+2)%4), (d+3)%4);
                if map[sx][sy].as_bool() != orig {
                    continue 'search_v;
                }
            }
        }
        if cnt != 1 {
            continue;
        }
        eprintln!("found ({}, {})", x, y);
        // dbg!((x, y));
        for dx in 0..2 { for dy in 0..2 {
            if is_corner(&map, x-dx, y-dy) {
                n_vertex -= 1;
            }
        }}
        map[x][y] = Cell::as_unk(!orig);
        for dx in 0..2 { for dy in 0..2 {
            if is_corner(&map, x-dx, y-dy) {
                n_vertex += 1;
            }
        }}
        // todo(tos)
    }
}

fn is_corner(map: &Vec<Vec<Cell>>, x: usize, y: usize) -> bool {
    let mut cnt = 0;
    for dx in 0..2 {
        for dy in 0..2 {
            if map[x+dx][y+dy].as_bool() {
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
