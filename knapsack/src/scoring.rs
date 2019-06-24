use crate::*;

pub fn get_original_score(xt: usize, yt: usize, t_best: i32, t_team: i32) -> f64 {
    let (xt, yt, t_best) = (xt as f64, yt as f64, t_best as f64);
    let t_team = t_team as f64;
    f64::ceil(1000.0 * f64::log2(xt * yt) * t_best / t_team)
}

pub fn get_scores1(
    solution_set: &Vec<Solution>,
    problem_sizes: &HashMap<String, (usize, usize)>,
) -> Vec<f64> {
    // t_bestを自分の最良のやつだと思って、自分のスコアを最大化するやつ。

    let problem_name = &solution_set[0].problem_name;
    let &(xt, yt) = problem_sizes.get(problem_name).unwrap();

    let times = solution_set.iter().map(|s| s.time);
    let t_best = times.clone().min().unwrap();

    times
        .map(|t_team| get_original_score(xt, yt, t_best, t_team))
        .collect()
}

pub fn get_scores2(
    solution_set: &Vec<Solution>,
    problem_sizes: &HashMap<String, (usize, usize)>,
) -> Vec<f64> {
    // t_teamをbuyしないやつだと思って、t_bestを自分のやつだと思って、敵のスコアとの差分を最大化するやつ。

    let &(xt, yt) = problem_sizes.get(&solution_set[0].problem_name).unwrap();

    let others_solution = solution_set
        .iter()
        .find(|solution| solution.buy == "")
        .unwrap();
    let t_team = others_solution.time;

    let times = solution_set.iter().map(|s| s.time);

    times
        .map(|t_best| {
            get_original_score(xt, yt, t_team, t_team) - get_original_score(xt, yt, t_best, t_team)
        })
        .collect()
}

pub fn get_scores(solution_set: &Vec<Solution>, problem_sizes: &ProblemSizes) -> Vec<f64> {
    return get_scores2(solution_set, problem_sizes);
}
