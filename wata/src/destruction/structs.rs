use super::super::{P, Command};

#[derive(Copy, Clone, Debug)]
pub struct Bot {
    pub bid: usize,
    pub p: P,
}

//
// Commands invoked in a single time step by the bots
//

#[derive(Clone, Debug)]
pub struct CommandSet {
    pub     commands: Vec<Command>,
}

impl CommandSet {
    pub fn new(n_bots: usize) -> CommandSet {
        CommandSet {
            commands: vec![Command::Wait; n_bots],
        }
    }

    pub fn new_uniform(n_bots: usize, command: Command) -> CommandSet {
        CommandSet {
            commands: vec![command; n_bots],
        }
    }

    pub fn is_all_wait(&self) -> bool {
        return self.commands.iter().all(|&cmd| cmd == Command::Wait);
    }

    pub fn is_all_busy(&self) -> bool {
        return self.commands.iter().all(|&cmd| cmd != Command::Wait);
    }

    pub fn gvoid_below_layer(&mut self, bots: &[&Bot; 4]) {
        // TODO: 常に真下ではなく斜めを使ってわずかに稼ぐか？（優先度低い）
        let nd = P::new(0, -1, 0);

        for i in 0..4 {
            let b1 = bots[i];
            let b2 = bots[i ^ 3];
            assert_eq!(self.commands[b1.bid], Command::Wait);
            self.commands[b1.bid] = Command::GVoid(nd, b2.p - b1.p)
        }
    }

    pub fn flip_by_somebody(&mut self) {
        for i in 0..self.commands.len() {
            if self.commands[i] == Command::Wait {
                self.commands[i] = Command::Flip;
                return;
            }
        }
        panic!();
    }
}
