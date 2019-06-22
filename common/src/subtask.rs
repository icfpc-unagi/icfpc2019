use crate::*;

pub fn sub<T: Copy>(
    original: &Vec<Vec<T>>,
    default: T,
    flag: &Vec<Vec<bool>>,
    xmin: usize,
    xmax: usize,
    ymin: usize,
    ymax: usize,
) -> Vec<Vec<T>> {
    let mut s = mat![default; (xmax - xmin) + 2; (ymax - ymin) + 2];
    for x in xmin..xmax {
        for y in ymin..ymax {
            if flag[x][y] {
                s[x - xmin + 1][y - ymin + 1] = original[x][y];
            }
        }
    }
    s
}

pub fn create_subtask(
    square_map: &SquareMap,
    booster_map: &BoosterMap,
    flag: &Vec<Vec<bool>>,
) -> (SquareMap, BoosterMap, (usize, usize)) {
    let (xsize, ysize) = get_xysize(square_map);

    let xite = (0..xsize).filter(|&x| (0..ysize).any(|y| flag[x][y]));
    let xmin = xite.clone().min().unwrap();
    let xmax = xite.clone().max().unwrap() + 1;
    drop(xite);

    let yite = (0..ysize).filter(|&y| (0..xsize).any(|x| flag[x][y]));
    let ymin = yite.clone().min().unwrap();
    let ymax = yite.clone().max().unwrap() + 1;
    drop(yite);

    (
        sub(square_map, Square::Block, flag, xmin, xmax, ymin, ymax),
        sub(booster_map, None, flag, xmin, xmax, ymin, ymax),
        (xmin - 1, ymin - 1)  // 枠があるので-1
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let task = load_task_002();
        let (xsize, ysize) = get_xysize(&task.0);

        let mut flag = vec![vec![false; ysize]; xsize];

        for x in 2..20 {
            for y in 2..14 {
                flag[x][y] = ((x + y) % 2 == 0);
            }
        }

        let (s, b, xy) = create_subtask(&task.0, &task.1, &flag);

        dbg!(xy);
        print_task(&(s, b, 1, 1));
    }
}
