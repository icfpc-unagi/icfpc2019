fn main() {
    let task = std::env::args().nth(1).expect("task");
    let sol = std::env::args().nth(2).expect("sol");

    let task = common::read_task(&task);
    let sol = common::read_sol(&sol);
    assert_eq!(sol.len(), 1);
    let original_actions = &sol[0];

    let optimized_actions = common::optimize_pure_move_old(&task, original_actions);
    let (n1, n2) = (original_actions.len(), optimized_actions.len());
    println!(
        "{} -> {} ({:.2}%)",
        n1, n2,
        ((n1 - n2) as f32) / (n1 as f32) * 100.0
    );
    //println!("{}", common::actions_to_string(&optimized_actions));
}
