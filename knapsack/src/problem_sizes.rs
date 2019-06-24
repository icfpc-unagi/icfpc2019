use std::collections::HashMap;

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
