extern crate clap;
extern crate utils;

pub mod example;

fn main() {
    let mut app = clap::App::new("unagi").version("0.1").author("Team Unagi");
    let subcommands = [(example::app, example::main)];
    for subcommand in &subcommands {
        app = app.subcommand(subcommand.0());
    }
    let matches = app.get_matches();
    for subcommand in &subcommands {
        let matches = match matches.subcommand_matches(subcommand.0().get_name()) {
            Some(matches) => matches,
            _ => continue,
        };
        subcommand.1(matches);
        std::process::exit(0);
    }
    println!("{}", matches.usage());
}
