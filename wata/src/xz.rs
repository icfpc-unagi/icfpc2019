use *;

pub type V2<T> = Vec<Vec<T>>;

pub fn any_y(filled: &V3<bool>) -> V2<bool> {
    let r = filled.len();
    let mut ret = mat![false; r; r];
    for y in 0..r {
        for x in 0..r {
            for z in 0..r {
                ret[x][z] |= filled[x][y][z];
            }
        }
    }
    ret
}


pub fn shrink(orig: &V2<bool>, grid_size: usize) -> Vec<(Vec<usize>, Vec<usize>, V2<bool>)> {
    let rx = orig.len();
    let rz = orig[0].len();
    let mut cum = mat![0; rx+1; rz+1];
    for x in 0..rx {
        for z in 0..rz {
            cum[x+1][z+1] = orig[x][z] as i32 + cum[x][z+1] + cum[x+1][z] - cum[x][z];
        }
    }
    let mut ret = Vec::new();
    for bx in (1..=grid_size).rev() {
        for bz in (1..=grid_size).rev() {
            let rx_small = (rx + grid_size - bx + grid_size - 1) / grid_size;
            let rz_small = (rz + grid_size - bz + grid_size - 1) / grid_size;
            let mut small = mat![false; rx_small; rz_small];

            let mut xbounds = vec![0];
            for ix in 0..rx_small {
                xbounds.push(rx.min(bx + ix * grid_size));
            }
            let mut zbounds = vec![0];
            for iz in 0..rz_small {
                zbounds.push(rz.min(bz + iz * grid_size));
            }

            for ix in 0..rx_small {
                for iz in 0..rz_small {
                    let gx0 = xbounds[ix];
                    let gx1 = xbounds[ix+1];
                    let gz0 = zbounds[iz];
                    let gz1 = zbounds[iz+1];
                    // eprintln!("({}..{}, {}..{})", gx0, gx1, gz0, gz1);
                    small[ix][iz] = (cum[gx1][gz1] - cum[gx0][gz1] - cum[gx1][gz0] + cum[gx0][gz0]) > 0;
                }
            }
            ret.push((xbounds, zbounds, small));
        }
    }
    ret
}
