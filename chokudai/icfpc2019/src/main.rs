use common::*;
use chokudai::*;

fn main() {
    let taskfile = std::env::args().nth(1).expect("usage: args[1] = taskfile");
    let t = read_task(&taskfile);

    let initialMove = bootstrap_expand(&t);
    
    let (first_field, first_itemfield, FX, FY) = t;
    let H = first_field.len();
    let W = first_field[0].len();

    let first_state = get_first_state(first_field, first_itemfield, FX, FY);
    
    let mut final_action = make_action_by_state(&first_state, true);

    let ans_string = actions_to_string(&final_action);
    println!("{}", ans_string);
}
