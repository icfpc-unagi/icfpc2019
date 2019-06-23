
use image::*;
use std::iter::*;

pub fn gen_unagi() -> Vec<Vec<u8>> {
  let pngfile = std::fs::File::open("Unagi.png")
      .unwrap_or_else(|e| panic!("failed to open unagi: {}", e));
  let png = image::png::PNGDecoder::new(pngfile).unwrap();
  let (xsize, ysize) = png.dimensions();
  let xsize = xsize as usize;
  let ysize = xsize as usize;
  let row_bytes = png.row_bytes() as usize;
  let mut data = vec![vec![0u8; ysize]; row_bytes];
  let tmp = png.read_image().unwrap();
  let mut it = tmp.iter();
  for y in 0..ysize {
    for x in 0..row_bytes {
      data[x as usize][y as usize] = *it.next().unwrap();
    }
  }
  data.resize(xsize, vec![]);
  data
}
