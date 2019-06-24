

fn main() {
    let problem_sizes = knapsack::get_problem_sizes_from_task_files();
    let mut problem_sizes: Vec<_> = problem_sizes.into_iter().collect();
    problem_sizes.sort();

    println!("[");
    for (name, (xsize, ysize)) in problem_sizes {
        println!("  (\"{}\", ({}, {})),", name, xsize, ysize);
    }
    println!("]");
}
