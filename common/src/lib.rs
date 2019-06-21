#[derive(Copy, Debug, Clone)]
pub enum Square {
    Empty,
    Block,
    Extension,
    Fast,
    Drill,
    X,
}

pub fn read_map() -> (Vec<Vec<Square>>, usize, usize) {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
