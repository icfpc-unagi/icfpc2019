use crate::*;

fn bootstrap_expand(task: &RasterizedTask) {
    let (square_map, booster_map, start_x, start_y) = task;
    let start = (*start_x, *start_y);
    let (xsize, ysize) = get_xysize(square_map);
    // unimplemented!();

    let mut targets = vec![];
    for x in 0..xsize {
        for y in 0..ysize {
            if booster_map[x][y] == Some(Booster::Extension) {
                targets.push((x, y));
            }
        }
    }

    tsp(square_map, start, &targets, |_, _| true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
