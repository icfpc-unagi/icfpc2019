
use crate::*;

use task::RasterizedTask;
use std::vec::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapState {
    pub map: Vec<Vec<Square>>,
    pub booster: Vec<Vec<Option<Booster>>>,
}

impl MapState {
  pub  fn from(t: &RasterizedTask) -> MapState {
        MapState{
            map: t.0.clone(),
            booster: t.1.clone(),
        }
    }
  pub  fn empty(xsize: usize, ysize: usize)->MapState {
        MapState {
            map: vec![vec![Square::Empty;ysize];xsize],
            booster: vec![vec![None;ysize]; xsize],
        }
    }
}

impl MapState {
    pub fn xsize(&self) -> usize {
        self.map.len()
    }
    pub fn ysize(&self) -> usize {
        self.map[0].len()
    }
    pub fn size(&self) -> (usize, usize) {
        (self.xsize(), self.ysize())
    }
    pub fn is_enterable(&self, (x, y): (usize, usize)) -> bool {
        is_enterable(x, y, &self.map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
