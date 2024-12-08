use regex::Regex;
use std::fs;

fn read_from_file(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path).expect("Error: Could not read from file");
    contents.split("don't()").map(|s| s.to_string()).collect()
}

fn get_multiplied_numbers(text: &Vec<String>) -> Vec<i32> {
    let mut results: Vec<i32> = vec![];
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for part in text {
        if let Some((_, valid_part)) = part.split_once("do()") {
            results.extend(perform_multiplications_on_text(&re, valid_part));
        }
    }
    if let Some(first_part) = text.get(0) {
        results.extend(perform_multiplications_on_text(&re, first_part));
    }
    results
}

fn perform_multiplications_on_text(re: &Regex, text: &str) -> Vec<i32> {
    re.captures_iter(text)
        .filter_map(|cap| {
            let m1 = cap.get(1)?.as_str().parse::<i32>().ok()?;
            let m2 = cap.get(2)?.as_str().parse::<i32>().ok()?;
            Some(m1 * m2)
        })
        .collect()
}

fn main() {
    let path = "input.txt";
    let parts = read_from_file(path);
    let numbers = get_multiplied_numbers(&parts);

    let sum: i32 = numbers.iter().sum();
    println!("{:?}", sum);
}
