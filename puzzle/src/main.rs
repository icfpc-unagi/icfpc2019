// use std::fs::File;
// use std::io::prelude::*;

use common::{parse_map};

fn main() -> std::io::Result<()> {
    // println!("Hello, world!");
    let path = std::env::args().nth(1).expect("usage: args[1] = condfile");
    /*
    let mut file = File::open(path);
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    */
    let s = std::fs::read_to_string(path).expect("cannot read cond file");
    let ss: Vec<_> = s.split('#').collect();
    assert_eq!(ss.len(), 3);
    let nums: Vec<_> = ss[0].split(',').map(|n| n.parse::<i32>().unwrap()).collect();
    let isqs = parse_map(&ss[1]);
    let osqs = parse_map(&ss[2]);
    dbg!(&nums);
    Ok(())
}
