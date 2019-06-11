use std::io::Read;
use std::ops::*;
use std::collections::BTreeMap;

#[macro_export]
macro_rules! debug {
	($($v: expr),*) => {
		{
			use ::std::io::Write;
			$(let _ = write!(::std::io::stderr(), "{} = {:?} ", stringify!($v), $v);)*
			let _ = writeln!(::std::io::stderr(), "@ {}:{}", file!(), line!());
		}
	}
}
#[macro_export]
macro_rules! mat {
	($e:expr) => { $e };
	($e:expr; $d:expr $(; $ds:expr)*) => { vec![mat![$e $(; $ds)*]; $d] };
}
#[macro_export]
macro_rules! ok {
	($a:ident$([$i:expr])*.$f:ident()$(@$t:ident)*) => {
		$a$([$i])*.$f($($t),*)
	};
	($a:ident$([$i:expr])*.$f:ident($e:expr$(,$es:expr)*)$(@$t:ident)*) => { {
		let t = $e;
		ok!($a$([$i])*.$f($($es),*)$(@$t)*@t)
	} };
}

pub trait SetMin {
	fn setmin(&mut self, v: Self) -> bool;
}
impl<T> SetMin for T where T: PartialOrd {
	fn setmin(&mut self, v: T) -> bool {
		*self > v && { *self = v; true }
	}
}
pub trait SetMax {
	fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMax for T where T: PartialOrd {
	fn setmax(&mut self, v: T) -> bool {
		*self < v && { *self = v; true }
	}
}

pub type V3<T> = Vec<Vec<Vec<T>>>;

#[derive(Clone, Debug)]
pub struct Model {
	pub r: usize,
	pub filled: V3<bool>
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct P {
	pub x: i32,
	pub y: i32,
	pub z: i32,
}

impl P {
	pub fn mlen(&self) -> i32 {
		self.x.abs() + self.y.abs() + self.z.abs()
	}

