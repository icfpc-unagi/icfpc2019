use common::SetMinMax;

pub struct KnapsackProblem {
    pub item_sets: Vec<Vec<(usize, f64)>>,
    pub capacity: usize,
}

fn get_max<T: PartialOrd + Copy>(a: &[T], mut m: T) -> T {
    for &x in a {
        m.setmax(x);
    }
    m
}

pub fn solve_knapsack_problem(problem: &KnapsackProblem) -> Vec<usize> {
    let &KnapsackProblem {
        ref item_sets,
        capacity,
    } = problem;

    let n_item_sets = item_sets.len();

    let mut dp = vec![vec![]; n_item_sets + 1];
    // dp[i][x] := i番目までのアイテムセットが終わってて、ここまででxだけ使った時の、最良コスト。
    dp[0] = (0..=capacity).map(|x| ((x * 100) as f64, !0, !0)).collect();

    for i in 0..n_item_sets {
        // この行は要らないことにする。必ずweightが0のアイテムが含まれるはずで、そいつを必ず使ってくれ。
        // dp[i + 1] = (0..=capacity).map(|x| (dp[i][x].0, !0, x)).collect();
        dp[i + 1] = vec![(std::f64::MIN, !0, !0); capacity + 1];

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
        get_max(&dp[n_item_sets], (std::f64::MIN, !0, !0)).0,
        dp[n_item_sets][capacity].0
    );

    let mut x = capacity;
    let mut selection = vec![!0; n_item_sets];
    for i in (0..n_item_sets).rev() {
        let t = dp[i + 1][x];
        selection[i] = t.1;
        x = t.2;
    }
    eprintln!("Capacity: {}, Remaining: {}, Best Score: {}", capacity, x, dp[n_item_sets][capacity].0);

    selection
}
