extern crate wata;
use wata::*;

fn main() {
    let mut n = 1;
    while let Some(file) = std::env::args().nth(n) {
        let model = wata::read(&file);
        let (l, h) = destruction::util::get_bounding_box(&model.filled);
        println!("{}:  y(+{:2}), x({:2}+{:2}), z({:2}+{:2})", file,
        h.y, l.x, h.x-l.x, l.z, h.z-l.z
        );
        n += 1;
    }
}