    pub fn fmt_ld(self) -> String {
        if self.x != 0 {
            return format!("x {}", self.x);
        }
        if self.y != 0 {
            return format!("y {}", self.y);
        }
        if self.z != 0 {
            return format!("z {}", self.z);
        }
        panic!();
    }
}

pub const NEAR: [P; 18] = [
	P { x: -1, y: -1, z: 0 }, P { x: -1, y: 0, z: -1 }, P { x: -1, y: 0, z: 0 }, P { x: -1, y: 0, z: 1 }, P { x: -1, y: 1, z: 0 },
	P { x: 0, y: -1, z: -1 }, P { x: 0, y: -1, z: 0 }, P { x: 0, y: -1, z: 1 },
	P { x: 0, y: 0, z: -1 }, P { x: 0, y: 0, z: 1 },
	P { x: 0, y: 1, z: -1 }, P { x: 0, y: 1, z: 0 }, P { x: 0, y: 1, z: 1 },
	P { x: 1, y: -1, z: 0 }, P { x: 1, y: 0, z: -1 }, P { x: 1, y: 0, z: 0 }, P { x: 1, y: 0, z: 1 }, P { x: 1, y: 1, z: 0 }
];

pub const ADJ: [P; 6] = [
	P { x: -1, y: 0, z: 0 }, P { x: 1, y: 0, z: 0 },
	P { x: 0, y: -1, z: 0 }, P { x: 0, y: 1, z: 0 },
	P { x: 0, y: 0, z: -1 }, P { x: 0, y: 0, z: 1 },
];
impl P {
	pub fn new(x: i32, y: i32, z: i32) -> P {
		P { x, y, z }
	}
	pub fn is_valid(&self, r: usize) -> bool {
		let r = r as i32;
		0 <= self.x && self.x < r && 0 <= self.y && self.y < r && 0 <= self.z && self.z < r
	}
	pub fn near(&self, r: usize) -> Vec<P> {
		let mut near = vec![];
		for d in &NEAR {
			let q = self + d;
			if q.is_valid(r) {
				near.push(q);
			}
		}
		near
	}
	pub fn is_near(&self) -> bool {
		let dx = self.x.abs();
		let dy = self.y.abs();
		let dz = self.z.abs();
		dx <= 1 && dy <= 1 && dz <= 1 && 0 < dx + dy + dz && dx + dy + dz <= 2
	}
	pub fn adj(&self, r: usize) -> Vec<P> {
		let mut adj = vec![];
		for d in &ADJ {
			let q = self + d;
			if q.is_valid(r) {
				adj.push(q);
			}
		}
		adj
	}
}

impl<'a> Add for &'a P {
	type Output = P;
	fn add(self, a: &P) -> P {
		P::new(self.x + a.x, self.y + a.y, self.z + a.z)
	}
}

impl<'a> Sub for &'a P {
	type Output = P;
	fn sub(self, a: &P) -> P {
		P::new(self.x - a.x, self.y - a.y, self.z - a.z)
	}
}

impl Neg for P {
    type Output = P;
    fn neg(self) -> P {
        P::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<i32> for P {
	type Output = P;
	fn mul(self, a: i32) -> P {
		P::new(self.x * a, self.y * a, self.z * a)
	}
}

impl Div<i32> for P {
	type Output = P;
	fn div(self, a: i32) -> P {
		P::new(self.x / a, self.y / a, self.z / a)
	}
}

macro_rules! impl_all {
	($t:ident$(<$($g:ident),*>)*; $Op:ident:$op:ident:$Opa:ident:$opa:ident) => {
		impl<$($($g),*)*> $Op for $t$(<$($g),*>)* where for<'b> &'b $t$(<$($g),*>)*: $Op<Output = $t$(<$($g),*>)*> {
			type Output = $t$(<$($g),*>)*;
			#[inline]
			fn $op(self, a: $t$(<$($g),*>)*) -> $t$(<$($g),*>)* { (&self).$op(&a) }
		}
		impl<'a, $($($g),*)*> $Op<&'a $t$(<$($g),*>)*> for $t$(<$($g),*>)* where for<'b> &'b $t$(<$($g),*>)*: $Op<Output = $t$(<$($g),*>)*> {
			type Output = $t$(<$($g),*>)*;
			#[inline]
			fn $op(self, a: &$t$(<$($g),*>)*) -> $t$(<$($g),*>)* { (&self).$op(&a) }
		}
		impl<'a, $($($g),*)*> $Op<$t$(<$($g),*>)*> for &'a $t$(<$($g),*>)* where for<'b> &'b $t$(<$($g),*>)*: $Op<Output = $t$(<$($g),*>)*> {
			type Output = $t$(<$($g),*>)*;
			#[inline]
			fn $op(self, a: $t$(<$($g),*>)*) -> $t$(<$($g),*>)* { (&self).$op(&a) }
		}
		impl<$($($g),*)*> $Opa for $t$(<$($g),*>)* where for<'b> &'b $t$(<$($g),*>)*: $Op<Output = $t$(<$($g),*>)*> {
			#[inline]
			fn $opa(&mut self, a: $t$(<$($g),*>)*) { *self = (&*self).$op(&a) }
		}
	}
}

impl_all!(P; Add:add:AddAssign:add_assign);
impl_all!(P; Sub:sub:SubAssign:sub_assign);

macro_rules! impl_index {
	($($T: ty),*) => {
		$(
			impl Index<P> for V3<$T> {
				type Output = $T;
				fn index(&self, p: P) -> &$T {
					&self[p.x as usize][p.y as usize][p.z as usize]
				}
			}
			impl IndexMut<P> for V3<$T> {
				fn index_mut(&mut self, p: P) -> &mut $T {
					&mut self[p.x as usize][p.y as usize][p.z as usize]
				}
			}
		)*
	};
}

impl_index!(bool, usize);

#[derive(Clone, Debug)]
pub struct InitV3<T: Clone> {
	version: u32,
	init: T,
	data: V3<(T, u32)>
}

impl<T: Clone> InitV3<T> {
	pub fn new(v: T, r: usize) -> InitV3<T> {
		InitV3 { version: 0, init: v.clone(), data: mat![(v, 0); r; r; r] }
	}
	pub fn init(&mut self) {
		if self.version == u32::max_value() {
			for v in &mut self.data {
				for v in v {
					for v in v {
						v.1 = 0;
					}
				}
			}
			self.version = 1;
		} else {
			self.version += 1;
		}
	}
}

impl<T: Clone> Index<P> for InitV3<T> {
	type Output = T;
	#[inline]
	fn index(&self, i: P) -> &T {
		let e = &self.data[i.x as usize][i.y as usize][i.z as usize];
		if e.1 == self.version {
			&e.0
		} else {
			&self.init
		}
	}
}

impl<T: Clone> IndexMut<P> for InitV3<T> {
	#[inline]
	fn index_mut(&mut self, i: P) -> &mut T {
		let e = &mut self.data[i.x as usize][i.y as usize][i.z as usize];
		if e.1 != self.version {
			e.1 = self.version;
			e.0 = self.init.clone();
		}
		&mut e.0
	}
}

// pub const SEEDS: usize = 20;

pub fn read(path: &str) -> Model {
	let file = std::fs::File::open(path).unwrap();
	let mut reader = std::io::BufReader::new(file);
	let mut bytes = vec![];
	reader.read_to_end(&mut bytes).unwrap();
	let r = bytes[0] as usize;
	let mut filled = mat![false; r; r; r];
	for x in 0..r {
		for y in 0..r {
			for z in 0..r {
				let p = x * r * r + y * r + z;
				if bytes[1 + p / 8] >> (p % 8) & 1 != 0 {
					filled[x][y][z] = true;
				}
			}
		}
	}
	Model { r, filled }
}

pub fn fission_to(filled: &V3<bool>, to: &Vec<P>) -> (Vec<usize>, Vec<Command>)  {
    eprintln!("fission: started");
    let mut log_bots = Vec::new();
    let mut log_cmds = Vec::new();
    {
        let fusion_cmds = postproc::fusion_all(filled, to.clone());
        let mut sim = sim::SimState::from_positions(filled.clone(), to.clone());

        let mut ip = 0;
        while ip < fusion_cmds.len() {
            log_bots.push(sim.bots.clone());
            let n = sim.bots.len();
            let mut cmds_step = Vec::new();
            for i in ip..ip+n {
                cmds_step.push(fusion_cmds[i]);
            }
            // eprintln!("{:?}", cmds_step);
            log_cmds.push(cmds_step.clone());
            sim.step(cmds_step);
            ip += n;
        }
        assert_eq!(ip, fusion_cmds.len());
        assert_eq!(sim.bots.len(), 0);
        let last_cmds = log_cmds.pop();
        assert_eq!(last_cmds, Some(vec![Command::Halt]));
    }
    let mut bots = log_bots.pop().unwrap(); //.into_iter().collect();

    let mut fission_cmds = Vec::new();
    let mut sim = sim::SimState::new(filled.len(), to.len());

    while let Some(cmds) = log_cmds.pop() {
        let next_bots = bots;
        bots = log_bots.pop().unwrap(); //.into_iter().collect();
        // eprintln!("fusion: {:?} <- {:?}", next_bots, bots);
        // eprintln!("fission: {:?} -> ", sim.bots);

        let mut back_cmds = BTreeMap::new();
        for (bot, &cmd) in bots.iter().zip(cmds.iter()) {
            // if let Command::FusionS(nd) = cmd {
            //     continue;
            // }
            if let Some(next_bot) = next_bots.iter().find(|&b| b.bid == bot.bid) {
                let back_bot = sim.bots.iter().find(|&b| b.p == next_bot.p).unwrap();
                let back_cmd = match cmd {
                    Command::Wait => Command::Wait,
                    Command::SMove(d) => Command::SMove(-d),
                    Command::LMove(d1, d2) => Command::LMove(-d2, -d1),
                    Command::FusionP(nd) => Command::Fission(nd, next_bot.seeds.len() - bot.seeds.len() - 1),
                    _ => panic!(),
                };
                back_cmds.insert(back_bot.bid, back_cmd);
            }
        }
        let back_cmds: Vec<_> = back_cmds.values().cloned().collect();
        fission_cmds.append(&mut back_cmds.clone());
        sim.step(back_cmds);
    }

    let mut bids = Vec::new();
    for &pos in to.iter() {
        let bot = sim.bots.iter().find(|&b| b.p == pos).unwrap();
        bids.push(bot.bid);
    }
    eprintln!("fission: finished!");
    (bids, fission_cmds)
}

pub mod bfs;
pub mod command;
pub use command::Command;
pub mod sim;
pub mod postproc;
pub mod destruction;
pub mod occupy;
pub use occupy::*;
pub mod xz;
