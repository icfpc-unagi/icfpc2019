use common::*;
use task::*;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 4 {
        eprintln!("Usage: {} <task.desc> <solution.sol> <output.png>", args[0]);
        std::process::exit(1);
    }
    
    let mut task = read_task(&args[1]);
    let actions = read_sol(&args[2]);
    let actions = actions[0].clone().into_iter().map(
        |a| {
            match a {
                Action::Move(_) | Action::TurnL | Action::TurnR | Action::Extension(_, _) => a,
                _ => Action::Nothing
            }
        }
    ).collect::<Vec<_>>();
    for i in 0..task.0.len() {
        for j in 0..task.0[0].len() {
            task.1[i][j] = Some(Booster::Extension);
        }
    }
    let sol = local_optimization::DynamicSolution::new(&task.0, &task.1, &local_optimization::get_initial_state(&task), &actions);
    let png = std::fs::File::create(&args[3])
        .unwrap_or_else(|e| panic!("failed to open {}: {}", &args[3], e));
    gen_png(
        task.0.len(),
        task.0[0].len(),
        |x, y| {
            let c = sol.dynamic_map.fill_count[x][y].min(5) as u8;
            if task.0[x][y] == Square::Block {
                (0, 0, 0)
            } else {
                (255, 255 - 50 * c, 255 - 50 * c)
            }
        },
        png,
    )
    .unwrap();
}

fn gen_png<F: Fn(usize, usize) -> (u8, u8, u8), W: std::io::Write>(
    xsize: usize,
    ysize: usize,
    f: F,
    w: W,
) -> std::io::Result<()> {
    image::png::PNGEncoder::new(w).encode(
        image::RgbImage::from_fn(xsize as u32, ysize as u32, |x, y| {
            let (r, g, b) = f(x as usize, y as usize);
            image::Rgb::<u8> { data: [r, g, b] }
        })
        .iter()
        .as_slice(),
        xsize as u32,
        ysize as u32,
        image::RGB(8),
    )
}
