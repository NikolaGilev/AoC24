use regex::Regex;
use std::fs;

fn read_from_file(path: &str) -> Vec<i32> {
    let contents = fs::read_to_string(path).expect("Error: Could not read from file");
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut results: Vec<i32> = vec![];

    for (_, [mult1, mult2]) in re.captures_iter(&contents).map(|c| c.extract()) {
        results.push(
            mult1.parse::<i32>().expect("Error: ups") * mult2.parse::<i32>().expect("error: ups"),
        );
    }
    results
}

fn main() {
    let path = "input.txt";
    let numbers = read_from_file(path);
    let sum: i32 = numbers.iter().sum();
    println!("{:?}", sum);
}
