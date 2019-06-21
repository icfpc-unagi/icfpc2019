use crate::{Square, Booster};

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

fn parse_obstacles(s: &str) -> Vec<Vec<(usize, usize)>> {
    s.split(';').map(|t| parse_map(t)).collect()
}

fn parse_boosters(s: &str) -> Vec<(Booster, usize, usize)> {
    if s == "" {
        vec![]
    } else {
        s.split(';').map(|t| {
            let p = parse_point(&t[1..]);
            (t[..1].parse::<Booster>().unwrap(), p.0, p.1)
        }).collect()
    }
}

fn parse_task(task: &str) -> TaskSpecification {
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

type RasterizedTask = (Vec<Vec<Square>>, Vec<Vec<Option<Booster>>>, usize, usize);

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

fn print_task(task: &RasterizedTask) {
    let map = &task.0;
    let boosters = &task.1;
    let ixy = (task.2, task.3);
    let xsize = map.len();
    let ysize = map[0].len();

    for y in (0..ysize).rev() {
        eprint!("{:02}:", y);
        for x in 0..xsize {
            eprint!("{}",
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
                                        Booster::X => 'X',
                                    }
                                } else {
                                    ' '
                                }
                            },
                            Square::Block => '#',
                            Square::Filled => '.',
                        }
                    }
            );
        }
        eprintln!();
    }
}

pub fn read_task(path: &str) -> RasterizedTask {
    let s = std::fs::read_to_string(path).expect("cannot read task file");
    let task = parse_task(&s);

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
