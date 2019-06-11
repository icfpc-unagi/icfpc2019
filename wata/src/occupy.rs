use *;

pub fn set_occupied(bp: P, command: Command, occupied: &mut InitV3<bool>) {
	match command {
		Command::SMove(d) => {
			let len = d.mlen();
			let d = d / len;
			for i in 1..=len {
				let p = bp + d * i;
				assert!(!occupied[p]);
				occupied[p] = true;
			}
		},
		Command::LMove(d1, d2) => {
			let len1 = d1.mlen();
			let d1 = d1 / len1;
			let len2 = d2.mlen();
			let d2 = d2 / len2;
			for i in 1..=len1 {
				let p = bp + d1 * i;
				assert!(!occupied[p]);
				occupied[p] = true;
			}
			for i in 1..=len2 {
				let p = bp + d1 * len1 + d2 * i;
				assert!(!occupied[p]);
				occupied[p] = true;
			}
		},
		_ => {
		}
	}
}

pub fn check_occupied(bp: P, command: Command, occupied: &InitV3<bool>) -> bool {
	match command {
		Command::SMove(d) => {
			let len = d.mlen();
			let d = d / len;
			for i in 1..=len {
				let p = bp + d * i;
				if occupied[p] {
					return false;
				}
			}
		},
		Command::LMove(d1, d2) => {
			let len1 = d1.mlen();
			let d1 = d1 / len1;
			let len2 = d2.mlen();
			let d2 = d2 / len2;
			for i in 1..=len1 {
				let p = bp + d1 * i;
				if occupied[p] {
					return false;
				}
			}
			for i in 1..=len2 {
				let p = bp + d1 * len1 + d2 * i;
				if occupied[p] {
					return false;
				}
			}
		},
		_ => {
		}
	}
	true
}

