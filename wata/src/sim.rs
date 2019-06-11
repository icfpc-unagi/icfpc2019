#![allow(unused)]
use *;
use Command::*;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bot {
    pub bid: usize,  // should be sorted by (unique) bid
    pub p: P,
    pub seeds: BTreeSet<usize>,
}

impl Bot {
    fn new(n_seeds: usize) -> Bot {
        Bot {
            bid: 1,
            p: P::new(0, 0, 0),
            seeds: (2..=n_seeds).collect(),
        }
    }

    fn from_position(p: P) -> Bot {
        Bot {
            bid: 0,
            p,
            seeds: BTreeSet::new(),
        }
    }

    fn fission(&mut self, nd: P, m: usize) -> Bot {
        let mut seeds_old = std::mem::replace(&mut self.seeds, BTreeSet::new()).into_iter();
        let bid = seeds_old.next().unwrap();
        // let seeds = seeds_old.take(m).collect();
        let mut seeds = BTreeSet::new();
        for _ in 0..m {
            seeds.insert(seeds_old.next().unwrap());
        }
        self.seeds = seeds_old.collect();
        Bot {
            bid,
            p: self.p + nd,
            seeds,
        }
    }

    fn fusion(&mut self, mut other: Bot) {
        self.seeds.insert(other.bid);
        self.seeds.append(&mut other.seeds);
    }
}

#[derive(Clone, Debug)]
pub struct SimState {
    // energy: i64,
    // harmonics: bool,
    pub matrix: V3<bool>,
    pub bots: BTreeSet<Bot>,
}

impl SimState {
    pub fn new(r: usize, n_seeds: usize) -> SimState {
        let bot = Bot::new(n_seeds);
        let mut bots = BTreeSet::new();
        bots.insert(bot);
        SimState {
            matrix: mat![false; r; r; r],
            bots,
        }
    }

    pub fn from_positions(matrix: V3<bool>, positions: Vec<P>) -> SimState {
        let mut bots = BTreeSet::new();
        for (i, &pos) in positions.iter().enumerate() {
            let mut bot = Bot::from_position(pos);
            bot.bid = i;
            bots.insert(bot);
        }
        SimState {
            matrix,
            bots,
        }
    }

    pub fn step(&mut self, cmds: Vec<Command>) {
        let bots = std::mem::replace(&mut self.bots, BTreeSet::new());
        assert!(bots.len() == cmds.len());
        let mut pbots = BTreeMap::new();
        let mut sbots = BTreeMap::new();
        for (mut bot, cmd) in bots.into_iter().zip(cmds) {
            if cmd == Halt {
                continue;
            }
            if let FusionP(nd) = cmd {
                pbots.insert((bot.p, bot.p + nd), bot);
                continue;
            }
            if let FusionS(nd) = cmd {
                sbots.insert((bot.p + nd, bot.p), bot);
                continue;
            }
            match cmd {
                SMove(d) => {bot.p += d}
                LMove(d1, d2) => {bot.p += d1 + d2}
                Fission(nd, m) => {
                    self.bots.insert(bot.fission(nd, m));
                }
                _ => {}
            }
            self.bots.insert(bot);
        }
        for (k, mut pbot) in pbots.into_iter() {
            let sbot = sbots.remove(&k).unwrap();
            pbot.fusion(sbot);
            self.bots.insert(pbot);
        }
    }
}
