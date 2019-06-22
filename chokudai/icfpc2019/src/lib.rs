use common::*;

use std::time::{Duration, Instant};
use std::thread::sleep;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State{
    pub p: WorkerState,     //プレイヤー情報
    pub field: Vec<Vec<Square>>,    //壁情報
    pub item_field: Vec<Vec<Option<Booster>>>,   //アイテム情報
}

///初期Stateを作るための関数
pub fn get_first_state(mut field: Vec<Vec<Square>>, item_field: Vec<Vec<Option<Booster>>>, fx: usize, fy: usize) -> State{
    State{
        p: WorkerState::new2(fx, fy, &mut field),
        field: field,
        item_field: item_field,
    }
}

fn make_list_with_startpoint(S: &State, H: usize, W: usize, start_point: (usize, usize)) -> Vec<(usize, usize)>{

    let mut ans: Vec<(usize, usize)> = Vec::with_capacity(0);
    if start_point.0 != !0 {

        //println!("start_point {} {}", start_point.0, start_point.1);

        let mut first_dir = !0;
        for k in 0..4 {
            let (nx, ny) = apply_move(start_point, k);
            if S.field[nx][ny] != Square::Empty {
                first_dir = k;
                break;
            }
        }
        
        let mut current_point = start_point;    //今いる場所
        let mut current_dir = first_dir;    //壁の向き
        let mut first = true;


        if first_dir == !0 {
            ans.push(start_point);
        }
        else{
            let mut loop_cnt = 0;
            let maxloop = 2500;
            let mut loopbit = 1 << first_dir;

            while loop_cnt < maxloop{
                loop_cnt += 1;
                //println!("({}, {}), {}", current_point.0, current_point.1, current_dir);
                
                //loop check
                if current_point == start_point {
                    if first{
                        first = false;
                    }
                    else if ((loopbit >> current_dir) & 1) == 1{
                        break;
                    }
                    else {
                        loopbit |= (1 << current_dir);
                    }
                }

                //add
                if S.field[current_point.0][current_point.1] == Square::Empty{
                    ans.push(current_point);
                }

                let mut ok = false;
                //move
                for k in 0..4 {
                    let nk = (current_dir + k) % 4;
                    let next_point = apply_move(current_point, nk);
                    if S.field[next_point.0][next_point.1] == Square::Empty {
                        current_point = next_point;
                        current_dir = (nk + 3) % 4;
                        ok = true;
                        break;
                    }
                }
                if !ok{
                    break;
                }
            }
        }
    }
    ans
}

pub fn make_simple_target_list(S: &State, H: usize, W:usize) -> Vec<(usize, usize)>{
    
    let mut start_point = (!0, !0);
    let mut dist = 999999;

    for x in 0..H{
        for y in 0..W {
            if S.field[x][y] == Square::Empty {
                let tdist = get_diff(x, S.p.x) + get_diff(y, S.p.y);
                if dist > tdist {
                    start_point = (x, y);
                    dist = tdist;
                }
            }
        }
    }
    make_list_with_startpoint(S, H, W, start_point)
}

///State
pub fn make_easy_target_list(S: &State, H: usize, W:usize, T: &Vec<Vec<usize>>, UseOptimization: usize) -> (usize, Vec<(usize, usize)>){


    let mut start_point = (!0, !0);
    let mut dist = 999999;
    let mut lastAction = 0;

    use rand:: Rng;
    let mut rng = rand::thread_rng();


    for x in 0..H{
        for y in 0..W {
            if S.field[x][y] == Square::Empty {

                let tdist = get_diff(x, S.p.x) + get_diff(y, S.p.y);
                if lastAction == 0 && dist > tdist {
                    start_point = (x, y);
                    dist = tdist;
                }

                for k in 0..4 {
                    let (nx, ny) = apply_move((x, y), k);
                    if  T[nx][ny] != !0 && lastAction < T[nx][ny]{
                        start_point = (x, y);
                        dist = tdist;
                        lastAction = T[nx][ny];
                        
                    }
                    if UseOptimization == 2 && T[nx][ny] != !0 && rng.gen::<usize>() % 2 == 1{
                        start_point = (x, y);
                        dist = tdist;
                        lastAction = T[nx][ny];
                    }
                }
            }
        }
    }

    let ans = make_list_with_startpoint(S, H, W, start_point);
    (lastAction, ans)
}

pub fn get_diff(a:usize, b:usize) -> usize{
    if a > b{
        return a - b;
    }
    b - a
}


