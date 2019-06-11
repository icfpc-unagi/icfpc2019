extern crate wata;

use wata::Command;
use wata::destruction;

fn emit(commands: Vec<Command>) {
    for command in commands.iter() {
        println!("{}", command.to_string());
    }
}

fn main() {
    assert_eq!(std::env::args().nth(1).unwrap(), ""); // I am destroy-only solver
    let file = std::env::args().nth(2).unwrap();
    let model = wata::read(&file);

    let mut app = destruction::strategy_large::App::new(&model);
    app.prepare_bot_grid(8, 4);
    for ix in 0..8 {
        app.bot_grid_relps[ix][2].z = 50;
        app.bot_grid_relps[ix][3].z = 73;
    }

    app.prepare_session_schedule();

    // Hack!
    app.session_absps.swap(1, 2);

    app.fission();
    app.destroy_all();
    app.harmonize();
    app.fusion();

    let commands = app.get_trace();
    emit(commands);
    eprintln!("{}", file);
}
