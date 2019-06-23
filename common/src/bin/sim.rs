extern crate getopts;
extern crate image;

use common::*;

use getopts::*;
use std::iter::*;
fn main() {
  // Parse options
  let args = std::env::args().collect::<Vec<_>>();
  let mut opts = Options::new();
  opts
    .optopt("v", "verbose", "verbose logs", "0,1,2,3")
    .optopt("g", "png", "output png", "file path")
    .parsing_style(ParsingStyle::FloatingFrees);
  let matches = opts.parse(&args[1..]).unwrap_or_else(|e| panic!(e));
  if matches.free.len() < 2 {
    eprintln!(
      "{}",
      opts.usage(&format!("Usage: {} <desc> <sol> [buy]", &args[0]))
    );
    std::process::exit(1);
  }

  let verbose: i32 = matches.opt_get_default("v", 0).unwrap();
  let png = matches.opt_str("g").and_then(|path| {
    std::fs::File::create(&path)
      .map_err(|e| eprintln!("failed to open {}: {}", &path, e))
      .ok()
  });

  let task_path = &matches.free[0];
  let sol_path = &matches.free[1];
  let buy = matches.free.get(2).map(|b| parse_buy(&b)).unwrap_or(vec![]);

  // Initialize
  let (mut map, mut booster, init_x, init_y) = read_task(task_path);
  let xsize = map.len();
  let ysize = map[0].len();
  let sol = &read_sol(sol_path);
  let mut workers = WorkersState::new_t0_with_options(init_x, init_y, &mut map, buy);
  let mut time = 0;

  // Play
  let hr = &repeat('-').take(xsize + 5).collect::<String>();
  let mut time_filled = vec![vec![0usize; ysize]; xsize];
  let mut action_iters = vec![sol[0].iter()];
  while action_iters.iter().any(|it| it.clone().next().is_some()) {
    let actions = action_iters.iter_mut().map(|it| it.next().copied().unwrap_or(Action::Nothing)).collect::<Vec<_>>();
    let update = apply_multi_action(&actions, &mut workers, &mut map, &mut booster);
    for i in action_iters.len()..action_iters.len()+update.num_cloned {
      action_iters.push(sol[i].iter());
    }
    for f in &update.filled {
      time_filled[f.0][f.1] = time;
    }
    time += 1;
    if verbose >= 2 {
      eprintln!("Time: {}", time);
      eprintln!("Actions: {:?}", actions);
      eprintln!("{:?}", workers);
      eprintln!("{:?}", update);
      if verbose >= 3 {
            // TODO: print workers
        print_task(&(map.clone(), booster.clone(), 9999, 9999));
      }
      eprintln!("{}", hr);
    }
  }

  // If -g specified, output png.
  if let Some(png) = png {
    image::png::PNGEncoder::new(png)
      .encode(
        image::RgbImage::from_fn(xsize as u32, ysize as u32, |x, y| {
          let x = x as usize;
          let y = y as usize;
          match map[x][y] {
            Square::Block => image::Rgb::<u8> { data: [0, 0, 0] },
            Square::Empty => image::Rgb::<u8> { data: [255, 0, 0] },
            Square::Filled => {
              let t = (time_filled[x][y] as f32) / time as f32;
              image::Rgb::<u8> {
                data: [
                  (t * 255.0) as u8,
                  ((1.0 - (t - 0.5) * (t - 0.5) * 4.0) * 255.0) as u8,
                  ((1.0 - t) * 255.0) as u8,
                ],
              }
            }
          }
        })
        .iter()
        .as_slice(),
        xsize as u32,
        ysize as u32,
        image::RGB(8),
      )
      .unwrap();
  }

  // Validate fulfillness
  if map.iter().any(|v| v.iter().any(|&s| s == Square::Empty)) {
    eprintln!("Not fulfilled!");

    print_task(&(map.clone(), booster.clone(), 99999, 99999));
    std::process::exit(1);
  }
  println!("{}", time);
}
