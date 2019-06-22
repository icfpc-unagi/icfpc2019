use crate::{Booster, Square};

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

    (0..ts.len() / 2)
        .map(|i| parse_point_tokens(&ts[i * 2], &ts[i * 2 + 1]))
        .collect()
}

fn parse_point(s: &str) -> (usize, usize) {
    let ts: Vec<_> = s.split(',').collect();
    parse_point_tokens(&ts[0], &ts[1])
}

fn parse_obstacles(s: &str) -> Vec<Vec<(usize, usize)>> {
    s.split(';').map(|t| parse_map(t)).collect()
}

fn parse_boosters(s: &str) -> Vec<(Booster, usize, usize)> {
    if s == "" {
        vec![]
    } else {
        s.split(';')
            .map(|t| {
                let p = parse_point(&t[1..]);
                (t[..1].parse::<Booster>().unwrap(), p.0, p.1)
            })
            .collect()
    }
}

fn parse_task_specification(task: &str) -> TaskSpecification {
    let ss: Vec<_> = task.split('#').collect();
    eprintln!("task: {:?}", ss);

    TaskSpecification {
        frame: parse_map(ss[0]),
        initial_location: parse_point(ss[1]),
        obstacles: parse_obstacles(ss[2]),
        boosters: parse_boosters(ss[3]),
    }
}

////////////////////////////////////////////////////////////////////////////////
// Rasterize
////////////////////////////////////////////////////////////////////////////////

pub type SquareMap = Vec<Vec<Square>>;
pub type BoosterMap = Vec<Vec<Option<Booster>>>;

pub fn get_xysize<T>(map: &Vec<Vec<T>>) -> (usize, usize) {
    return (map.len(), map[0].len());
}

pub type RasterizedTask = (SquareMap, BoosterMap, usize, usize);

fn get_size(task: &TaskSpecification) -> (usize, usize) {
    (
        task.frame.iter().map(|&p| p.0).max().unwrap() + 2,
        task.frame.iter().map(|&p| p.1).max().unwrap() + 2,
    )
}

fn draw_contour(accsum: &mut Vec<Vec<i32>>, contour: &Vec<(usize, usize)>) {
    for i in 0..contour.len() {
        let p1 = contour[i];
        let p2 = contour[(i + 1) % contour.len()];

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

    accsum
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| {
                    if c % 2 == 0 {
                        Square::Block
                    } else {
                        Square::Empty
                    }
                })
                .collect()
        })
        .collect()
}

pub fn print_task(task: &RasterizedTask) {
    let map = &task.0;
    let boosters = &task.1;
    let ixy = (task.2, task.3);
    let xsize = map.len();
    let ysize = map[0].len();

    for y in (0..ysize).rev() {
        eprint!("{:02}:", y);
        for x in 0..xsize {
            eprint!(
                "{}",
                if ixy == (x, y) {
                    'I'
                } else {
                    match map[x][y] {
                        Square::Empty => {
                            if let Some(b) = boosters[x][y] {
                                match b {
                                    Booster::Extension => 'B',
                                    Booster::Drill => 'L',
                                    Booster::Fast => 'F',
                                    Booster::Teleport => 'R',
                                    Booster::X => 'X',
                                }
                            } else {
                                ' '
                            }
                        }
                        Square::Block => '#',
                        Square::Filled => '.',
                    }
                }
            );
        }
        eprintln!();
    }
}

pub fn parse_task(task_specification: &str) -> RasterizedTask {
    let task = parse_task_specification(&task_specification);

    let (xsize, ysize) = get_size(&task);

    let mut accsum = vec![vec![0; ysize]; xsize];
    draw_contour(&mut accsum, &task.frame);

    for obstacle in task.obstacles {
        draw_contour(&mut accsum, &obstacle);
    }

    let squares = accsum_to_squares(&mut accsum);

    let mut boosters = vec![vec![None; ysize]; xsize];
    for b in task.boosters {
        let (x, y) = (b.1 + 1, b.2 + 1);
        assert_eq!(boosters[x][y], None);
        boosters[x][y] = Some(b.0);
    }

    let ret = (
        squares,
        boosters,
        task.initial_location.0 + 1,
        task.initial_location.1 + 1,
    );

    print_task(&ret);
    ret
}

pub fn read_task(path: &str) -> RasterizedTask {
    let s = std::fs::read_to_string(path).expect("cannot read task file");
    parse_task(&s.trim_end())
}

////////////////////////////////////////////////////////////////////////////////
// テスト用
////////////////////////////////////////////////////////////////////////////////

pub fn load_task_001() -> RasterizedTask {
    parse_task("(0,0),(6,0),(6,1),(8,1),(8,2),(6,2),(6,3),(0,3)#(0,0)##")
}

pub fn load_task_002() -> RasterizedTask {
    parse_task("(30,24),(32,24),(32,21),(33,21),(33,19),(34,19),(34,18),(31,18),(31,22),(28,22),(28,18),(29,18),(29,17),(27,17),(27,23),(26,23),(26,43),(14,43),(14,23),(16,23),(16,21),(6,21),(6,31),(0,31),(0,21),(2,21),(2,0),(16,0),(16,11),(20,11),(20,23),(21,23),(21,17),(24,17),(24,14),(29,14),(29,15),(34,15),(34,16),(36,16),(36,21),(35,21),(35,24),(34,24),(34,25),(36,25),(36,22),(39,22),(39,23),(42,23),(42,26),(39,26),(39,25),(38,25),(38,31),(36,31),(36,34),(33,34),(33,31),(34,31),(34,28),(30,28)#(0,21)#(6,10),(8,10),(8,1),(11,1),(11,10),(12,10),(12,14),(15,14),(15,15),(12,15),(12,16),(10,16),(10,17),(9,17),(9,16),(6,16),(6,14),(5,14),(5,15),(4,15),(4,14),(3,14),(3,13),(6,13),(6,12),(3,12),(3,11),(6,11);(18,26),(19,26),(19,24),(21,24),(21,26),(24,26),(24,29),(25,29),(25,30),(24,30),(24,32),(22,32),(22,33),(25,33),(25,34),(24,34),(24,40),(23,40),(23,34),(22,34),(22,35),(21,35),(21,38),(20,38),(20,37),(16,37),(16,36),(20,36),(20,35),(19,35),(19,32),(18,32),(18,30),(17,30),(17,29),(18,29),(18,28),(15,28),(15,27),(16,27),(16,24),(17,24),(17,27),(18,27)#X(9,19);L(17,13);F(5,6);F(13,19);F(14,4);B(16,18);B(5,8)")
}

pub fn load_example_01() -> RasterizedTask {
    parse_task("(0,0),(10,0),(10,10),(0,10)#(0,0)#(4,2),(6,2),(6,7),(4,7);(5,8),(6,8),(6,9),(5,9)#B(0,1);B(1,1);F(0,2);F(1,2);L(0,3);X(0,9)")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        load_task_001();
        load_task_002();
        load_example_01();
    }
}
