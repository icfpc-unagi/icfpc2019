use crate::*;

pub fn reverse_actions(actions: &[Action]) -> Vec<Action> {
    let mut rev_actions: Vec<_> = actions.into_iter().collect();
    rev_actions.reverse();

    rev_actions.iter().map(
        |action| {
            match action {
                Action::TurnR => Action::TurnL,
                Action::TurnL => Action::TurnR,
                Action::Move(dir) => Action::Move((dir + 2) % 4),
                _ => panic!("irreversible action: {:?}", action),
            }
        }
    ).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (mut map, mut booster, init_x, init_y) = load_task_002();
        let mut worker = WorkerState::new3(init_x, init_y, &mut map, &mut booster);
        // let bfs = BFS::new(a)

        let actions = vec![
            Action::Move(0),
            Action::TurnR,
            Action::Move(0),
            Action::TurnL,
        ];

        let rev_actions = reverse_actions(&actions);
        dbg!(&rev_actions);
    }
}
