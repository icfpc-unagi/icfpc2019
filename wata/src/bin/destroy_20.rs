extern crate wata;
use wata::*;

use wata::Command;

fn emit(commands: Vec<Command>) {
    for command in commands.iter() {
        println!("{}", command.to_string());
    }
}

fn main() {
    //solve FA011
    let file = std::env::args().nth(1).unwrap();
    let model = wata::read(&file);
    let commands = destroy_small_face(model);
    emit(commands);
    eprintln!("{}", file);
}


pub fn destroy_small_face(model: Model) -> Vec<Command> {
    let r = model.r;
    let mut x_max = 2;
    let mut y_max = 3; // 1;
    let mut z_max = 2;
    for x in 0..r { for y in 0..r { for z in 0..r { if model.filled[x][y][z] {
        x_max = x_max.max(x);
        y_max = y_max.max(y);
        z_max = z_max.max(z);
    }}}}
    let (x_max, y_max, z_max) = (x_max as i32, y_max as i32, z_max as i32);

    let mut x_min = 99;
    let mut y_min = 99;
    let mut z_min = 99;
    for x in 0..r { for y in 0..r { for z in 0..r { if model.filled[x][y][z] {
        x_min = x_min.min(x);
        y_min = y_min.min(y);
        z_min = z_min.min(z);
    }}}}
    let (x_min, _y_min, z_min) = (x_min as i32, y_min as i32, z_min as i32);

    eprintln!("{}, {}", x_min, x_max);

    let r = model.r as i32;
    let mut all = vec![];

    //
    // Fission
    //
    let bot_ps = (0..8).map(|i| {
        P::new(
            x_min - 1 + ((i >> 0) & 1) * (x_max - x_min + 2),
            ((i >> 1) & 1) * y_max,
            z_min - 1 + ((i >> 2) & 1) * (z_max - z_min + 2))
    }).collect();
    let (order, commands) = fission_to(&mat![false; r as usize; r as usize; r as usize], &bot_ps);
    all.extend(commands);

    //
    // GVoid
    //
    {
        let gvoid_ps: Vec<_> = (0..8).map(|i| {
            P::new(
                x_min + ((i >> 0) & 1) * (x_max - x_min),
                ((i >> 1) & 1) * y_max,
                z_min + ((i >> 2) & 1) * (z_max - z_min))
        }).collect();

        for mask in [3, 5, 6].iter() {
            let mut commands = vec![Command::Wait; 8];

            for i in 0..8 {
                let my_bid = order[i] - 1;  // ord is 1-indexed
                let my_bot_p = bot_ps[i];
                let my_gvoid_p = gvoid_ps[i];

                let opposite_gvoid_p = gvoid_ps[i ^ mask];
                commands[my_bid] = Command::GFill(
                    my_gvoid_p - my_bot_p,
                    opposite_gvoid_p - my_gvoid_p,
                )
            }

            all.extend(commands);
        }
    }

    //
    // Fusion
    //
    let mut bot_ps2 = vec![P::new(0, 0, 0); 8];
    for i in 0..8 {
        bot_ps2[order[i] - 1] = bot_ps[i];
    }
    let commands = postproc::fusion_all(&model.filled, bot_ps2);
    all.extend(commands);

    return all;
}
