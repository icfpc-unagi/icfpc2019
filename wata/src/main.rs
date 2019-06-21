use common::*;

fn nearest(map: &Vec<Vec<Square>>, (cx, cy): (usize, usize)) -> usize {
    if map[cx][cy] == Square::Empty {
        return 0
    }
    let n = map.len();
    let m = map[0].len();
    let mut ds = mat![!0; n; m];
    let mut que = std::collections::VecDeque::new();
    que.push_back((cx, cy));
    ds[cx][cy] = 0;
    while let Some(p) = que.pop_front() {
        let d = ds[p.0][p.1];
        for dir in 0..4 {
            let (x, y) = apply_move(p, dir);
            if map[x][y] == Square::Empty {
                return d + 1;
            } else if map[x][y] == Square::Filled && ds[x][y] == !0 {
                ds[x][y] = d + 1;
                que.push_back((x, y));
            }
        }
    }
    panic!("no empty squares")
}

fn fill_manipulators(map: &mut Vec<Vec<Square>>, (x, y): (usize, usize), manipulator: &[(i32, i32)]) -> usize {
    let mut count = 0;
    if map[x][y] == Square::Empty {
        map[x][y] = Square::Filled;
        count += 1;
    }
    for &dy in &[-1, 0, 1] {
        let x = x + 1;
        let y = (y as i32 + dy) as usize;
        if map[x][y] == Square::Empty {
            map[x][y] = Square::Filled;
            count += 1;
        }
    }
    count
}

fn greedy(map: &Vec<Vec<Square>>, _boosters: &Vec<Vec<Option<Booster>>>, (sx, sy): (usize, usize)) -> Vec<Action> {
    let mut map = map.clone();
    let n = map.len();
    let m = map[0].len();
    let manipulator = vec![(1, -1), (1, 0), (1, 1)];
    fill_manipulators(&mut map, (sx, sy), &manipulator);
    let mut num_empty = 0;
    for i in 0..n {
        for j in 0..m {
            if map[i][j] == Square::Empty {
                num_empty += 1;
            }
        }
    }
    let mut p = (sx, sy);
    let mut actions = vec![];
    while num_empty > 0 {
        let (_, d) = (0..4).map(|d| {
            let q = apply_move(p, d);
            if map[q.0][q.1] == Square::Block {
                (!0, d)
            } else {
                let near = nearest(&map, q);
                (near, d)
            }
        }).min().unwrap();
        actions.push(Action::Move(d));
        p = apply_move(p, d);
        num_empty -= fill_manipulators(&mut map, p, &manipulator);
    }
    actions
}

fn main() {
    let (map, boosters, sx, sy) = read_task(&std::env::args().nth(1).unwrap());
    let moves = greedy(&map, &boosters, (sx, sy));
    let moves = actions_to_string(&moves);
    eprintln!("turns: {}", moves.len());
    println!("{}", moves);
}
