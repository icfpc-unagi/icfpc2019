use common::*;
use common::reach::*;
use common::bfs::*;

pub struct State{
    pub p: PlayerState,     //プレイヤー情報
    pub field: Vec<Vec<Square>>,    //壁情報
    pub item_field: Vec<Vec<Option<Booster>>>,   //アイテム情報
}

///初期Stateを作るための関数
fn get_first_state(field: Vec<Vec<Square>>, item_field: Vec<Vec<Option<Booster>>>, fx: usize, fy: usize) -> State{
    State{
        p: PlayerState{
            x: fx,
            y: fy,
            dir: 0,
            time: 0,
            unused_boosters: Vec::with_capacity(0),
            active_boosters: Vec::with_capacity(0),
            manipulators: vec![(0, 0), (1, 1), (1, 0), (1, -1)],
        },
        field: field,
        item_field: item_field,
    }
}

///回る順番を順番に
fn make_easy_target_list(S: &State, H: usize, W:usize) -> Vec<(usize, usize)>{

    let mut ans: Vec<(usize, usize)> = Vec::with_capacity(0);

    let mut start_point = (!0, !0);
    let mut dist = 999999;

    //'a: for x in 0..H{
    //    for y in 0..W {
    //        if S.field[x][y] == Square::Empty {
    //            start_point = (x, y);
    //            break 'a;
    //        }
    //    }
    //}

    for x in 0..H{
        for y in 0..W {
            if S.field[x][y] == Square::Empty {
                let tdist = get_diff(x, S.p.x) + get_diff(y, S.p.y);
                if tdist >= dist{
                    continue;
                }
                if dist == 999999 {
                    start_point = (x, y);
                }
                for k in 0..4 {
                    let (nx, ny) = apply_move((x, y), k);
                    if S.field[nx][ny] != Square::Empty {
                        start_point = (x, y);
                        dist = tdist;
                        break;
                    }
                }
            }
        }
    }

    if start_point.0 != !0 {

        //println!("{} {}", start_point.0, start_point.1);

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
            while loop_cnt < 1000{
                loop_cnt += 1;
                //println!("({}, {}), {}", current_point.0, current_point.1, current_dir);
                
                //loop check
                if current_dir == first_dir && current_point == start_point {
                    if first{
                        first = false;
                    }
                    else{
                        break;
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
                        current_dir = (nk - 1) % 4;
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

fn get_diff(a:usize, b:usize) -> usize{
    if a > b{
        return a - b;
    }
    b - a
}

fn main() {
    let taskfile = std::env::args().nth(1).expect("usage: args[1] = taskfile");
    let (mut field, mut itemfield, mut FH, mut FW) = read_task(&taskfile);
    let H = field.len();
    let W = field[0].len();

    let first_state = get_first_state(field, itemfield, FH, FW);

    
    let mut final_action: Vec<Action> = Vec::with_capacity(0);
    let mut current_state = first_state;
    
    let t = read_task(&taskfile);
    let mut bfs = BFS::new(H, W);

    loop{
        let point_list = make_easy_target_list(&current_state, H, W);
        if point_list.len() == 0{
            break;
        }


        for dxy in &current_state.p.manipulators{
            if is_visible(&(current_state.field), (current_state.p.x, current_state.p.y), *dxy){
                current_state.field[current_state.p.x + (*dxy).0 as usize][current_state.p.y + (*dxy).1 as usize] = Square::Filled;
            }
        }

        for i in 0..point_list.len() {
            let target_pos = point_list[i];

            //println!("{} {}", target_pos.0, target_pos.1);

            //塗り済みであるかの検出
            if current_state.field[target_pos.0][target_pos.1] != Square::Empty{
                continue;
            }

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
                                pos = (target_pos.0 - 1, target_pos.1 - 1);
                                d = 0;
                            }
                            else {
                                pos = (target_pos.0 + 1, target_pos.1 + 1);
                                d = 2;
                            }
                        }
                        else {
                            if target_pos.1 < next_target_pos.1 {
                                pos = (target_pos.0 + 1, target_pos.1 - 1);
                                d = 3;
                            }
                            else {
                                pos = (target_pos.0 - 1, target_pos.1 + 1);
                                d = 1;
                            }
                        }
                        if current_state.field[pos.0][pos.1] != Square::Block {
                            use_double_position = (pos, d);
                        }
                    }
                }
            }
            
            
            let mut actions: Vec<Action> = Vec::with_capacity(0);
            if use_double_position.1 != !0 {
                println!("double at ({}, {}) for ({}, {})", (use_double_position.0).0 , (use_double_position.0).1,target_pos.0, target_pos.1);
                println!("now : {} {}", current_state.p.x, current_state.p.y);
                actions = bfs.search_fewest_actions_to_move(&t.0, &current_state.p, (use_double_position.0).0, (use_double_position.0).1);
                let now_dir = current_state.p.dir;
                if (now_dir + 1) % 4 == use_double_position.1{
                    actions.push(Action::TurnR);
                }
                else if (now_dir + 2) % 4 == use_double_position.1{
                    actions.push(Action::TurnR);
                    actions.push(Action::TurnR);
                }
                else if (now_dir + 3) % 4 == use_double_position.1{
                    actions.push(Action::TurnL);
                }
            }
            else{
                println!("single at ({}, {})", target_pos.0, target_pos.1);
                println!("now : {} {}", current_state.p.x, current_state.p.y);

                actions = bfs.search_fewest_actions_to_move(&t.0, &current_state.p, target_pos.0, target_pos.1);
                //actions = bfs.search_fewest_actions_to_wrap(&current_state.p, target_pos.0, target_pos.1);

            }

            for act in actions
            {
                current_state.p.apply_action(act);
                final_action.push(act);
                for dxy in &current_state.p.manipulators{
                    if is_visible(&(current_state.field), (current_state.p.x, current_state.p.y), *dxy){
                        current_state.field[current_state.p.x + (*dxy).0 as usize][current_state.p.y + (*dxy).1 as usize] = Square::Filled;
                    }
                }
            }
        }
        
    }

    let ans_string = actions_to_string(&final_action);
    println!("{}", ans_string);
}
