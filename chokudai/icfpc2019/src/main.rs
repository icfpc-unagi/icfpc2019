use common::*;
use chokudai::*;

fn main() {
    let taskfile = std::env::args().nth(1).expect("usage: args[1] = taskfile");
    let t = read_task(&taskfile);

    let mut best_string = "".to_string();
    let mut best_size = 99999999;

    let mut loop_cnt = 0;

    loop{
        let initialMove = bootstrap_expand_1_migimae(&t, loop_cnt);
        if loop_cnt + 4 > (initialMove.2).manipulators.len() {
            break;
        }
        loop_cnt += 1;

        let (first_field, first_itemfield, FX, FY) = t.clone();
        let H = first_field.len();
        let W = first_field[0].len();
        let default_field = first_field.clone();

        let first_state = get_first_state(first_field, first_itemfield, FX, FY);
        
        //途中で塗られたものを使用するバージョン
        //let mut second_state = get_first_state((initialMove.0).0, (initialMove.0).1, (initialMove.2).x, (initialMove.2).y);
        //途中で塗られたものを使用しないバージョン
        let mut second_state = get_first_state(default_field, (initialMove.0).1, (initialMove.2).x, (initialMove.2).y);
        eprintln!("{}", second_state.p.manipulators.len());
        second_state.p.manipulators = (initialMove.2).manipulators;
        eprintln!("{}", second_state.p.manipulators.len());

        //let mut final_action = make_action_by_state(&first_state, 1);
        let mut final_action = make_action_by_state(&second_state, 1);
        
        let pre_string = actions_to_string(&initialMove.1);
        let ans_string = actions_to_string(&final_action);

        let size =  (initialMove.1).len() + final_action.len();
        eprintln!("add: {} size: {}", loop_cnt, size);
        if best_size > size{
            best_string = pre_string + &ans_string;
            best_size = size;
        }

    }

    println!("{}", best_string);
}
