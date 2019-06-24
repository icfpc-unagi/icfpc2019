use std::io::BufRead;
use std::collections::HashMap;
use common::SetMinMax;

//
// 基本の構造体 ＋ 読み込みなど
//

#[derive(Debug, Clone)]
pub struct Solution {
    // (問題名),(買った物),(なんかsolutionのID),(ステップ数)
    pub problem_name: String,
    pub buy: String,
    pub solution_name: String,
    pub time: i32,
}

impl Solution {
    fn cost(&self) -> usize {
        let buy_cost_map: HashMap<char, usize> = [
            ('B', 1000), // Manipulator
            ('F', 300),  // Fast Wheels
            ('L', 700),  // Drill
            ('R', 1200), // Teleport
            ('C', 2000), // Cloning
        ]
            .iter()
            .cloned()
            .collect();

        self.buy
            .chars()
            .map(|c| buy_cost_map.get(&c).unwrap())
            .sum()
    }

    fn cost100d(&self) -> usize {
        self.cost() / 100
    }
}

pub fn read_solutions(input_path: &str) -> Vec<Solution> {
    let file = std::fs::File::open(input_path).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut solutions = vec![];

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        let line = line.trim();
        if line == "" {
            continue;
        }

        let tokens: Vec<_> = line.split(',').collect();
        // dbg!(&tokens);

        solutions.push(Solution {
            problem_name: tokens[0].to_owned(),
            buy: tokens[1].to_owned(),
            solution_name: tokens[2].to_owned(),
            time: tokens[3].parse().unwrap(),
        })
    }

    eprintln!("Solutions: {}", solutions.len());

    solutions
}

pub fn get_solution_sets(solutions: &Vec<Solution>) -> Vec<Vec<Solution>> {
    let mut problem_to_solutions: HashMap<String, Vec<Solution>> = std::collections::HashMap::new();
    for solution in solutions {
        problem_to_solutions
            .entry(solution.problem_name.clone())
            .or_insert(vec![])
            .push(solution.clone());
    }
    eprintln!("Problems: {}", problem_to_solutions.len());

    let mut solution_sets: Vec<_> = problem_to_solutions.values().cloned().collect();
    solution_sets.sort_by_key(|solution_set| solution_set[0].problem_name.clone());
    solution_sets
}

pub fn get_scores(
    solution_set: &Vec<Solution>,
    problem_sizes: &HashMap<String, (usize, usize)>,
) -> Vec<i32> {
    let problem_name = &solution_set[0].problem_name;
    let &(xt, yt) = problem_sizes.get(problem_name).unwrap();

    let times = solution_set.iter().map(|s| s.time);
    let t_best = times.clone().min().unwrap();
    let (xt, yt, t_best) = (xt as f64, yt as f64, t_best as f64);

    times
        .map(|t_team| {
            let t_team = t_team as f64;
            f64::ceil(1000.0 * f64::log2(xt * yt) * t_best / t_team) as i32
        })
        .collect()
}

//
// 問題サイズ
//
pub fn get_problem_sizes() -> HashMap<String, (usize, usize)> {
    eprintln!("Retrieving problem sizes...");
    let path_patterns = [
        "../data/part-1-initial/*.desc",
        "../data/part-2-teleports/*.desc",
        "../data/part-3-clones/*.desc",
    ];

    let mut problem_sizes = HashMap::new();

    let mut x = 0;

    for path_pattern in &path_patterns {
        for path in glob::glob(path_pattern).unwrap() {
            let path_buf = path.unwrap();
            let problem_name = path_buf.file_stem().unwrap().to_str().unwrap().to_owned();
            let path_str = path_buf.to_str().unwrap().to_owned();
            let task = common::read_task(&path_str);

            if task
                .1
                .iter()
                .map(|row| row.iter().map(|&c| c == Some(common::Booster::X)).any(|b| b))
                .any(|b| b)
            {
                x += 1;
            }

            let (xsize, ysize) = common::get_xysize(&task.0);
            let (xsize, ysize) = (xsize - 2, ysize - 2); // TODO
            problem_sizes.insert(problem_name.clone(), (xsize, ysize));
        }
    }

    eprintln!("X: {}", x);

    eprintln!(
        "Sizes loaded for problems: {}\n{:?}",
        problem_sizes.len(),
        &problem_sizes
    );
    problem_sizes
}

//
// ナップサック問題関連
//

pub fn solve_knapsack_problem(item_sets: &Vec<Vec<(usize, i32)>>, capacity: usize) -> Vec<usize> {
    let n_item_sets = item_sets.len();

    let mut dp = vec![vec![]; n_item_sets + 1];
    // dp[i][x] := i番目までのアイテムセットが終わってて、ここまででxだけ使った時の、最良コスト。
    dp[0] = (0..=capacity).map(|x| ((x * 100) as i32, !0, !0)).collect();

    for i in 0..n_item_sets {
        dp[i + 1] = (0..=capacity).map(|x| (dp[i][x].0, !0, x)).collect();

        for (j, (weight, score)) in item_sets[i].iter().enumerate() {
            if *weight > capacity {
                continue;
            }
            for x in 0..=capacity - weight {
                let ts = dp[i][x].0 + score;
                dp[i + 1][x + weight].setmax((ts, j, x));
            }
        }
    }
    assert_eq!(
        dp[n_item_sets].iter().max().unwrap().0,
        dp[n_item_sets][capacity].0
    );

    let mut x = capacity;
    let mut selection = vec![!0; n_item_sets];
    for i in (0..n_item_sets).rev() {
        let t = dp[i + 1][x];
        selection[i] = t.1;
        x = t.2;
    }

    selection
}

pub fn get_knapsack_problem(
    solution_sets: &Vec<Vec<Solution>>,
    budget: usize,
    problem_sizes: &HashMap<String, (usize, usize)>,
) -> (Vec<Vec<(usize, i32)>>, usize) {
    (
        solution_sets
            .iter()
            .map(|solutions| {
                let scores = get_scores(solutions, problem_sizes);
                solutions
                    .iter()
                    .map(|solution| solution.cost100d())
                    .zip(scores)
                    .collect()
            })
            .collect(),
        budget / 100,
    )
}

pub fn solve(
    solutions: &Vec<Solution>,
    budget: usize,
    problem_sizes: &HashMap<String, (usize, usize)>,
) -> Vec<Solution> {
    let solution_sets = get_solution_sets(solutions);

    let (item_sets, capacity) = get_knapsack_problem(&solution_sets, budget, &problem_sizes);
    let selection = solve_knapsack_problem(&item_sets, capacity);

    let mut selected_solutions = vec![];
    for i in 0..selection.len() {
        let s = selection[i];
        if s == !0 {
            continue;
        } else {
            selected_solutions.push(solution_sets[i][s].clone());
        }
    }
    eprintln!("Selected: {}", selected_solutions.len());
    selected_solutions
}