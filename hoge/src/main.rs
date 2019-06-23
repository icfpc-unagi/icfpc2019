fn main() {
    let task = std::env::args().nth(1).expect("task");
    let sol = std::env::args().nth(2).expect("sol");

    let task = common::read_task(&task);
    let sol = common::read_sol(&sol);
    assert_eq!(sol.len(), 1);
    let original_actions = &sol[0];

    let optimized_actions = common::optimize_pure_move(&task, original_actions);
    //println!("{} -> {}", original_actions.len(), optimized_actions.len());
    println!("{}", common::actions_to_string(&optimized_actions));
}
