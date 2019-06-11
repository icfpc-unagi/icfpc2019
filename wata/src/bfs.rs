use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use *;

/*
DONE:
- Only SMOVE
- Support LMOVE
- Restore
- multiple start points
- change `filled` to lambda
- Support goal

TODO items:
- sublinear initialization
*/

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub struct C(i32, i32); // (#commands, #continuous move)

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SearchState {
    p: P,
    d: usize, // Direction [0, 7), where 6 is a special direction
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct HeapState {
    c: C,
    s: SearchState,
}

impl Ord for HeapState {
    fn cmp(&self, _: &HeapState) -> Ordering {
        unimplemented!()
    }
}

impl PartialOrd for HeapState {
    fn partial_cmp(&self, other: &HeapState) -> Option<Ordering> {
        return Some(self.c.cmp(&other.c).reverse()); // Greater, as BimaryHeap is a max-heap.
    }
}

pub type V4<T> = Vec<Vec<Vec<Vec<T>>>>;

macro_rules! impl_index4 {
	($($T: ty),*) => {
		$(
			impl Index<SearchState> for V4<$T> {
				type Output = $T;
				fn index(&self, s: SearchState) -> &$T {
					&self[s.p.x as usize][s.p.y as usize][s.p.z as usize][s.d]
				}
			}
			impl IndexMut<SearchState> for V4<$T> {
				fn index_mut(&mut self, s: SearchState) -> &mut $T {
					&mut self[s.p.x as usize][s.p.y as usize][s.p.z as usize][s.d]
				}
			}
		)*
	};
}

impl_index4!(C, SearchState);
impl_index!(Vec<C>);

const LIMIT_SMOVE: i32 = 15;
const LIMIT_LMOVE: i32 = 5;

const MAX_C: C = C(std::i32::MAX, std::i32::MAX);

pub const DIR6: [P; 6] = [
    P { x: 1, y: 0, z: 0 },
    P { x: -1, y: 0, z: 0 },
    P { x: 0, y: 1, z: 0 },
    P { x: 0, y: -1, z: 0 },
    P { x: 0, y: 0, z: 1 },
    P { x: 0, y: 0, z: -1 },
];

const DUMMY_SEARCH_STATE: SearchState = SearchState {
    p: P {
        x: -1,
        y: -1,
        z: -1,
    },
    d: 6,
};

pub struct BFS {
    pub r: usize,
    pub cost: V3<Vec<C>>,
    pub prev: V3<Vec<SearchState>>,
    pub que: BinaryHeap<HeapState>,
    pub touched: Vec<SearchState>,
}

impl BFS {
    pub fn new(r: usize) -> BFS {
        BFS {
            r,
            cost: mat![MAX_C; r; r; r; 7],
            prev: mat![DUMMY_SEARCH_STATE; r; r; r; 7],
            que: BinaryHeap::new(),
            touched: vec![],
        }
    }

    fn enqueue<G: FnMut(P) -> bool>(
        &mut self,
        next: SearchState,
        cost: C,
        prev: SearchState,
        filled: &mut G,
    ) {
        if !next.p.is_valid(self.r) || (*filled)(next.p) {
            return;
        }

        if cost < self.cost[next] {
            if self.cost[next] == MAX_C {
                // First time to writing into that state?
                self.touched.push(next);
            }

            self.cost[next] = cost;
            self.prev[next] = prev;
            self.que.push(HeapState { c: cost, s: next });
        }
    }

    pub fn bfs<G: FnMut(P) -> bool, H: FnMut(P) -> bool>(
        &mut self,
        mut filled: G,
        starts: &Vec<P>,
        goal: H,
    ) -> Option<P> {
        assert_eq!(self.touched.len(), 0); // To confirm that the workspace is clean
        // Direction 6 is a special state only for the initialization
        for &p in starts.iter() {
            self.enqueue(
                SearchState { p, d: 6 },
                C(0, LIMIT_SMOVE),
                DUMMY_SEARCH_STATE,
                &mut filled,
            );
        }

        self.bfs_main(filled, goal)
    }

    pub fn bfs_continue<G: FnMut(P) -> bool>(
        &mut self,
        filled: G,
        goal_set: &BTreeSet<P>,
    ) -> Option<P> {

        let mut cost = MAX_C;
        let mut ret = None;
        for &p in goal_set.iter() {
            for d in 0..7 {
                let s = SearchState { p, d };
                if self.cost[s] < cost {
                    cost = self.cost[s];
                    ret = Some(p);
                }
            }
        }
        ret.or_else(||{
            let goal = |p: P| goal_set.contains(&p);
            self.bfs_main(filled, goal)
        })
    }

    pub fn bfs_main<G: FnMut(P) -> bool, H: FnMut(P) -> bool>(
        &mut self,
        mut filled: G,
        mut goal: H,
    ) -> Option<P> {

        while !self.que.is_empty() {
            let HeapState { c, s } = self.que.pop().unwrap();
            if c != self.cost[s] {
                continue;
            }
            let prev = self.prev[s];

            // Turn: starting a new command
            for td in 0..6 {
                self.enqueue(SearchState { p: s.p, d: td }, C(c.0 + 1, 0), s, &mut filled);
            }

            // Turn: 2nd step of a LMove command
            if c.1 <= LIMIT_LMOVE {
                for td in 0..6 {
                    let ts = SearchState { p: s.p, d: td };
                    self.enqueue(ts, C(c.0, LIMIT_SMOVE - LIMIT_LMOVE), s, &mut filled);
                }
            }

            // Straight: continue
            if s.d != 6 {
                let ss = SearchState {
                    p: s.p + DIR6[s.d],
                    d: s.d,
                };
                if c.1 + 1 <= LIMIT_SMOVE {
                    self.enqueue(
                        ss,
                        C(c.0, c.1 + 1),
                        prev,
                        &mut filled,
                    );
                } else {  // straight as next step
                    self.enqueue(
                        ss,
                        C(c.0 + 1, 1),
                        s,
                        &mut filled,
                    );
                }
            }

            if goal(s.p) {
                return Some(s.p);
            }
        }

        return None;
    }

    pub fn restore(&self, p: P) -> Vec<Command> {
        // Get the path to `p`

        let d: usize = (0..7).min_by_key(|&d| self.cost[p][d]).unwrap();
        let mut s = SearchState { p, d };
        assert_ne!(self.cost[s], MAX_C); // To confirm the reachability

        let mut cmds = vec![];

        while self.prev[s] != DUMMY_SEARCH_STATE {
            let ps1 = self.prev[s];

            if self.cost[s].0 != self.cost[ps1].0 {
                assert_eq!(self.cost[s].0, self.cost[ps1].0 + 1);
                cmds.push(Command::SMove(s.p - ps1.p));
                s = ps1;
            } else {
                let ps2 = self.prev[ps1];
                assert_eq!(self.cost[s].0, self.cost[ps2].0 + 1);
                cmds.push(Command::LMove(ps1.p - ps2.p, s.p - ps1.p));
                s = ps2;
            }
        }

        cmds.reverse();
        return cmds;
    }

    pub fn restore_backward(&self, p: P) -> Vec<Command> {
        let mut cmds_fwd = self.restore(p);
        cmds_fwd.reverse();
        return cmds_fwd.iter().map(|cmd| {
            match cmd {
                Command::SMove(p) => Command::SMove(-*p),
                Command::LMove(p, q) => Command::LMove(-*q, -*p),
                _ => panic!()
            }
        }).collect();
    }

    pub fn get_cost(&self, p: P) -> i32 {
        return self.cost[p].iter().min().unwrap().0;
    }

    pub fn clear(&mut self) {
        eprintln!("BFS: {} states touched", self.touched.len());
        for &s in self.touched.iter() {
            self.cost[s] = MAX_C;
            self.prev[s] = DUMMY_SEARCH_STATE;
        }
        self.que.clear();
        self.touched.clear();
    }

    pub fn show(&self) {
        for y in 0..self.r as i32 {
            println!("[y={}]", y);

            for x in 0..self.r as i32 {
                for z in 0..self.r as i32 {
                    let c = self.get_cost(P::new(x, y, z));
                    if c == std::i32::MAX {
                        print!("X ");
                    } else {
                        print!("{} ", c);
                    }
                }
                println!();
            }
        }
    }
}
