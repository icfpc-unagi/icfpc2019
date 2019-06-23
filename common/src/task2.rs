use crate::*;

pub fn boosters_to_string(boosters: &Vec<(Booster, usize, usize)>) -> String {
    let t: Vec<_> = boosters.iter().map(
        |(b, x, y)|
            format!("{}({},{})",
                match b {
                    Booster::Extension => "B",
                    Booster::Fast => "F",
                    Booster::Drill => "L",
                    Booster::Teleport => "R",
                    Booster::X => "X",
                    Booster::CloneWorker => "C",
                },
                x,
                y
            )
    ).collect();
    t.join(";")
}

pub fn raster_map_to_task_specification(
    raster_map: &Vec<Vec<bool>>,
    mnum: usize,
    fnum: usize,
    dnum: usize,
    rnum: usize,
    cnum: usize,
    xnum: usize,
) -> String {
    let (xsize, ysize) = get_xysize(&raster_map);

    // 外周
    let contour = vectorize(raster_map);

    // 空いてるセルにinitial positionとboosterを置いていく
    let mut empty_cells = (0..xsize)
        .map(|x| (0..ysize).map(move |y| (x, y)))
        .flatten()
        .filter(|(x, y)| raster_map[*x][*y]);

    let mut empty_cells = empty_cells.collect::<std::collections::VecDeque<_>>();

    let initial_position = empty_cells.pop_front().unwrap();
    dbg!(initial_position);

    let mut boosters = vec![];
    let mut doit = |num, booster| {
        for _ in 0..num {
            let (x, y) = empty_cells.pop_back().unwrap();
            boosters.push((booster, x, y));
        }
    };
    doit(mnum, Booster::Extension);
    doit(fnum, Booster::Fast);
    doit(dnum, Booster::Drill);
    doit(rnum, Booster::Teleport);
    doit(cnum, Booster::CloneWorker);
    doit(xnum, Booster::X);
    drop(doit);

    // ぜんぶ座標を-1する
    let contour = contour.iter().map(|(x, y)| (x - 1, y - 1)).collect();
    let initial_position = (initial_position.0 - 1, initial_position.1 - 1);
    let boosters = boosters.iter().map(|(b, x, y)| (*b, x - 1, y - 1)).collect();

    format!(
        "{}#({},{})##{}",
        contour_to_string(&contour),
        initial_position.0, initial_position.1,
        boosters_to_string(&boosters),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tasks = [
            load_task_001(),
        ];

        for task in tasks.iter() {
            let raster_map = to_bool_map(&task.0, Square::Empty);
            let s = raster_map_to_task_specification(&raster_map, 1, 1, 1, 1, 1, 1);
            eprintln!("{}", s);
        }
    }
}
