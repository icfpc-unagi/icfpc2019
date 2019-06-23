
use chokudai::*;
use common::*;

fn main() {
    let taskfile = std::env::args().nth(1).expect("usage: args[1] = taskfile");
    let initialType = std::env::args().nth(2).unwrap_or("migimae".to_owned());

    let t = read_task(&taskfile);

    let (first_field, first_itemfield, FX, FY) = t.clone();
    let H = first_field.len();
    let W = first_field[0].len();
    let default_field = first_field.clone();

    let mut best_size = 99999999;
    let mut best_second_state: State = get_first_state(first_field, first_itemfield, FX, FY);
    let mut best_pre_action: Vec<Action> = Vec::with_capacity(0);
    let mut best_ans_action: Vec<Action> = Vec::with_capacity(0);

    let loop_first = 0;
    let mut loop_cnt = loop_first;


    loop {
        let mut initialMove;
        if initialType == "migimae" {
            initialMove = bootstrap_expand_1_migimae(&t, loop_cnt);
        } else {
            initialMove = bootstrap_expand_2_migi(&t, loop_cnt);
        }

        if loop_cnt + 4 > (initialMove.2).manipulators.len() && loop_cnt != loop_first {
            //println!("{}", (initialMove.2).manipulators.len());
            break;
        }
        loop_cnt += 1;

        //let first_state = get_first_state(first_field, first_itemfield, FX, FY);

        //途中で塗られたものを使用するバージョン
        //let mut second_state = get_first_state((initialMove.0).0, (initialMove.0).1, (initialMove.2).x, (initialMove.2).y);
        //途中で塗られたものを使用しないバージョン
        let mut second_state = get_first_state(
            default_field.clone(),
            (initialMove.0).1,
            (initialMove.2).x,
            (initialMove.2).y,
        );
        //eprintln!("{}", second_state.p.manipulators.len());
        second_state.p.manipulators = (initialMove.2).manipulators;
        //eprintln!("{}", second_state.p.manipulators.len());

        //let mut final_action = make_action_by_state(&first_state, 1);
        let mut final_action = make_action_by_state(&second_state, 1);
        let (flag, act) = optimization_actions(&second_state, &final_action, 5);
        if flag {
            final_action = act;
        }

        let size = (initialMove.1).len() + final_action.len();
        eprintln!("add: {} size: {}", loop_cnt - 1, size);
        if best_size > size && size != 0 {
            best_size = size;
            best_pre_action = initialMove.1;
            best_second_state = second_state.clone();
            best_ans_action = final_action.clone();
        }

    }
    let mut final_action = make_action_by_state(&best_second_state, 1);
    let (flag, act) = optimization_actions(&best_second_state, &best_ans_action, 30);
    if flag {
        best_ans_action = act;
    }

    let pre_string = actions_to_string(&best_pre_action);
    let ans_string = actions_to_string(&best_ans_action);
    best_size = best_pre_action.len() + best_ans_action.len();

    let mut best_string = pre_string.to_string() + &ans_string;
    eprintln!("Best: {}", best_size);
    println!("{}", best_string);
}
