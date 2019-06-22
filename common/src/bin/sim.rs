extern crate getopts;

use common::*;

use getopts::*;
use std::iter::*;
fn main() {
  let args = std::env::args().collect::<Vec<_>>();
  let mut opts = Options::new();
  opts
    .optflag("s", "silent", "")
    .parsing_style(ParsingStyle::FloatingFrees);
  let matches = opts.parse(&args[1..]).unwrap_or_else(|e| panic!(e));
  if matches.free.len() < 2 {
    eprintln!("{}", opts.usage(&format!("Usage: {} <desc> <sol>", &args[0])));
    std::process::exit(1);
  }

  let task_path = &matches.free[0];
  let sol_path = &matches.free[1];

  let (mut map, mut booster, init_x, init_y) = read_task(task_path);
  let xsize = map.len();
  // let ysize = map[0].len();
  let sol = read_sol(sol_path);
  let mut worker = WorkerState::new2(init_x, init_y, &mut map);
  let mut time = 0;

  let hr = &repeat('-').take(xsize + 5).collect::<String>();
  for action in sol {
    apply_action(action, &mut worker, &mut map, &mut booster);
    time += 1;
    if !matches.opt_present("s") {
      eprintln!("Action: {}", action);
      eprintln!("{:?}", worker);
      print_task(&(map.clone(), booster.clone(), worker.x, worker.y));
      eprintln!("{}", hr);
    }
  }

  if map.iter().any(|v| v.iter().any(|&s| s == Square::Empty)) {
    eprintln!("Not fulfilled!");
    print_task(&(map.clone(), booster.clone(), worker.x, worker.y));
    std::process::exit(1);
  }
  println!("{}", time);
}
