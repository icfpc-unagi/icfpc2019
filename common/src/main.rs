use common::*;

fn main() {
    let t = read_task("/Users/akiba/Downloads/part-1-initial/prob-001.desc");
    //let t = read_task("/Users/akiba/Downloads/part-2-teleports/prob-151.desc");

    let a = PlayerState {
        x: t.2,
        y: t.3,
        dir: 0,
        unused_boosters: vec![],
        active_boosters: vec![],
        manipulators: vec![(1, 0), (1, 1), (1, -1)],
    };

    let mut bfs = BFS::new(&t.0);
    let actions = bfs.search_fewest_actions_to_move(&a, 2, 2);
    dbg!(&actions);

    let actions = bfs.search_fewest_actions_to_move(&a, 3, 3);
    dbg!(&actions);

    let actions = bfs.search_fewest_actions_to_move(&a, 2, 2);
    dbg!(&actions);
}
