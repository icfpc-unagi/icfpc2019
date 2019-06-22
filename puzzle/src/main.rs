// use std::fs::File;
// use std::io::prelude::*;

use common::{parse_map, apply_move};

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
enum Cell {
    Out,
    Unk,
    In,
}

use Cell::*;

fn main() -> std::io::Result<()> {
    // println!("Hello, world!");
    let path = std::env::args().nth(1).expect("usage: args[1] = condfile");
    /*
    let mut file = File::open(path);
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    */
    let s = std::fs::read_to_string(path).expect("cannot read cond file");
    let ss: Vec<_> = s.split('#').collect();
    assert_eq!(ss.len(), 3);
    let nums: Vec<_> = ss[0].split(',').map(|n| n.parse::<i32>().unwrap()).collect();
    let tsize = nums[2] as usize;
    let isqs = parse_map(&ss[1]);
    let osqs = parse_map(&ss[2]);
    dbg!(&nums);
    // dbg!(&osqs);
    let n = tsize + 2;
    let mut map = vec![vec![Unk; n]; n];
    // let map = gen_polygon(tsize, &isqs, &osqs);
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
        for (x, y) in &osqs {
            let x = x+&1;
            let y = y+1;
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
            break;
        }
    }
    Ok(())
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
