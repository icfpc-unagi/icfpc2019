use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerState {
    pub x: usize, //・今いる座標
    pub y: usize,
    pub dir: usize,                           //・向いている向き
    pub unused_boosters: Vec<Booster>,        //・持っている
    pub active_boosters: Vec<(Booster, i32)>, //・発動中の効果、残りターン
    pub manipulators: Vec<(i32, i32)>,        // マニピュレータたちの相対位置（方向0のときの）
}

impl PlayerState {
    pub fn new_initial(x: usize, y: usize) -> PlayerState {
        PlayerState {
            x,
            y,
            dir: 0,
            unused_boosters: vec![],
            active_boosters: vec![],
            manipulators: vec![(1, 0), (1, 1), (1, -1)],
        }
    }

    pub fn apply_action(&mut self, action: Action) {
        match action {
            Action::Move(dir) => {
                // TODO: Fastがonだったら2マス移動したりする
                let (x, y) = apply_move((self.x, self.y), dir);
                self.x = x;
                self.y = y;
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
            Action::Fast =>
            // TODO: unused_boostersからfastを除いてactive_boostersに追加
            {
                unimplemented!()
            }
            Action::Drill =>
            // TODO: マップに対する作用は一体？？？
            {
                unimplemented!()
            }
            Action::Reset => unimplemented!(),
            Action::Teleport(x, y) => unimplemented!(),
        }
        // TODO: 発動中の効果の有効期限を減らしたりする
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = PlayerState {
            x: 10,
            y: 20,
            dir: 0,
            unused_boosters: vec![],
            active_boosters: vec![],
            manipulators: vec![(1, 0), (1, 1), (1, -1)],
        };

        let mut b = a.clone();
        b.apply_action(Action::Move(0));
        assert_eq!(b, PlayerState { x: 11, ..a.clone() });
        // dbg!(b);

        let mut b = a.clone();
        b.apply_action(Action::TurnR);
        assert_eq!(
            b,
            PlayerState {
                dir: 1,
                manipulators: vec![(0, -1,), (1, -1,), (-1, -1,),],
                ..a.clone()
            }
        );
        // dbg!(b);
    }
}
