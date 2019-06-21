#[derive(Copy, Debug, Clone)]
pub enum Square {
    Empty,
    Block,
    Filled,
}

#[derive(Copy, Debug, Clone)]
pub enum Booster {
    Extension,
    Fast,
    Drill,
    X,
}

/*
#[derive(Copy, Debug, Clone)]
pub struct PlayerState {
    x: usize,  //・今いる座標
    y: usize,
    dir: usize,  //・向いている向き
    unused_boosters: Vec<Square>,  //・持っている
    active_boosters: Vec<(Square, i32)>,  //・発動中の効果、残りターン
}

fn parse_map(map: &str) -> Vec<(usize, usize)> {
    let tokens: Vec<_> = map.split(',').collect();

    for i in 0..tokens.len() / 2 {
        let x = tokens[i * 2][1..];
        let y = tokens[i * 2 + 1][..-1];
        println!("{} {}", x, y);
    }

    unimplemented!();
}

fn parse_task(task: &str) -> (
    Vec<(usize, usize)>,
    (usize, usize),
    Vec<Vec<(usize, usize)>>,
    Vec<(Square, usize, usize)>,
) {
    let ss: Vec<_> = task.split('#').collect();
    println!("task: {:?}", ss);

    (
        parse_map(ss[0]),
        (0, 0),
        vec![],
        vec![],
    )
}
*/

pub fn read_task(path: &str) -> (Vec<Vec<Square>>, Vec<Vec<Option<Booster>>>, usize, usize) {
    /*
    let s = std::fs::read_to_string(path).unwrap();
    println!("{}", s);

    task = parse_task(&s);
    println!("{:?}", task);

    unimplemented!();
    */

    let (h, w) = (10, 10);

    let mut f = vec![vec![Square::Empty; w]; h];
    for x in 0..w {
        f[0][x] = Square::Block;
        f[h - 1][x] = Square::Block
    }
    for y in 0..h {
        f[y][0] = Square::Block;
        f[y][w - 1] = Square::Block;
    }
    return (
        f,
        vec![vec![None; w]; h],
        1,
        1
    );
}

#[cfg(test)]
mod tests {
    use super::read_task;

    #[test]
    fn it_works() {
        // assert_eq!(2 + 2, 4);
        //read_map()
    }
}

fn main() {
    let t = read_task("/Users/akiba/Downloads/part-1-initial/prob-001.desc");
    println!("{:?}", t);
}