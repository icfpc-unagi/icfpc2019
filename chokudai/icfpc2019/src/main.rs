use common::*;
use chokudai::*;

fn main() {
    let taskfile = std::env::args().nth(1).expect("usage: args[1] = taskfile");
    let t = read_task(&taskfile);

    //let initialMove = bootstrap_expand(&t);
    
    let (first_field, first_itemfield, FX, FY) = t;
    let H = first_field.len();
    let W = first_field[0].len();
    let default_field = first_field.clone();

    let first_state = get_first_state(first_field, first_itemfield, FX, FY);
    
    //途中で塗られたものを使用するバージョン
    //let second_state = get_first_state((initialMove.0).0, (initialMove.0).1, (initialMove.2).x, (initialMove.2).y);
    //途中で塗られたものを使用しないバージョン
    //let second_state = get_first_state(default_field, (initialMove.0).1, (initialMove.2).x, (initialMove.2).y);
    
    let mut final_action = make_action_by_state(&first_state, 1);
    //let mut final_action = make_action_by_state(&second_state, true);
    
    //let pre_string = actions_to_string(&initialMove.1);
    let ans_string = actions_to_string(&final_action);

    //println!("{}{}", pre_string, ans_string);
    println!("{}", ans_string);
}
