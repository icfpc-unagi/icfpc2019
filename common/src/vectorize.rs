use crate::*;

fn to_bool_map<T: Copy + std::cmp::PartialEq>(
    original: &Vec<Vec<T>>,
    true_value: T,
) -> Vec<Vec<bool>> {
    original
        .iter()
        .map(|row| row.iter().map(|&cell| cell == true_value).collect())
        .collect()
}

pub fn vectorize(raster: &Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    // 外周は使われていないと仮定！
    // 中身true、枠がfalse

    let (xsize, ysize) = get_xysize(&raster);

    // Construct graph
    let mut adj = mat![vec![]; xsize; ysize];
    for y in 1..ysize {
        let mut x0 = 0;
        while x0 < xsize {
            if raster[x0][y] != raster[x0][y - 1] {
                let mut x1 = x0 + 1;

                while x1 < xsize && raster[x1][y] != raster[x1][y - 1] {
                    x1 += 1;
                }
                // dbg!((y, x0, x1));

                adj[x0][y].push((x1, y));
                adj[x1][y].push((x0, y));
                x0 = x1;
            } else {
                x0 += 1;
            }
        }
    }
    for x in 1..xsize {
        let mut y0 = 0;
        while y0 < ysize {
            if raster[x][y0] != raster[x - 1][y0] {
                let mut y1 = y0 + 1;

                while y1 < ysize && raster[x][y1] != raster[x - 1][y1] {
                    y1 += 1;
                }
                //dbg!((x, y0, y1));

                adj[x][y0].push((x, y1));
                adj[x][y1].push((x, y0));
                y0 = y1;
            } else {
                y0 += 1;
            }
        }
    }

    // Traverse
    let start = (0..xsize)
        .map(|x| (0..ysize).map(move |y| (x, y)))
        .flatten()
        .filter(|(x, y)| raster[*x][*y])
        .min()
        .unwrap();
    assert_eq!(adj[start.0][start.1].len(), 2);

    let mut crr = start;
    let mut prv = (!0, !0);
    let mut contour = vec![];

    loop {
        contour.push(crr);

        let mut nxt = (!0, !0);
        assert_eq!(adj[crr.0][crr.1].len(), 2);

        for &a in adj[crr.0][crr.1].iter() {
            if a != prv {
                nxt = a;
                break;
            }
        }

        if nxt == start {
            break;
        }

        prv = crr;
        crr = nxt;
    }

    contour
}

pub fn contour_to_string(contour: &Vec<(usize, usize)>) -> String {
    let mut s = String::new();
    for p in contour.iter() {
        s += &format!("({},{}),", p.0, p.1);
    }
    s[..s.len() - 1].to_string()
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
            let raster = to_bool_map(&task.0, Square::Empty);
            let c = vectorize(&raster);
            eprintln!("{}", contour_to_string(&c));
            eprintln!("{:?}", c);

            let task2_description = format!(
                "{}#(0,0)##",
                contour_to_string(&(c.iter().map(|(x, y)| (x - 1, y - 1)).collect())));
            let task2 = parse_task(&task2_description);
            let raster2 = to_bool_map(&task2.0, Square::Empty);

            assert_eq!(raster, raster2);
        }
    }
}
