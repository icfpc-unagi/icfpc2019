use common::*;
use std::iter::*;

fn main() {
  // TODO: flags to switch features
  let args = std::env::args().collect::<Vec<_>>();
  if args.len() < 3 {
    eprintln!("usage: {} <desc> <sol>", args.get(0).unwrap());
    std::process::exit(1);
  }
  let (mut map, mut booster, init_x, init_y) = read_task(&args[1]);
  let xsize = map.len();
  // let ysize = map[0].len();
  let sol = read_sol(&args[2]);
  let mut worker = WorkerState::new2(init_x, init_y, &mut map);
  let mut time = 0;

  let hr = &repeat('-').take(xsize + 5).collect::<String>();
  for action in sol {
      apply_action(action, &mut worker, &mut map, &mut booster);
      time += 1;
      eprintln!("Action: {}", action);
      eprintln!("{:?}", worker);
      print_task(&(map.clone(), booster.clone(), worker.x, worker.y));
      eprintln!("{}", hr);
  }
  println!("{}", time);
}
