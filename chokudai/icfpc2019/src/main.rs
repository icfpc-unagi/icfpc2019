
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
    let first_state = get_first_state(first_field, first_itemfield, FX, FY);

    let mut best_size = 99999999;
    let mut best_second_state: State = first_state.clone();
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
        let mut final_action = make_action_by_state(&second_state, &ChokudaiOptions::chokudai());
        let (flag, act) = optimization_actions(
            &second_state,
            &final_action,
            1,
            &ChokudaiOptions::chokudai(),
        );
        if flag {
            final_action = act;
        }

        let f2_action = optimize_pure_move(
            &second_state.field,
            &second_state.item_field,
            &second_state.p,
            &final_action,
        );

        let size = (initialMove.1).len() + f2_action.len();
        eprintln!("add: {} size: {}", loop_cnt - 1, size);
        if best_size > size && size != 0 {
            best_size = size;
            best_pre_action = initialMove.1;
            best_second_state = second_state.clone();
            best_ans_action = f2_action.clone();
        }
    }

    //let mut final_action = make_action_by_state(&best_second_state, &ChokudaiOptions::default());
    let (flag, act) = optimization_actions(
        &best_second_state,
        &best_ans_action,
        2,
        &ChokudaiOptions::chokudai(),
    );
    best_ans_action = act;

    let mut best_action: Vec<Action> = Vec::with_capacity(0);
    for act in &best_pre_action {
        best_action.push(*act);
    }
    for act in &best_ans_action {
        best_action.push(*act);
    }

    best_size = best_action.len();
    eprintln!("Best: {}", best_size);


    //iwi opt
    let best2 = optimize_pure_move(
        &first_state.field,
        &first_state.item_field,
        &first_state.p,
        &best_action,
    );


    best_size = best2.len();

    let mut best_string = actions_to_string(&best2);


    eprintln!("Best: {}", best_size);
    println!("{}", best_string);
}
