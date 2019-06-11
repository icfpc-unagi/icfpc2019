use std::cmp::min;
use super::super::{P, Model, V3};
use super::structs::Bot;
use std::collections::{HashMap, BinaryHeap};

pub struct Harmonizer {
    pub model: Model,  // Never modified!! Original!!
    pub pos_to_step: V3<usize>,
    pub step_to_poss: HashMap<usize, Vec<P>>,
}

impl Harmonizer {
    pub fn new(model: &Model) -> Harmonizer {
        let r = model.r;
        Harmonizer {
            model: model.clone(),
            pos_to_step: mat![!0; r; r; r],
            step_to_poss: HashMap::new(),
        }
    }

    pub fn gvoid_below_layer(&mut self, bots: &[&Bot; 4], current_step: usize) {
        // TODO
        let ps: Vec<_> = bots.iter().map(|b| b.p).collect();
        let y = ps[0].y - 1;
        let xmin = ps.iter().map(|p| p.x).min().unwrap();
        let xmax = ps.iter().map(|p| p.x).max().unwrap();
        let zmin = ps.iter().map(|p| p.z).min().unwrap();
        let zmax = ps.iter().map(|p| p.z).max().unwrap();

        let mut poss = self.step_to_poss.entry(current_step).or_insert(vec![]);
        for x in xmin..(xmax + 1) {
            for z in zmin..(zmax + 1) {
                let p = P::new(x, y, z);
                if self.model.filled[p] && self.pos_to_step[p] == !0 {
                    self.pos_to_step[p] = current_step;
                    poss.push(p);
                }
            }
        }
    }

    pub fn check_complete(&self) {
        let r = self.model.r;
        let mut f = false;
        for x in 0..r {
            for y in 0..r {
                for z in 0..r {
                    if !self.model.filled[x][y][z] {
                        continue;
                    }
                    if self.pos_to_step[x][y][z] == !0 {
                        eprintln!("Position {} {} {} not voided", x, y, z);
                        f = true;
                    }
                }
            }
        }
        if f {
            panic!();
        }
    }

    pub fn compute_reachable_step(&self) -> V3<usize> {
        let r = self.model.r;
        let mut que = BinaryHeap::new();
        let mut stp = mat![!0; r; r; r];  // Larger is better

        macro_rules! enqueue {
			($s: expr, $p: expr) => { {
			    (|s: usize, p: P| {
			        if !p.is_valid(r) || !self.model.filled[p] {
			            return;
			        }
			        let s = min(s, self.pos_to_step[p]);
                    if s <= stp[p] && stp[p] != !0 {
                        return;
                    }
                    stp[p] = s;
                    que.push((s, p));
			    })($s, $p)
			} };
		}

        for x in 0..r {
            for z in 0..r {
                enqueue!(!0, P::new(x as i32, 0, z as i32));
            }
        }

        while let Some((s, p)) = que.pop() {
            // eprintln!("{:?} {:?}", s, p);

            if s < stp[p] {
                continue;
            }
            for tp in p.adj(r) {
                enqueue!(s, tp);
            }
        }

        if false {
            let z = r / 2 - 1;
            for y in (0..r).rev() {
                for x in 0..r {
                    eprint!("{}", if self.pos_to_step[x][y][z] == !0 {
                        "    ".to_owned()
                    } else {
                        format!(" {:3}", self.pos_to_step[x][y][z])
                    });
                }
                eprintln!();
            }

            let z = r / 2 - 1;
            for y in (0..r).rev() {
                for x in 0..r {
                    eprint!("{}", if stp[x][y][z] == !0 {
                        "    ".to_owned()
                    } else {
                        format!(" {:3}", stp[x][y][z])
                    });
                }
                eprintln!();
            }
        }

        return stp;
    }

    pub fn compute_harmony_requirement(&self, n_steps: usize) -> Vec<bool> {
        let reachable_step = self.compute_reachable_step();

        let mut harmony_required = vec![false; n_steps];

        for (&step, poss) in self.step_to_poss.iter() {
            let broken_step = poss.iter().map(|p| reachable_step[*p]).min();

            if let Some(broken_step) = broken_step {
                if broken_step != step {
                    eprintln!("Broken period: {} -> {}", broken_step, step);
                }

                // Broken in `broken_step`, fixed in `step`
                for s in (broken_step + 1)..(step + 1) {
                    harmony_required[s] = true;
                }
            }
        }

        return harmony_required;

        // println!("{:?}", reachable_step);
    }
}