pub fn make_move(a2 :&Vec<Action>, R: usize, L: usize, d: usize) -> Vec<Action>{
    let mut stockR = R;
    let mut stockL = L;
    let mut now_dir = d;

    let mut actions: Vec<Action> = Vec::with_capacity(0);

    
    for act in a2{
        if *act == Action::TurnR{
            stockR += 1;
        }
        if *act == Action::TurnL{
            stockL += 1;
        }
    }

    if stockR >= 1 && stockL >= 1{
        println!("!?");
    }

    for act in a2{
        match &act{
            Action::Move(d) =>{
                if *d == (now_dir + 1) % 4 {
                    if stockR >= 1 {
                        stockR -= 1;
                        actions.push(Action::TurnR);
                        now_dir = (now_dir + 1) % 4;
                    }
                }
                else if *d == (now_dir + 3) % 4{
                    if stockL >= 1 {
                        stockL -= 1;
                        actions.push(Action::TurnL);
                        now_dir = (now_dir + 3) % 4;
                    }
                    else if stockR == 2 || stockL == 2 {
                        stockR = 0;
                        stockL = 1;
                        actions.push(Action::TurnL);
                        now_dir = (now_dir + 3) % 4;
                    }
                }
                else if *d == (now_dir + 2) % 4{
                    if stockR >= 2{
                        stockR -= 2;
                        actions.push(Action::TurnR);
                        actions.push(Action::TurnR);
                        now_dir = (now_dir + 2) % 4;
                    }
                }
                

                actions.push(*act);
            },
            _ =>{

            }
        }
    }

    while stockR > 0 {
        actions.push(Action::TurnR);
        stockR -= 1;
    }
    while stockL > 0 {
        actions.push(Action::TurnL);
        stockL -= 1;
    }
    actions
}

const optimization_num: usize = 2; //0..OptimizationNum

