use super::super::{V3, P};

pub fn get_filled_positions(filled: &V3<bool>) -> Vec<P> {
    let r = filled.len();
    let mut ps = vec![];
    for x in 0..r {
        for y in 0..r {
            for z in 0..r {
                let p = P::new(x as i32, y as i32, z as i32);
                if filled[p] {
                    ps.push(p);
                }
            }
        }
    }
    return ps;
}

pub fn get_bounding_box(filled: &V3<bool>) -> (P, P) {
    let ps = get_filled_positions(filled);
    return (
        P::new(
            ps.iter().map(|p| p.x).min().unwrap(),
            ps.iter().map(|p| p.y).min().unwrap(),
            ps.iter().map(|p| p.z).min().unwrap(),
        ),
        P::new(
            ps.iter().map(|p| p.x).max().unwrap(),
            ps.iter().map(|p| p.y).max().unwrap(),
            ps.iter().map(|p| p.z).max().unwrap(),
        )
    );
}
