use crate::Square;


type APos = (usize, usize);
type RPos = (i32, i32);


pub fn is_visible(map: &Vec<Vec<Square>>, current_pos: APos, relative_pos: RPos) -> bool {
    // reachableかを返す。現在地がblockでないことは仮定してる
    let rposs = deps(relative_pos);
    for r in rposs {
        let sq = *get_mat(&map, current_pos, r).unwrap_or(&Square::Block);
        if sq == Square::Block {
            return false;
        }
    }
    return true;
}


fn get_mat<T>(mat: &Vec<Vec<T>>, a: APos, r: RPos) -> Option<&T> {
    let (ax, ay) = a;
    let (rx, ry) = r;
    let x = (ax as i32 + rx) as usize;
    let y = (ay as i32 + ry) as usize;
    let vec = mat.get(x)?;
    return vec.get(y);
}


fn deps(v: RPos) -> Vec<RPos> {
    let (x, y) = v;
    let sx = x.signum();
    let sy = y.signum();
    let mut txs = vec![];
    for i in 0..x.abs() {
        txs.push((2 * i + 1) * y.abs());
    }
    let mut tys = vec![];
    for i in 0..y.abs() {
        tys.push((2 * i + 1) * x.abs());
    }
    txs.reverse();
    tys.reverse();

    let mut cx = 0;
    let mut cy = 0;
    let mut tx = txs.pop();
    let mut ty = tys.pop();
    let mut rets = vec![];
    while !(tx.is_none() && ty.is_none()) {
        if tx == ty {
            cx += sx;
            cy += sy;
            tx = txs.pop();
            ty = tys.pop();
        } else if ty.is_none() {
            cx += sx;
            tx = txs.pop();
        } else if tx.is_none() {
            cy += sy;
            ty = tys.pop();
        } else if tx < ty {
            cx += sx;
            tx = txs.pop();
        } else {
            cy += sy;
            ty = tys.pop();
        }
        rets.push((cx, cy));
    }
    return rets;
}


#[allow(dead_code)]
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a%b)
    }
}


#[allow(dead_code)]
fn main() {
    println!("{:?}", deps((3, -5)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(deps((-2, 0)), [(-1, 0), (-2, 0)]);
        assert_eq!(deps((2, 4)), [(0, 1), (1, 1), (1, 2), (1, 3), (2, 3), (2, 4)]);
        assert_eq!(deps((0, 0)), []);
        assert_eq!(deps((3, 1)), [(1, 0), (2, 1), (3, 1)]);
        for dx in -1 ..= 1 {
            for dy in -1 ..= 1 {
                if (dx, dy) == (0, 0) {
                    continue;
                }
                assert_eq!(deps((dx, dy)), [(dx, dy)]);
            }
        }
    }
}
