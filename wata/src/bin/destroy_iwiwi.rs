extern crate wata;

use wata::Command;
use wata::destruction::destroy;

fn emit(commands: Vec<Command>) {
    for command in commands.iter() {
        println!("{}", command.to_string());
    }
}

fn main() {
    assert_eq!(std::env::args().nth(1).unwrap(), ""); // I am destroy-only solver
    let file = std::env::args().nth(2).unwrap();
    let model = wata::read(&file);
    let commands = destroy(model);
    emit(commands);
    eprintln!("{}", file);
}
