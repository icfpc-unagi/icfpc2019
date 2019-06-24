use knapsack::*;

fn main() {
    let in_file_path = std::env::args()
        .nth(1)
        .expect("usage: args[1] = in_file_path");
    let budget = std::env::args().nth(2).expect("usage: args[2] = budget");
    let budget = budget.parse().unwrap();
    let solutions = read_solutions(&in_file_path);
    eprintln!("Loading done");
    let selected_solutions = solve(&solutions, budget);

    for solution in &selected_solutions {
        println!(
            "{},{},{}",
            solution.problem_name, solution.buy, solution.solution_name
        )
    }
}
