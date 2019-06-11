extern crate wata;

use wata::Command;
use wata::destruction::strategy_large;

fn emit(commands: Vec<Command>) {
    for command in commands.iter() {
        println!("{}", command.to_string());
    }
}

fn main() {
    assert_eq!(std::env::args().nth(1).unwrap(), ""); // I am destroy-only solver
    let file = std::env::args().nth(2).unwrap();
    let model = wata::read(&file);

    let n_bots_x = std::env::args().nth(3).unwrap().parse().unwrap();
    let n_bots_z = std::env::args().nth(4).unwrap().parse().unwrap();
    let use_support = std::env::args().nth(5).unwrap().parse().unwrap();
    let use_dense = std::env::args().nth(6).unwrap().parse().unwrap();
    eprintln!(
        "Config: n_bots_x={}, n_bots_z={}, support={}, dense={}",
        n_bots_x, n_bots_z, use_support, use_dense);

    let commands = strategy_large::destroy_large_with_config(
        model, n_bots_x, n_bots_z, use_support, use_dense);
    emit(commands);
    eprintln!("{}", file);
}
