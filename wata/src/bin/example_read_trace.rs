#![allow(unused)]
extern crate wata;
use wata::*;

fn main() {
    let commands = command::read_trace("-");
    println!("{:?}", commands);
}
