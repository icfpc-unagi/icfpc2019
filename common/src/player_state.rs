use crate::*;
use std::vec::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerState {
    pub x: usize, //・今いる座標
    pub y: usize,
    pub dir: usize,                    //・向いている向き
    pub time: usize,                   // 経過時間
    pub unused_boosters: Vec<Booster>, //・持っている
    // TODO: Flatten?
    pub active_boosters: Vec<(Booster, usize)>, //・発動中の効果、期限 (この時間に無効になる)
    pub manipulators: Vec<(i32, i32)>,          // マニピュレータたちの相対位置（方向0のときの）
}

impl PlayerState {
    pub fn new(x: usize, y: usize) -> PlayerState {
        PlayerState {
            x,
            y,
            dir: 0,
            time: 0,
            unused_boosters: vec![],
            active_boosters: vec![],
            manipulators: vec![(1, 0), (1, 1), (1, -1)],
        }
    }

    pub fn apply_action(&mut self, action: Action) {
        match action {
            Action::Move(dir) => {
                // TODO: Fastがonだったら2マス移動したりする
                let pos = apply_move((self.x, self.y), dir);
                if self
                    .active_boosters
                    .iter()
                    .find(|x| x.0 == Booster::Fast)
                    .is_some()
                {
                    let pos = apply_move((pos.0, pos.1), dir);
                    self.x = pos.0;
                    self.y = pos.1;
                } else {
                    self.x = pos.0;
                    self.y = pos.1;
                }
            }
            Action::Nothing => (),
            Action::TurnR => {
                self.dir += 1;
                self.dir %= 4;

                for m in self.manipulators.iter_mut() {
                    let p = *m;
                    m.0 = p.1;
                    m.1 = -p.0;
                }
            }
            Action::TurnL => {
                self.dir += 3;
                self.dir %= 4;

                for m in self.manipulators.iter_mut() {
                    let p = *m;
                    m.0 = -p.1;
                    m.1 = p.0;
                }
            }
            Action::Extension(dx, dy) => self.manipulators.push((dx, dy)),
            Action::Fast => {
                // TODO: Stop shifting
                self.active_boosters.push((
                    self.unused_boosters.remove(
                        self.unused_boosters
                            .iter()
                            .position(|&x| x == Booster::Fast)
                            .expect("no Fast remaining"),
                    ),
                    self.time + 50,
                ));
            }
            Action::Drill => {
                // TODO: Stop shifting
                self.active_boosters.push((
                    self.unused_boosters.remove(
                        self.unused_boosters
                            .iter()
                            .position(|&x| x == Booster::Drill)
                            .expect("no Drill remaining"),
                    ),
                    self.time + 30,
                ));
            }
            Action::Reset => unimplemented!(),
            Action::Teleport(x, y) => unimplemented!(),
        }
        self.time += 1;
        self.active_boosters = self
            .active_boosters
            .iter()
            .filter(|&x| x.1 < self.time)
            .cloned()
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = PlayerState::new(10, 20);
        let mut b = a.clone();
        b.apply_action(Action::Move(0));
        assert_eq!(
            b,
            PlayerState {
                time: 1,
                x: 11,
                ..a.clone()
            }
        );
        // dbg!(b);

        let mut b = a.clone();
        b.apply_action(Action::TurnR);
        assert_eq!(
            b,
            PlayerState {
                time: 1,
                dir: 1,
                manipulators: vec![(0, -1,), (1, -1,), (-1, -1,),],
                ..a.clone()
            }
        );
        // dbg!(b);
    }
}
