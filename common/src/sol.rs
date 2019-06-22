use crate::Action;

use std::iter::*;
use std::str::*;

pub fn read_sol(path: &str) -> Vec<Vec<Action>> {
    parse_sol(&std::fs::read_to_string(path).expect(&format!("cannot read {}", path)))
}

pub fn read_sol1(path: &str) -> Vec<Action> {
    let mut sol = read_sol(path);
    assert_eq!(sol.len(), 1);
    sol.pop().unwrap()
}

pub fn parse_sol(s: &str) -> Vec<Vec<Action>> {
    eprintln!("solution: {}", s);
    let mut iter = s.chars().peekable();
    let mut vs = vec![];
    let mut v = vec![];
    while let Some(c) = iter.next() {
        if c.is_whitespace() {
            continue;
        }
        if c == '#' {
            vs.push(v);
            v = vec![];
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
                let x = consume_int(&mut iter).unwrap();
                assert_eq!(Some(','), iter.next());
                let y = consume_int(&mut iter).unwrap();
                assert_eq!(Some(')'), iter.next());
                Action::Extension(x, y)
            }
            'F' => Action::Fast,
            'L' => Action::Drill,
            'R' => Action::Reset,
            'T' => {
                assert_eq!(Some('('), iter.next());
                let x = consume_int(&mut iter).unwrap();
                assert_eq!(Some(','), iter.next());
                let y = consume_int(&mut iter).unwrap();
                assert_eq!(Some(')'), iter.next());
                Action::Teleport(x, y)
            },
            'C' => Action::CloneWorker,
            // Panic
            _ => panic!("unexpected `{}`", c),
        })
    }
    vs.push(v);
    vs
}

fn consume_int<I: FromStr, Iter: Iterator<Item = char>>(
    p: &mut Peekable<Iter>,
) -> Result<I, <I as FromStr>::Err> {
    let mut s = String::new();
    while let Some(&c) = p.peek() {
        if c != '-' && !c.is_ascii_digit() {
            break;
        }
        s.push(c);
        p.next();
    }
    s.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        eprintln!("{:?}", parse_sol("WASD QE Z B(10,-1) F L R T(1,2)"));
    }

    fn test2() {
        eprintln!("{:?}", parse_sol("WASD QEC#Z B(10,-1) F L R T(1,2)"));
    }

    #[test]
    fn test_example_01_1() {
        eprintln!("{:?}", read_sol("../data/part-1-examples/example-01-1.sol"));
    }
}