pub fn make_action_by_state(first_state: &State, UseOptimization: usize) -> Vec<Action>
{
    let H = first_state.field.len();
    let W = first_state.field[0].len();

    let FX = first_state.p.x;
    let FY = first_state.p.y;

    let mut bfs = BFS::new(H, W);

    let mut final_action :Vec<Action> = Vec::with_capacity(0);

    loop{
        //eprintln!("start!");
        
        let mut current_state = first_state.clone();

        
        let mut LastActionTable = vec![vec![!0; W]; H];
        
        LastActionTable[FX][FY] = 0;
        current_state.field[FX][FY] = Square::Filled;

        let mut action_cnt = 0;
        for act in &final_action
        {
            action_cnt += 1;
            let upd = apply_action(*act, &mut current_state.p, &mut current_state.field, &mut current_state.item_field);
            for p in upd.filled {
                LastActionTable[p.0][p.1] = action_cnt;
                //eprintln!("{}", action_cnt);
            }
        }

        let mut empty_cell = 0;
        for x in 0..H {
            for y in 0..W {
                if current_state.field[x][y] == Square::Empty {
                    empty_cell += 1;
                }
            }
        }
        //eprintln!("empty : {}  Len : {}", empty_cell, final_action.len());


        let (mut last_act, point_list) = make_easy_target_list(&current_state, H, W, &LastActionTable, UseOptimization);
        
        //eprintln!("List : {}", point_list.len());
        if(point_list.len() == 0){
            break;
        }
        if last_act > final_action.len() - 5 {
            last_act = final_action.len();
        }

        //途中状態まで移動する
        let mut current_state = first_state.clone();

        //eprintln!("current {} {} {}", current_state.p.x, current_state.p.y, current_state.p.dir);
        //eprintln!("first {} {} {}", first_state.p.x, first_state.p.y, first_state.p.dir);

        for dxy in &current_state.p.manipulators{
            if is_visible(&(current_state.field), (current_state.p.x, current_state.p.y), *dxy){
                //current_state.field[current_state.p.x + (*dxy).0 as usize][current_state.p.y + (*dxy).1 as usize] = Square::Filled;
            }
        }

        let mut temp_action: Vec<Action> = Vec::with_capacity(0);

        for i in 0..last_act {
            let act = final_action[i];
            temp_action.push(act);
            apply_action(act, &mut current_state.p, &mut current_state.field, &mut current_state.item_field);
        }


        //復帰するべき状態
        let back_state = current_state.p.clone();

        for i in 0..point_list.len() {
            let target_pos = point_list[i];

            //eprintln!("check: {} {}", target_pos.0, target_pos.1);

            //塗り済みであるかの検出
            if current_state.field[target_pos.0][target_pos.1] != Square::Empty{
                //eprintln!("skip");
                continue;
            }

            let mut actions: Vec<Action> = Vec::with_capacity(0);

            if UseOptimization == 0 {
                let (a2, gx, gy) = bfs.search_fewest_actions_to_wrap(&current_state.field, &current_state.p, target_pos.0, target_pos.1);
                    
                actions = a2;
            }
            else if UseOptimization == 1 ||UseOptimization == 2{
                //２連塗チェック
                let mut use_double_position = ((!0, !0), !0);
                
                if i != point_list.len() - 1 {
                    let next_target_pos = point_list[i+1];
                    if current_state.field[next_target_pos.0][next_target_pos.1] == Square::Empty{
                        let diff = get_diff(target_pos.0, next_target_pos.0) + get_diff(target_pos.1, next_target_pos.1);
                        if diff == 1 {
                            let mut pos = (!0, !0);
                            let mut d = !0;
                            if target_pos.0 != next_target_pos.0 {
                                if target_pos.0 < next_target_pos.0 {
                                    if  current_state.field[target_pos.0][target_pos.1 - 1] != Square::Block {
                                        pos = (target_pos.0 - 1, target_pos.1 - 1);
                                        d = 0;
                                    }
                                }
                                else {
                                    if  current_state.field[target_pos.0][target_pos.1 + 1] != Square::Block {
                                        pos = (target_pos.0 + 1, target_pos.1 + 1);
                                        d = 2;
                                    }
                                }
                            }
                            else {
                                if target_pos.1 < next_target_pos.1 {
                                    if current_state.field[target_pos.0][target_pos.1 + 1] != Square::Block {
                                        pos = (target_pos.0 + 1, target_pos.1 - 1);
                                        d = 3;
                                    }
                                }
                                else {
                                    if current_state.field[target_pos.0][target_pos.1 - 1] != Square::Block {
                                        pos = (target_pos.0 - 1, target_pos.1 + 1);
                                        d = 1;
                                    }
                                }
                            }
                            if d != !0 && current_state.field[pos.0][pos.1] != Square::Block {
                                use_double_position = (pos, d);
                            }
                        }
                    }
                }
                
                
                if use_double_position.1 != !0 && point_list.len() >= 4 {
                    //eprintln!("double at ({}, {}, {}) for ({}, {})", (use_double_position.0).0 , (use_double_position.0).1, use_double_position.1,target_pos.0 , target_pos.1);
                    //eprintln!("now : {} {} {}", current_state.p.x, current_state.p.y, current_state.p.dir);
                    let mut a2 = bfs.search_fewest_actions_to_move(&current_state.field, &current_state.p, (use_double_position.0).0, (use_double_position.0).1);
                    let mut now_dir = current_state.p.dir;
                    let (a3, gx, gy) = bfs.search_fewest_actions_to_wrap(&current_state.field, &current_state.p, target_pos.0, target_pos.1);
                    
                    if a2.len() < a3.len() + 20 {
                        
                        let mut stockR = 0;
                        let mut stockL = 0;

                        if (now_dir + 1) % 4 == use_double_position.1{
                            stockR = 1;
                            //actions.push(Action::TurnR);
                        }
                        else if (now_dir + 2) % 4 == use_double_position.1{
                            //actions.push(Action::TurnR);
                            //actions.push(Action::TurnR);
                            stockR = 2;
                        }
                        else if (now_dir + 3) % 4 == use_double_position.1{
                            stockL = 1;
                            //actions.push(Action::TurnL);
                        }

                        actions = make_move(&a2, stockR, stockL, now_dir);

                    }
                    else{
                        
                        actions = make_move(&a3, 0, 0, now_dir);
                        //actions = a3;
                    }
                }
                else{
                    //eprintln!("single at ({}, {})", target_pos.0, target_pos.1);
                    //eprintln!("now : {} {}", current_state.p.x, current_state.p.y);

                    //actions = bfs.search_fewest_actions_to_move(&t.0, &current_state.p, target_pos.0, target_pos.1);
                    let (a2, gx, gy) = bfs.search_fewest_actions_to_wrap(&current_state.field, &current_state.p, target_pos.0, target_pos.1);
                    
                    actions = make_move(&a2, 0, 0, current_state.p.dir);

                    //let tmp_string = actions_to_string(&a2);
                    //actions = a2;
                    //println!("go: {}", tmp_string);
                }
            }

            'actloop: for act in actions
            {
                // apply_action で field と item_field も更新する
                let ret = apply_action(act, &mut current_state.p, &mut current_state.field, &mut current_state.item_field);
                temp_action.push(act);
                
                for (tx, ty) in ret.filled{
                    if tx == target_pos.0 && ty == target_pos.1{
                        break 'actloop;
                    }
                }
            }
        }

        
        //Actionを差し込む前の状態にちゃんと戻す
        if last_act != final_action.len() {
            let a2 = bfs.search_fewest_actions_to_move(&current_state.field, &current_state.p, back_state.x, back_state.y);
            
            for act in a2
            {
                // apply_action で field と item_field も更新する
                apply_action(act, &mut current_state.p, &mut current_state.field, &mut current_state.item_field);
                temp_action.push(act);
            }

            let now_dir = current_state.p.dir;
            
            let mut a3 :Vec<Action> = Vec::with_capacity(0);
            if (now_dir + 1) % 4 == back_state.dir{
                a3.push(Action::TurnR);
            }
            else if (now_dir + 2) % 4 == back_state.dir{
                a3.push(Action::TurnR);
                a3.push(Action::TurnR);
            }
            else if (now_dir + 3) % 4 == back_state.dir{
                a3.push(Action::TurnL);
            }
            
            for act in a3
            {
                // apply_action で field と item_field も更新する
                apply_action(act, &mut current_state.p, &mut current_state.field, &mut current_state.item_field);
                temp_action.push(act);
            }
        }

        //eprintln!("{} {} {}", current_state.p.x, current_state.p.y, current_state.p.dir);
        //eprintln!("{} {} {}", back_state.x, back_state.y, back_state.dir);

        //let mut current_state = first_state.clone();
        for i in last_act..final_action.len() {
            let act = final_action[i];
            temp_action.push(act);
            apply_action(act, &mut current_state.p, &mut current_state.field, &mut current_state.item_field);
            //eprintln!("{} {} {}", current_state.p.x, current_state.p.y, current_state.p.dir);
        }

        final_action = temp_action;
        //if final_action.len() > 10{
        //    break;
        //}
        
    }

    final_action
}

