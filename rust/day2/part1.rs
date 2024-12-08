use std::fs;

fn get_levels(file_path: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(file_path).expect("Error: Could not read from file");
    contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect()
        })
        .collect()
}

fn is_safe(level: &[i32]) -> bool {
    if level.is_empty() {
        return false;
    }

    let is_increasing = level[0] <= level[1];
    for window in level.windows(2) {
        if let [a, b] = window {
            let difference = a - b;

            let is_incorrect = difference == 0
                || difference.abs() > 3
                || (is_increasing && a > b)
                || (!is_increasing && a < b);
            if is_incorrect {
                return false;
            }
        }
    }
    return true;
}

fn main() {
    let levels = get_levels("input.txt");
    let result = levels.into_iter().filter(is_safe).count();
    println!("{:?}", result);
}
