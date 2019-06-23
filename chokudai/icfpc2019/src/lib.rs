use common::*;
use std::thread::sleep;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub p: WorkerState,                        //プレイヤー情報
    pub field: Vec<Vec<Square>>,               //壁情報
    pub item_field: Vec<Vec<Option<Booster>>>, //アイテム情報
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChokudaiOptions {
    pub OptType: usize,   //移動の最適化タイプ　0:貪欲 1: 連続した2手のみ先読み
    pub RandFlag: bool,   //true: 初期位置の採用方法のランダム導入（影響弱め）
    pub miningFlag: bool, //true: 直線移動の強化
}

impl Default for ChokudaiOptions {
    fn default() -> ChokudaiOptions {
        ChokudaiOptions {
            OptType: 1,
            RandFlag: false,
            miningFlag: false,
        }
    }
}

impl ChokudaiOptions {
    pub fn small() -> Vec<ChokudaiOptions> {
        vec![
            ChokudaiOptions {
                OptType: 1,
                RandFlag: false,
                miningFlag: false,
            },
            ChokudaiOptions {
                OptType: 1,
                RandFlag: false,
                miningFlag: true,
            },
        ]
    }

}

///初期Stateを作るための関数
pub fn get_first_state(
    mut field: Vec<Vec<Square>>,
    item_field: Vec<Vec<Option<Booster>>>,
    fx: usize,
    fy: usize,
) -> State {
    State {
        p: WorkerState::new2(fx, fy, &mut field),
        field: field,
        item_field: item_field,
    }
}

fn make_list_with_startpoint(
    S: &State,
    H: usize,
    W: usize,
    start_point: (usize, usize),
) -> Vec<(usize, usize)> {
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

        let mut current_point = start_point; //今いる場所
        let mut current_dir = first_dir; //壁の向き
        let mut first = true;


        if first_dir == !0 {
            ans.push(start_point);
        } else {
            let mut loop_cnt = 0;
            let maxloop = 2500;
            let mut loopbit = 1 << first_dir;

            while loop_cnt < maxloop {
                loop_cnt += 1;
                //println!("({}, {}), {}", current_point.0, current_point.1, current_dir);

                //loop check
                if current_point == start_point {
                    if first {
                        first = false;
                    } else if ((loopbit >> current_dir) & 1) == 1 {
                        break;
                    } else {
                        loopbit |= (1 << current_dir);
                    }
                }

                //add
                if S.field[current_point.0][current_point.1] == Square::Empty {
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
                if !ok {
                    break;
                }
            }
        }
    }
    ans
}