///返り値：成功フラグ、新しいAction列
pub fn shortening_actions(first_state: &State, actions: &Vec<Action>, Seconds: usize) -> (bool, Vec<Action>){

    let start = Instant::now();

    let minimum_range = 10;
    let maximum_range = 100;

    loop{

        let end = start.elapsed();
        let time = end.as_secs();
        if time >= Seconds as u64{
            break;
        }

        use rand:: Rng;
        let mut rng = rand::thread_rng();

        let action_range = rng.gen::<usize>() % (maximum_range - minimum_range + 1) + minimum_range;
        let start_action = rng.gen::<usize>() % (actions.len() - action_range);
        let end_action = start_action + action_range;

        let (flag, act) = shortening(&first_state, actions, start_action, end_action);

        if flag{
            return (true, act);
        }
    }
    
    (false, Vec::with_capacity(0))
}

fn shortening(first_state: &State, acts: &Vec<Action>, start:usize, end:usize) -> (bool, Vec<Action>){

    let H = first_state.field.len();
    let W = first_state.field[0].len();
    let actions = acts.clone();

    let mut start_state = first_state.clone();
    for i in 0..start {
        let act = actions[i];
        apply_action(act, &mut start_state.p, &mut start_state.field, &mut start_state.item_field);
    }
    let start_position = start_state.p.clone();
    let start_field = start_state.field.clone();
    let start_itemfield = start_state.item_field.clone();

    let mut current_state = start_state.clone();
    for i in start..end {
        let act = actions[i];
        apply_action(act, &mut current_state.p, &mut current_state.field, &mut current_state.item_field);
    }
    current_state.field = start_field;
    current_state.item_field = start_itemfield;
    let end_position = current_state.p.clone();

    for i in end..actions.len() {
        let act = actions[i];
        apply_action(act, &mut current_state.p, &mut current_state.field, &mut current_state.item_field);
    }

    let check_state = State{ 
        p: start_position.clone(),
        field: current_state.field.clone(),
        item_field: current_state.item_field.clone(),
    };

    let mut empty_cells = 0;
    let mut max_x = 0;
    let mut min_x = 999999;
    let mut max_y = 0;
    let mut min_y = 999999;
    
    for x in 0..H {
        for y in 0..W {
            if check_state.field[x][y] == Square::Empty{
                empty_cells += 1;
                if max_x < x { max_x = x; }
                if min_x > x { min_x = x; }
                if max_y < y { max_y = y; }
                if min_y > y { min_y = y; }
            }
        }
    }

    println!("RemoveRange : {}", end - start);
    println!("EmptyCell : {}", empty_cells);
    println!("range : ({}, {}) to ({}, {})", min_x, min_y, max_x, max_y);

    let x_move = max_x - min_x + (max_x - std::cmp::max(start_position.x, end_position.x) + (std::cmp::min(start_position.x, end_position.x) - min_x));
    let y_move = max_y - min_y + (max_y - std::cmp::max(start_position.y, end_position.y) + (std::cmp::min(start_position.y, end_position.y) - min_y));
    let mut need_to_move = x_move + y_move;
    if start_position.dir != end_position.dir{
        if start_position.dir == (end_position.dir + 2) % 4{
            need_to_move += 2;
        }
        else{
            need_to_move += 1;
        }
    }

    println!("needToMove : {}", need_to_move);

    (false, Vec::with_capacity(0))
}