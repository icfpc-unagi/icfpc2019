
use image::*;
use std::iter::*;

pub fn gen_unagi() -> Vec<Vec<bool>> {
  let pngfile = std::fs::File::open("/nfs/github/puzzle/Unagi.png")
      .unwrap_or_else(|e| panic!("failed to open unagi: {}", e));
  let png = image::png::PNGDecoder::new(pngfile).unwrap();
  let (xsize, ysize) = png.dimensions();
  let xsize = xsize as usize;
  let ysize = ysize as usize;
  let row_bytes = png.row_bytes() as usize;
  assert_eq!(row_bytes % xsize, 0);
  let bytes_per_pixel = row_bytes / xsize;
  // dbg!((xsize, ysize, row_bytes));
  let mut data = vec![vec![false; ysize]; xsize];
  let tmp = png.read_image().unwrap();
  let mut it = tmp.iter();
  for y in 0..ysize {
    for x in 0..xsize {
        let mut sum = 0.0;
        for b in 0..bytes_per_pixel {
            sum += *it.next().unwrap() as f64;
        }
        sum /= bytes_per_pixel as f64;
      data[x as usize][y as usize] = sum < 180.0;
    }
  }
  data
}
