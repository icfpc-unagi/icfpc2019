use crate::*;

pub fn vectorize(raster: Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    // 外周は使われていないと仮定！
    let (xsize, ysize) = get_xysize(&raster);

    for x in 0..xsize - 1 {
        for y in 0..ysize - 1 {

        }
    }

    unimplemented!();
}
