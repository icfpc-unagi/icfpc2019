#![allow(unused)]
extern crate wata;
use wata::*;

fn main() {
    let mut bfs = wata::bfs::BFS::new(3);

    let r = 4;
    let filled = vec![
        mat![false; r; r],
        vec![
            vec![false, false, false, false],
            vec![false,  true,  true, false],
            vec![false,  true,  true, false],
            vec![false, false, false, false],
        ],
        mat![false; r; r],
        mat![false; r; r],
    ];

    let filled_func = |p: P| {
        return filled[p];
    };

    let goal_func = |p: P| {
        return (p.x, p.y, p.z) == (2, 2, 2);
    };

    let ret = bfs.bfs(filled_func, &vec![P::new(0, 0, 0)], goal_func);
    println!("{:?}", ret);
    bfs.show();
    println!("{:?}", bfs.restore(P::new(2, 2, 2)));

    bfs.clear();
    bfs.show();

    let ret = bfs.bfs(filled_func, &vec![P::new(0, 0, 0)], goal_func);
    println!("{:?}", ret);
    bfs.show();
    println!("{:?}", bfs.restore(P::new(2, 2, 2)));

    println!("2,2,2 -> 2,2,2");
    bfs.clear();
    let ret = bfs.bfs(filled_func, &vec![P::new(2, 2, 2)], goal_func);
    println!("{:?}", ret);
    bfs.show();
    println!("{:?}", bfs.restore(P::new(2, 2, 2)));

    // Expected panic:
    // let ret = bfs.bfs(filled_func, &vec![P::new(0, 0, 0)], goal_func);
    // bfs.restore(P::new(1, 1, 1));
}
