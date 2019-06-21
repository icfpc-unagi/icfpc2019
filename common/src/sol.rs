use crate::Action;

use std::iter::*;
use std::str::*;

pub fn read_sol(path: &str) -> Vec<Action> {
    parse_sol(&std::fs::read_to_string(path).expect("path {}"))
}

pub fn parse_sol(s: &str) -> Vec<Action> {
    eprintln!("read: {}", s);
    let mut iter = s.chars().peekable();
    let mut v = vec![];
    while let Some(c) = iter.next() {
        if c.is_whitespace() {
            continue;
        }
        v.push(match c {
            // Move
            'W' => Action::Move(3),
            'A' => Action::Move(2),
            'S' => Action::Move(1),
            'D' => Action::Move(0),
            // Do nothing
            'Z' => Action::Nothing,
            // Turn
            'Q' => Action::TurnL,
            'E' => Action::TurnR,
            // Boost
            'B' => {
                assert_eq!(Some('('), iter.next());
                let x = consume_i32(&mut iter).unwrap();
                assert_eq!(Some(','), iter.next());
                let y = consume_i32(&mut iter).unwrap();
                assert_eq!(Some(')'), iter.next());
                Action::Extension(x, y)
            }
            'F' => Action::Fast,
            'L' => Action::Drill,
            'R' => Action::Reset,
            'T' => {
                assert_eq!(Some('('), iter.next());
                let x = consume_i32(&mut iter).unwrap();
                assert_eq!(Some(','), iter.next());
                let y = consume_i32(&mut iter).unwrap();
                assert_eq!(Some(')'), iter.next());
                Action::Teleport(x, y)
            },
            // Panic
            _ => panic!("unexpected `{}`", c),
        })
    }
    v
}

fn consume_i32<Iter: Iterator<Item = char>>(
    p: &mut Peekable<Iter>,
) -> Result<i32, <i32 as FromStr>::Err> {
    let mut s = String::new();
    while let Some(&c) = p.peek() {
        if c != '-' && !c.is_ascii_digit() {
            break;
        }
        s.push(c);
        p.next();
    }
    dbg!(s).parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        eprintln!("{:?}", parse_sol("WASD QE Z B(10,-1) F L R T(1,2)"));
    }

    #[test]
    fn test_example_01_1() {
        eprintln!("{:?}", read_sol("../data/part-1-examples/example-01-1.sol"));
    }
}
