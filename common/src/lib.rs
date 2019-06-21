#[derive(Copy, Debug, Clone)]
pub enum Square {
    Empty,
    Block,
    Extension,
    Fast,
    Drill,
    X,
}

pub fn read_map(path: &str) -> (Vec<Vec<Square>>, usize, usize) {
    /*
    let s = std::fs::read_to_string(path).unwrap();
    println!("{}", s);

    let ss: Vec<str> = s.split('#').collect();
    */
    /* unimplemented!();  */

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
    return (f, 1, 1);
}

#[cfg(test)]
mod tests {
    use super::read_map;

    #[test]
    fn it_works() {
        // assert_eq!(2 + 2, 4);
        //read_map()
    }
}

fn main() {
    let t = read_map("/Users/akiba/Downloads/part-1-initial/prob-001.desc");
    println!("{:?}", t);
}