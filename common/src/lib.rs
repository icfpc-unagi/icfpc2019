pub mod reach;

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum Square {
    Empty,
    Block,
    Filled,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum Booster {
    Extension,
    Fast,
    Drill,
    X,
}

pub fn apply_move((x, y): (usize, usize), dir: usize) -> (usize, usize) {
    match dir {
        0 => (x + 1, y),
        1 => (x, y - 1),
        2 => (x - 1, y),
        3 => (x, y + 1),
        _ => panic!("illegal dir: {}", dir)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Move(usize),
    Nothing,
    TurnR,
    TurnL,
    Extension(i32, i32),
    Fast,
    Drill
}

pub fn actions_to_string(list: &Vec<Action>) -> String {
    let mut out = String::new();
    for mv in list {
        match mv {
            Action::Move(dir) => out += ["D", "S", "A", "W"][*dir],
            Action::Nothing => out += "Z",
            Action::TurnR => out += "E",
            Action::TurnL => out += "Q",
            Action::Extension(dx, dy) => out += &format!("B({},{})", dx, dy),
            Action::Fast => out += "F",
            Action::Drill => out += "L"
        }
    }
    out
}

pub struct PlayerState {
    x: usize,  //・今いる座標
    y: usize,
    dir: usize,  //・向いている向き
    unused_boosters: Vec<Square>,  //・持っている
    active_boosters: Vec<(Square, i32)>,  //・発動中の効果、残りターン
}

////////////////////////////////////////////////////////////////////////////////
// Parse
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
struct TaskSpecification {
    frame: Vec<(usize, usize)>,
    initial_location: (usize, usize),
    obstacles: Vec<Vec<(usize, usize)>>,
    boosters: Vec<(Booster, usize, usize)>,
}

fn parse_point_tokens(x: &str, y: &str) -> (usize, usize) {
    let x = &x[1..];
    let y = &y[..y.len() - 1];
    (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
}

fn parse_map(s: &str) -> Vec<(usize, usize)> {
    let ts: Vec<_> = s.split(',').collect();

    (0..ts.len() / 2).map(|i| {
        parse_point_tokens(&ts[i *  2], &ts[i * 2 + 1])
    }).collect()
}

fn parse_point(s: &str) -> (usize, usize) {
    let ts: Vec<_> = s.split(',').collect();
    parse_point_tokens(&ts[0], &ts[1])
}

fn parse_task(task: &str) -> TaskSpecification {
    let ss: Vec<_> = task.split('#').collect();
    eprintln!("task: {:?}", ss);

    TaskSpecification {
        frame: parse_map(ss[0]),
        initial_location: parse_point(ss[1]),
        obstacles: vec![],
        boosters: vec![],
    }
}

////////////////////////////////////////////////////////////////////////////////
// Rasterize
////////////////////////////////////////////////////////////////////////////////

fn get_size(task: &TaskSpecification) -> (usize, usize) {
    (
        task.frame.iter().map(|&p| p.0).max().unwrap() + 2,
        task.frame.iter().map(|&p| p.1).max().unwrap() + 2,
    )
}

fn draw_contour(accsum: &mut Vec<Vec<i32>>, contour: &Vec<(usize, usize)>) {
    for i in 0..contour.len() {
        let p1 = contour[i];
        let p2 = contour[(i + 1) %  contour.len()];

        if p1.1 == p2.1 {
            let y = p1.1;
            let xmin = usize::min(p1.0, p2.0);
            let xmax = usize::max(p1.0, p2.0);
            for x in xmin..xmax {
                let x = x + 1;
                let y = y + 1;
                assert_eq!(accsum[x][y], 0);
                accsum[x][y] = 1;
            }
        }
    }
}

fn accsum_to_squares(accsum: &mut Vec<Vec<i32>>) -> Vec<Vec<Square>> {
    let xsize = accsum.len();
    let ysize = accsum[0].len();
    for x in 0..xsize {
        for y in 1..ysize {
            accsum[x][y] += accsum[x][y - 1];
        }
    }
    dbg!(&accsum);

    accsum.iter().map(|row| {
        row.iter().map(|c| {
            if c % 2 == 0 {
                Square::Block
            } else {
                Square::Empty
            }
        }).collect()
    }).collect()
}

fn debug_map(map: &Vec<Vec<Square>>) {
    let xsize = map.len();
    let ysize = map[0].len();

    for y in (0..ysize).rev() {
        eprint!("{:02}:", y);
        for x in 0..xsize {
            eprint!("{}", match map[x][y] {
                Square::Empty => ' ',
                Square::Block => '#',
                Square::Filled => '.',
            })
        }
        eprintln!();
    }
}

pub fn read_task(path: &str) -> (Vec<Vec<Square>>, Vec<Vec<Option<Booster>>>, usize, usize) {
    let s = std::fs::read_to_string(path).unwrap();
    let task = parse_task(&s);
    eprintln!("{:?}", task);

    let (xsize, ysize) = get_size(&task);
    eprintln!("{} {}", xsize, ysize);

    let mut accsum = vec![vec![0; ysize]; xsize];
    draw_contour(&mut accsum, &task.frame);
    eprintln!("{:?}", accsum);

    let squares = accsum_to_squares(&mut accsum);
    eprintln!("{:?}", squares);

    debug_map(&squares);

    (
        squares,
        vec![vec![None; ysize]; xsize],
        task.initial_location.0 + 1,
        task.initial_location.1 + 1,
    )

    /*
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
    */
}