pub fn make_simple_target_list(S: &State, H: usize, W: usize) -> Vec<(usize, usize)> {
    let mut start_point = (!0, !0);
    let mut dist = 999999;

    for x in 0..H {
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
pub fn make_easy_target_list(
    S: &State,
    H: usize,
    W: usize,
    T: &Vec<Vec<usize>>,
    option: &ChokudaiOptions,
) -> (usize, Vec<(usize, usize)>) {

    let mut start_point = (!0, !0);
    let mut lastAction = !0;

    use rand::Rng;
    let mut rng = rand::thread_rng();

    for x in 0..H {
        for y in 0..W {
            if S.field[x][y] == Square::Empty {

                for k in 0..4 {
                    let (nx, ny) = apply_move((x, y), k);
                    if T[nx][ny] != !0 && (lastAction == !0 || lastAction < T[nx][ny]) {
                        start_point = (x, y);
                        lastAction = T[nx][ny];
                    }
                    if option.RandFlag && T[nx][ny] != !0 && rng.gen::<usize>() % 2 == 1 {
                        start_point = (x, y);
                        lastAction = T[nx][ny];
                    }
                }

            }
        }
    }
    if lastAction == !0 {
        start_point = (!0, !0);
    }

    let ans = make_list_with_startpoint(S, H, W, start_point);
    (lastAction, ans)
}

pub fn get_diff(a: usize, b: usize) -> usize {
    if a > b {
        return a - b;
    }
    b - a
}

pub fn make_move(a2: &Vec<Action>, R: usize, L: usize, d: usize) -> Vec<Action> {
    let mut stockR = R;
    let mut stockL = L;
    let mut now_dir = d;

    let mut actions: Vec<Action> = Vec::with_capacity(0);

    for act in a2 {
        if *act == Action::TurnR {
            stockR += 1;
        }
        if *act == Action::TurnL {
            stockL += 1;
        }
    }

    for act in a2 {
        match &act {
            Action::Move(d) => {
                if *d == (now_dir + 1) % 4 {
                    if stockR >= 1 {
                        stockR -= 1;
                        actions.push(Action::TurnR);
                        now_dir = (now_dir + 1) % 4;
                    }
                } else if *d == (now_dir + 3) % 4 {
                    if stockL >= 1 {
                        stockL -= 1;
                        actions.push(Action::TurnL);
                        now_dir = (now_dir + 3) % 4;
                    } else if stockR == 2 || stockL == 2 {
                        stockR = 0;
                        stockL = 1;
                        actions.push(Action::TurnL);
                        now_dir = (now_dir + 3) % 4;
                    }
                } else if *d == (now_dir + 2) % 4 {
                    if stockR >= 2 {
                        stockR -= 2;
                        actions.push(Action::TurnR);
                        actions.push(Action::TurnR);
                        now_dir = (now_dir + 2) % 4;
                    }
                }


                actions.push(*act);

            }
            _ => {}
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

fn check_straight(S: &State, tx: usize, ty: usize) -> bool {
    let d = S.p.dir;
    let sx = S.p.x;
    let sy = S.p.y;

    if d == 0 {
        if tx <= sx {
            return false;
        }

        //eprintln!("ok!");

        let px = tx - 1;
        let py = sy;
        let dx = (tx - px) as i32;
        let dy = (ty - py) as i32;

        /*
        let mut hasManu = false;
        for dxy in &S.p.manipulators {
            if *dxy == (dx, dy){
                hasManu = true;
                break;
            }
        }
        if !hasManu {return false;}
        if !is_visible(&S.field, (px, py), (dx, dy)) {return false;}
        */

        if get_diff(sy, ty) > 1 {
            return false;
        }

        for x in sx + 1..tx {
            if S.field[x][sy] == Square::Block {
                return false;
            }
        }

        return true;
    } else if d == 1 {

        if ty >= sy {
            return false;
        }
        if get_diff(sx, tx) > 1 {
            return false;
        }
        for y in ty..sy {
            if S.field[sx][y] == Square::Block {
                return false;
            }
        }
        return true;
    } else if d == 2 {
        if tx >= sx {
            return false;
        }
        if get_diff(sy, ty) > 1 {
            return false;
        }
        for x in tx + 1..sx {
            if S.field[x][sy] == Square::Block {
                return false;
            }
        }
        return true;
    } else if d == 3 {
        if ty <= sy {
            return false;
        }
        if get_diff(sx, tx) > 1 {
            return false;
        }
        for y in sy + 1..ty {
            if S.field[sx][y] == Square::Block {
                return false;
            }
        }
        //eprintln!("ok!");
        return true;
    }

    false
}

fn get_straight(S: &State, tx: usize, ty: usize) -> Vec<Action> {
    let d = S.p.dir;
    let sx = S.p.x;
    let sy = S.p.y;

    let mut need = 0;
    if d % 2 == 0 {
        need = get_diff(sx, tx) - 1;
    } else {
        need = get_diff(sy, ty) - 1;
    }
    let mut ret: Vec<Action> = Vec::with_capacity(0);
    for i in 0..need {
        ret.push(Action::Move(d));
    }
    ret
}

fn check_straight_left(S: &State, tx: usize, ty: usize) -> bool {
    let d = S.p.dir;
    let sx = S.p.x;
    let sy = S.p.y;

    if d == 0 {
        if tx < sx {
            return false;
        }
        if ty <= sy {
            return false;
        }
        for x in sx + 1..tx {
            if S.field[x][sy] == Square::Block {
                return false;
            }
        }
        for y in sy..ty {
            if S.field[tx][y] == Square::Block {
                return false;
            }
        }
        return true;

    } else if d == 1 {

        return true;
    } else if d == 2 {
        if tx > sx {
            return false;
        }
        if ty >= sy {
            return false;
        }
        for x in tx..sx {
            if S.field[x][sy] == Square::Block {
                return false;
            }
        }
        for y in ty..sy {
            if S.field[tx][y] == Square::Block {
                return false;
            }
        }
        return true;

    } else if d == 3 {

    }

    false
}

fn check_straight_right(S: &State, tx: usize, ty: usize) -> bool {
    let d = S.p.dir;
    let sx = S.p.x;
    let sy = S.p.y;

    false
}


fn get_next_action(
    first_state: &State,
    option: &ChokudaiOptions,
    final_action: &Vec<Action>,
    bfs: &mut BFS,
) -> Vec<Action> {
    let H = first_state.field.len();
    let W = first_state.field[0].len();
    let FX = first_state.p.x;
    let FY = first_state.p.y;
    let mut current_state = first_state.clone();

    let mut LastActionTable = vec![vec![!0; W]; H];

    for dxy in &first_state.p.manipulators {
        let nx = FX + dxy.0 as usize;
        let ny = FY + dxy.1 as usize;

        if is_visible(&first_state.field, (first_state.p.x, first_state.p.y), *dxy) {
            LastActionTable[nx][ny] = 0;
            current_state.field[nx][ny] = Square::Filled;
        }
    }

    let mut p_state: Vec<WorkerState> = Vec::with_capacity(0);
    p_state.push(current_state.p.clone());

    let mut action_cnt = 0;
    for act in final_action {
        action_cnt += 1;
        let upd = apply_action(
            *act,
            &mut current_state.p,
            &mut current_state.field,
            &mut current_state.item_field,
        );
        for p in upd.filled {
            LastActionTable[p.0][p.1] = action_cnt;
            //eprintln!("{}", action_cnt);
        }
        p_state.push(current_state.p.clone());
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

    let mut point_sum = 0;

    let mut AddActions: Vec<Vec<Action>> = vec![Vec::with_capacity(0); action_cnt + 1];

    loop {
        let (last_act, point_list) =
            make_easy_target_list(&current_state, H, W, &LastActionTable, option);

        //eprintln!("List : {}", point_list.len());
        if point_list.len() == 0 {
            break;
        }

        if last_act == action_cnt && AddActions[action_cnt].len() != 0 {
            break;
        }

        point_sum += point_list.len();


        current_state.p = p_state[last_act].clone();

        let mut temp_action: Vec<Action> = Vec::with_capacity(0);

        //復帰するべき状態
        let back_state = p_state[last_act].clone();

        let mut firstloop = false;
        if option.miningFlag {
            firstloop = true;
        }


        /*
        if (point_list.len() > 10) {
            let mut maxx = 0;
            let mut minx = 99999;
            let mut maxy = 0;
            let mut miny = 99999;

            for i in 0..point_list.len() {
                let (x, y) = point_list[i];
                if maxx < x {
                    maxx = x;
                }
                if minx > x {
                    minx = x;
                }
                if maxy < y {
                    maxy = y;
                }
                if miny > y {
                    miny = y;
                }
            }

            let mut V = vec![vec![','; maxx - minx + 1]; maxy - miny + 1];

            for i in 0..point_list.len() {
                let (x, y) = point_list[i];
                V[y - miny][x - minx] = 'o';
            }

            for y in 0..V.len() {
                for x in 0..V[0].len() {
                    eprint!("{}", V[y][x]);
                }
                eprintln!("");
            }
        }
        */


        for i in 0..point_list.len() {
            let target_pos = point_list[i];
            //println!("{}", firstloop);
            //eprintln!("check: {} {}", target_pos.0, target_pos.1);
            //塗り済みであるかの検出
            if current_state.field[target_pos.0][target_pos.1] != Square::Empty {
                //eprintln!("skip");
                continue;
            }

            let mut actions: Vec<Action> = Vec::with_capacity(0);

            if check_straight(&current_state, target_pos.0, target_pos.1) {
                //println!("find");
                actions = get_straight(&current_state, target_pos.0, target_pos.1);
            } else if false
                && !firstloop
                && check_straight_left(&current_state, target_pos.0, target_pos.1)
            {

            } else if !option.OptType == 0 {
                let (a2, gx, gy) = bfs.search_fewest_actions_to_wrap(
                    &current_state.field,
                    &current_state.p,
                    target_pos.0,
                    target_pos.1,
                );

                actions = make_move(&a2, 0, 0, current_state.p.dir);
            } else if option.OptType == 1 {
                //２連塗チェック
                let mut use_double_position = ((!0, !0), !0);

                if i != point_list.len() - 1 {
                    let next_target_pos = point_list[i + 1];
                    if current_state.field[next_target_pos.0][next_target_pos.1] == Square::Empty {
                        let diff = get_diff(target_pos.0, next_target_pos.0)
                            + get_diff(target_pos.1, next_target_pos.1);
                        if diff == 1 {
                            let mut pos = (!0, !0);
                            let mut d = !0;
                            if target_pos.0 != next_target_pos.0 {
                                if target_pos.0 < next_target_pos.0 {
                                    if current_state.field[target_pos.0][target_pos.1 - 1]
                                        != Square::Block
                                    {
                                        pos = (target_pos.0 - 1, target_pos.1 - 1);
                                        d = 0;
                                    }
                                } else {
                                    if current_state.field[target_pos.0][target_pos.1 + 1]
                                        != Square::Block
                                    {
                                        pos = (target_pos.0 + 1, target_pos.1 + 1);
                                        d = 2;
                                    }
                                }
                            } else {
                                if target_pos.1 < next_target_pos.1 {
                                    if current_state.field[target_pos.0][target_pos.1 + 1]
                                        != Square::Block
                                    {
                                        pos = (target_pos.0 + 1, target_pos.1 - 1);
                                        d = 3;
                                    }
                                } else {
                                    if current_state.field[target_pos.0][target_pos.1 - 1]
                                        != Square::Block
                                    {
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
                    let mut a2 = bfs.search_fewest_actions_to_move(
                        &current_state.field,
                        &current_state.p,
                        (use_double_position.0).0,
                        (use_double_position.0).1,
                    );
                    let mut now_dir = current_state.p.dir;
                    let mut stockR = 0;
                    let mut stockL = 0;

                    if (now_dir + 1) % 4 == use_double_position.1 {
                        stockR = 1;
                    } else if (now_dir + 2) % 4 == use_double_position.1 {
                        stockR = 2;
                    } else if (now_dir + 3) % 4 == use_double_position.1 {
                        stockL = 1;
                    }

                    actions = make_move(&a2, stockR, stockL, now_dir);
                } else {

                    // 「 型の角を見つけたら突っ込む
                    let mut leftfront = false;
                    let d = get_diff(current_state.p.x, target_pos.0)
                        + get_diff(current_state.p.y, target_pos.1);
                    let (nx, ny) =
                        apply_move((current_state.p.x, current_state.p.y), current_state.p.dir);
                    let (nx2, ny2) = apply_move((nx, ny), current_state.p.dir);
                    let (nx3, ny3) = apply_move((nx2, ny2), (current_state.p.dir + 3) % 4);

                    if (nx3 == target_pos.0 && ny3 == target_pos.1) {
                        if (current_state.field[nx][ny] != Square::Block
                            && current_state.field[nx2][ny2] != Square::Block)
                        {
                            leftfront = true;
                        }
                    }

                    if !leftfront {
                        let (a2, gx, gy) = bfs.search_fewest_actions_to_wrap(
                            &current_state.field,
                            &current_state.p,
                            target_pos.0,
                            target_pos.1,
                        );
                        actions = make_move(&a2, 0, 0, current_state.p.dir);
                    } else {
                        actions = Vec::with_capacity(0);
                        actions.push(Action::Move(current_state.p.dir));
                    }
                }
            }


            'actloop: for act in actions {
                // apply_action で field と item_field も更新する
                let ret = apply_action(
                    act,
                    &mut current_state.p,
                    &mut current_state.field,
                    &mut current_state.item_field,
                );
                temp_action.push(act);

                for (tx, ty) in ret.filled {
                    if tx == target_pos.0 && ty == target_pos.1 && !firstloop {
                        break 'actloop;
                    }
                }
            }
            firstloop = true;
        }

        //Actionを差し込む前の状態にちゃんと戻す
        if last_act != final_action.len() {
            let a2 = bfs.search_fewest_actions_to_move(
                &current_state.field,
                &current_state.p,
                back_state.x,
                back_state.y,
            );

            for act in a2 {
                // apply_action で field と item_field も更新する
                apply_action(
                    act,
                    &mut current_state.p,
                    &mut current_state.field,
                    &mut current_state.item_field,
                );
                temp_action.push(act);
            }

            let now_dir = current_state.p.dir;

            let mut a3: Vec<Action> = Vec::with_capacity(0);
            if (now_dir + 1) % 4 == back_state.dir {
                a3.push(Action::TurnR);
            } else if (now_dir + 2) % 4 == back_state.dir {
                a3.push(Action::TurnR);
                a3.push(Action::TurnR);
            } else if (now_dir + 3) % 4 == back_state.dir {
                a3.push(Action::TurnL);
            }


            for act in a3 {
                // apply_action で field と item_field も更新する
                apply_action(
                    act,
                    &mut current_state.p,
                    &mut current_state.field,
                    &mut current_state.item_field,
                );
                temp_action.push(act);
            }
        }

        for act in temp_action {
            AddActions[last_act].push(act);
        }
    }
    let mut next_action: Vec<Action> = Vec::with_capacity(0);
    for i in 0..final_action.len() {
        for act in &AddActions[i] {
            next_action.push(*act);
        }
        next_action.push(final_action[i]);
    }
    for act in &AddActions[final_action.len()] {
        next_action.push(*act);
    }
    next_action
}

pub fn make_action_by_state(first_state: &State, option: &ChokudaiOptions) -> Vec<Action> {
    let H = first_state.field.len();
    let W = first_state.field[0].len();

    let FX = first_state.p.x;
    let FY = first_state.p.y;

    let mut bfs = BFS::new(H, W);

    let mut final_action: Vec<Action> = Vec::with_capacity(0);

    loop {
        let next_action = get_next_action(first_state, option, &final_action, &mut bfs);
        if next_action.len() == final_action.len() {
            break;
        }
        final_action = next_action;
    }
    final_action
}

///○秒まで頑張って回す
pub fn optimization_actions(
    first_state: &State,
    actions: &Vec<Action>,
    Seconds: usize,
    option: &ChokudaiOptions,

) -> (bool, Vec<Action>) {

    let mut ans: Vec<Action> = actions.clone();

    let start = Instant::now();

    loop {
        let end = start.elapsed();
        let time = end.as_secs();
        if time >= Seconds as u64 {
            break;
        }
        let (flag, act) =
            shortening_actions(first_state, &ans, (Seconds as u64 - time) as usize, option);
        if flag {
            ans = act;
        }
    }

    if ans.len() == actions.len() {
        return (false, ans);
    }

    (true, ans)
}

///○秒の更新がある間は回す
///返り値：成功フラグ、新しいAction列
pub fn shortening_actions(
    first_state: &State,
    actions: &Vec<Action>,
    Seconds: usize,
    option: &ChokudaiOptions,
) -> (bool, Vec<Action>) {
    if actions.len() < 3 {
        return (false, Vec::with_capacity(0));
    }

    let start = Instant::now();

    let mut minimum_range = std::cmp::min(5, actions.len() / 2);
    let mut maximum_range = std::cmp::min(actions.len() - 2, std::cmp::max(30, minimum_range));
    while minimum_range >= maximum_range {
        minimum_range = std::cmp::min(5, actions.len() / 2);
        maximum_range = std::cmp::min(actions.len() - 2, std::cmp::max(30, minimum_range));
    }

    let H = first_state.field.len();
    let W = first_state.field[0].len();


    let mut bfs = BFS::new(H, W);

    loop {
        let end = start.elapsed();
        let time = end.as_secs();
        if time >= Seconds as u64 {
            break;
        }

        use rand::Rng;
        let mut rng = rand::thread_rng();

        let action_range = rng.gen::<usize>() % (maximum_range - minimum_range + 1) + minimum_range;
        let start_action = rng.gen::<usize>() % (actions.len() - action_range);
        let end_action = start_action + action_range;

        //println!("{} {} {}", start_action, end_action, actions.len());
        let (flag, act) = shortening(
            &first_state,
            actions,
            start_action,
            end_action,
            &mut bfs,
            option,
        );

        if flag {
            return (true, act);
        }
    }

    (false, Vec::with_capacity(0))
}

fn shortening(
    first_state: &State,
    acts: &Vec<Action>,
    start: usize,
    end: usize,
    bfs: &mut BFS,
    option: &ChokudaiOptions,
) -> (bool, Vec<Action>) {

    let H = first_state.field.len();
    let W = first_state.field[0].len();
    let actions = acts.clone();

    let mut start_state = first_state.clone();
    for i in 0..start {
        let act = actions[i];
        apply_action(
            act,
            &mut start_state.p,
            &mut start_state.field,
            &mut start_state.item_field,
        );
    }
    let start_position = start_state.p.clone();
    let start_field = start_state.field.clone();
    let start_itemfield = start_state.item_field.clone();

    let mut current_state = start_state.clone();
    for i in start..end {
        let act = actions[i];
        apply_action(
            act,
            &mut current_state.p,
            &mut current_state.field,
            &mut current_state.item_field,
        );
    }
    current_state.field = start_field;
    current_state.item_field = start_itemfield;
    let end_position = current_state.p.clone();

    for i in end..actions.len() {
        let act = actions[i];
        apply_action(
            act,
            &mut current_state.p,
            &mut current_state.field,
            &mut current_state.item_field,
        );
    }

    let check_state = State {
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
            if check_state.field[x][y] == Square::Empty {
                empty_cells += 1;
                if max_x < x {
                    max_x = x;
                }
                if min_x > x {
                    min_x = x;
                }
                if max_y < y {
                    max_y = y;
                }
                if min_y > y {
                    min_y = y;
                }
            }
        }
    }

    //eprintln!("RemoveRange : {}", end - start);
    //eprintln!("EmptyCell : {}", empty_cells);
    //eprintln!("range : ({}, {}) to ({}, {})", min_x, min_y, max_x, max_y);

    let x_move = get_diff(start_position.x, end_position.x);
    let y_move = get_diff(start_position.y, end_position.y);
    let mut need_to_move = x_move + y_move;
    if start_position.dir != end_position.dir {
        if start_position.dir == (end_position.dir + 2) % 4 {
            need_to_move += 2;
        } else {
            need_to_move += 1;
        }
    }

    //eprintln!("needToMove : {}", need_to_move);
    //eprintln!("");

    //とりあえず適当にまっすぐ繋いでみた後、上手い事回収する
    {
        let mut now_state = check_state.clone();
        let mut acts = bfs.search_fewest_actions_to_move(
            &now_state.field,
            &now_state.p,
            end_position.x,
            end_position.y,
        );
        let mut stockR = 0;
        let mut stockL = 0;

        if end_position.dir == (start_position.dir + 1) % 4 {
            stockR = 1;
        } else if end_position.dir == (start_position.dir + 2) % 4 {
            stockR = 2;
        } else if end_position.dir == (start_position.dir + 3) % 4 {
            stockL = 1;
        }
        let a2 = make_move(&acts, stockR, stockL, start_position.dir);

        let mut now_actions: Vec<Action> = Vec::with_capacity(0);
        for i in 0..start {
            let act = actions[i];
            now_actions.push(act);
        }
        for a in a2 {
            now_actions.push(a);
        }
        for i in end..actions.len() {
            let act = actions[i];
            now_actions.push(act);
        }

        let mut final_action = now_actions;
        loop {
            let next_action = get_next_action(&first_state, &option, &final_action, bfs);
            if final_action.len() == next_action.len() {
                break;
            }
            final_action = next_action;
        }

        if final_action.len() < actions.len() {
            eprintln!("OK ! {} => {}", actions.len(), final_action.len());
            return (true, final_action);
        }
    }


    (false, Vec::with_capacity(0))
}