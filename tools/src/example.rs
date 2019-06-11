use clap;

pub fn app<'a, 'b>() -> clap::App<'a, 'b> {
    clap::SubCommand::with_name("example")
        .about("Runs an example command.")
        .arg(
            clap::Arg::with_name("value")
                .short("v")
                .long("value")
                .value_name("VALUE")
                .help("Sets a custom value.")
                .takes_value(true),
        )
}

pub fn main(matches: &clap::ArgMatches) {
    println!("Hello, {}!", matches.value_of("value").unwrap_or("world"));
}
