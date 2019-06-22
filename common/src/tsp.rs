use crate::*;

// startから開始して、targetをぜんぶ回収して、is_goalを満たすマスまで移動する
pub fn tsp<F: Fn(usize, usize) -> bool>(
    map: Vec<Vec<Square>>,
    start: (usize, usize),
    mut targets: Vec<(usize, usize)>,
    is_goal_func: F,
) -> (Vec<Action>, (usize, usize)) {
    /*
    let (xsize, ysize) = xysize(map);
    let bfs = BFS::new(xsize, ysize);

    targets.push(start);
    let n = targets.len();

    let cost_to_goal = (0..n).map(|i|

    )
    */

    unimplemented!();
}
