
use image::*;
use std::iter::*;

fn gen_unagi() -> Vec<Vec<u8>> {
  let pngfile = std::fs::File::create("Unagi.png")
      .unwrap_or_else(|e| panic!("failed to open unagi: {}" e));
  let png = image::png::PNGDecoder::new(pngfile).unwrap();
  let (xsize, ysize) = png.dimensions();
  let mut data = vec![vec![0u8; ysize] png.row_bytes()];
  let mut it = png.read_image().unwrap().iter();
  for y in 0..ysize {
    for x in 0..png.row_bytes() {
      data[x as usize][y as usize] = it.next().unwrap();
    }
  }
  data.resize(xsize, 0);
  data
}
