
use crate::*;
use std::vec::*;

pub struct Map {
    pub map: Vec<Vec<Square>>,
    pub booster: Vec<Vec<Option<Booster>>>,
}

impl Map {
    pub fn xsize(&self) -> usize {
        self.map.len()
    }
    pub fn ysize(&self) -> usize {
        self.map[0].len()
    }
    pub fn size(&self) -> (usize, usize) {
        (self.xsize(), self.ysize())
    }
    pub fn is_enterable(&self, x: usize, y: usize) -> bool {
        is_enterable(x, y, &self.map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
