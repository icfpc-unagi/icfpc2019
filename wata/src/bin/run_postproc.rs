#![allow(unused)]
extern crate wata;

use wata::*;

fn main() {
    let file = std::env::args().nth(1).unwrap();
    let model = wata::read(&file);
    let r = model.r;

    let file = std::env::args().nth(2).unwrap();
    let mut cmds = wata::command::read_trace(&file);
    if cmds.len() > 0 && cmds[cmds.len() - 1] == Command::Halt {
        cmds.pop();
    }

    let mut sim = wata::sim::SimState::new(r, 40);

    let mut ip = 0;
    while ip < cmds.len() {
        let n = sim.bots.len();
        sim.step(Vec::from(&cmds[ip..ip + n]));
        ip += n;
    }
    assert_eq!(ip, cmds.len());

    for cmd in cmds {
        println!("{}", cmd.to_string());
    }

    eprintln!("{:?}", sim.bots);

    let mut positions = Vec::new();
    for bot in sim.bots.iter() {
        positions.push(bot.p);
    }
    for cmd in postproc::fusion_all(&model.filled, positions) {
        println!("{}", cmd.to_string());
    }
}
