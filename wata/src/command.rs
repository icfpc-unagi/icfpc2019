use std::io::BufRead;
use std::str::FromStr;
use *;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Command {
    Halt,
    Wait,
    Flip,
    SMove(P),
    LMove(P, P),
    FusionP(P),
    FusionS(P),
    Fission(P, usize),
    Fill(P),
    Void(P),
    GFill(P, P),
    GVoid(P, P),
}

impl ToString for Command {
    fn to_string(&self) -> String {
        match *self {
            Command::Halt => "HALT".to_owned(),
            Command::Wait => "WAIT".to_owned(),
            Command::Flip => "FLIP".to_owned(),
            Command::SMove(d) => format!("SMOVE {}", d.fmt_ld()),
            Command::LMove(d1, d2) => format!("LMOVE {} {}", d1.fmt_ld(), d2.fmt_ld()),
            Command::FusionP(p) => format!("FUSIONP {} {} {}", p.x, p.y, p.z),
            Command::FusionS(p) => format!("FUSIONS {} {} {}", p.x, p.y, p.z),
            Command::Fission(p, m) => format!("FISSION {} {} {} {}", p.x, p.y, p.z, m),
            Command::Fill(p) => format!("FILL {} {} {}", p.x, p.y, p.z),
            Command::Void(p) => format!("VOID {} {} {}", p.x, p.y, p.z),
            Command::GFill(nd, fd) => format!("GFILL {} {} {} {} {} {}", nd.x, nd.y, nd.z, fd.x, fd.y, fd.z),
            Command::GVoid(nd, fd) => format!("GVOID {} {} {} {} {} {}", nd.x, nd.y, nd.z, fd.x, fd.y, fd.z),
        }
    }
}

fn parse_ld(tokens: &[&str]) -> P {
    assert_eq!(tokens.len(), 2);
    let dir = tokens[0];
    let dis = i32::from_str(tokens[1]).unwrap();
    return match dir {
        "X" => P::new(dis, 0, 0),
        "Y" => P::new(0, dis, 0),
        "Z" => P::new(0, 0, dis),
        _ => panic!(),
    };
}

fn parse_nd(tokens: &[&str]) -> P {
    assert_eq!(tokens.len(), 3);
    return P::new(
        i32::from_str(tokens[0]).unwrap(),
        i32::from_str(tokens[1]).unwrap(),
        i32::from_str(tokens[2]).unwrap(),
    );
}

fn parse_fd(tokens: &[&str]) -> P {
    parse_nd(tokens)
}

impl std::str::FromStr for Command {
    type Err = (); // We don't use errors, immediate panic is preferred :)

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split(' ').collect();
        assert!(tokens.len() >= 1);

        return Ok(match tokens[0] {
            "HALT" => Command::Halt,
            "WAIT" => Command::Wait,
            "FLIP" => Command::Flip,
            "SMOVE" => {
                assert_eq!(tokens.len(), 3);
                Command::SMove(parse_ld(&tokens[1..]))
            }
            "LMOVE" => {
                assert_eq!(tokens.len(), 5);
                Command::LMove(parse_ld(&tokens[1..3]), parse_ld(&tokens[3..]))
            }
            "FUSIONP" => {
                assert_eq!(tokens.len(), 4);
                Command::FusionP(parse_nd(&tokens[1..]))
            }
            "FUSIONS" => {
                assert_eq!(tokens.len(), 4);
                Command::FusionS(parse_nd(&tokens[1..]))
            }
            "FISSION" => {
                assert_eq!(tokens.len(), 5);
                Command::Fission(parse_nd(&tokens[1..4]), usize::from_str(tokens[4]).unwrap())
            }
            "FILL" => {
                assert_eq!(tokens.len(), 4);
                Command::Fill(parse_nd(&tokens[1..]))
            }
            "VOID" => {
                assert_eq!(tokens.len(), 4);
                Command::Void(parse_nd(&tokens[1..]))
            }
            "GFILL" => {
                assert_eq!(tokens.len(), 7);
                Command::GFill(parse_nd(&tokens[1..4]), parse_fd(&tokens[4..]))
            }
            "GVOID" => {
                assert_eq!(tokens.len(), 7);
                Command::GVoid(parse_nd(&tokens[1..4]), parse_fd(&tokens[4..]))
            }
            _ => panic!(),
        });
    }
}

pub fn read_trace(path: &str) -> Vec<Command> {
    // Specify "-" to read from stdin

    let br: Box<BufRead>;
    if path == "-" {
        br = Box::new(std::io::BufReader::new(std::io::stdin()));
    } else {
        let file = std::fs::File::open(path).unwrap();
        br = Box::new(std::io::BufReader::new(file));
    }

    let mut commands = vec![];

    for line in br.lines() {
        let mut line_raw = line.unwrap();

        // Remove comment and strip
        let line_clean;
        if let Some(i) = line_raw.find('#') {
            line_clean = &line_raw[..i];
        } else {
            line_clean = &line_raw;
        }
        let line_clean = line_clean.trim();

        if line_clean.len() == 0 {
            continue;
        }

        let line_clean = line_clean.to_uppercase();

        commands.push(Command::from_str(&line_clean).unwrap());
    }

    return commands;
}
