use common::*;
use task::*;
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 3 {
        eprintln!("Usage: {} <task.desc> <output.png>", args[0]);
        std::process::exit(1);
    }

    let (map, booster, init_x, init_y) = read_task(&args[1]);
    let png = std::fs::File::create(&args[2])
        .unwrap_or_else(|e| panic!("failed to open {}: {}", &args[2], e));
    gen_png(
        map.len(),
        map[0].len(),
        |x, y| {
            if x == init_x && y == init_y {
                (255, 0, 0)
            } else if let Some(b) = booster[x][y] {
                match b {
                    Booster::Extension => (255, 255, 0),
                    Booster::Fast => (128, 128, 0),
                    Booster::Drill => (0, 255, 0),
                    Booster::Teleport => (128, 0, 255),
                    Booster::CloneWorker => (0, 255, 255),
                    Booster::X => (0, 0, 255),
                }
            } else {
                match map[x][y] {
                    Square::Block => (0, 0, 0),
                    Square::Empty => (255, 255, 255),
                    Square::Filled => (128, 128, 128),
                }
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
