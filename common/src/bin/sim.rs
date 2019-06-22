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
    .optflag("s", "silent", "silence verbose logs")
    .optopt("g", "png", "output png", "file path")
    .parsing_style(ParsingStyle::FloatingFrees);
  let matches = opts.parse(&args[1..]).unwrap_or_else(|e| panic!(e));
  if matches.free.len() < 2 {
    eprintln!(
      "{}",
      opts.usage(&format!("Usage: {} <desc> <sol>", &args[0]))
    );
    std::process::exit(1);
  }

  let silent = matches.opt_present("s");
  let png = matches.opt_str("g").and_then(|path| {
    std::fs::File::create(&path)
      .map_err(|e| eprintln!("failed to open {}: {}", &path, e))
      .ok()
  });

  let task_path = &matches.free[0];
  let sol_path = &matches.free[1];

  // Initialize
  let (mut map, mut booster, init_x, init_y) = read_task(task_path);
  let xsize = map.len();
  let ysize = map[0].len();
  let sol = read_sol1(sol_path);
  let mut worker = WorkerState::new2(init_x, init_y, &mut map);
  let mut time = 0;

  // Play
  let hr = &repeat('-').take(xsize + 5).collect::<String>();
  let mut time_filled = vec![vec![0usize; ysize]; xsize];
  for action in sol {
    let update = apply_action(action, &mut worker, &mut map, &mut booster);
    for f in &update.filled {
      time_filled[f.0][f.1] = time;
    }
    time += 1;
    if !silent {
      eprintln!("Time: {}", time);
      eprintln!("Action: {}", action);
      eprintln!("{:?}", worker);
      eprintln!("{:?}", update);
      print_task(&(map.clone(), booster.clone(), worker.x, worker.y));
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
    print_task(&(map.clone(), booster.clone(), worker.x, worker.y));
    std::process::exit(1);
  }
  println!("{}", time);
}
