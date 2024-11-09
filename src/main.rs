use std::{env, fs};

const NUM_OF_LINES: usize = 2;

fn main() {
    let dev_mode = env::args().any(|a| a == "--DEV");

    let input = match dev_mode {
        true => fs::read_to_string("./input").unwrap(),
        false => {
            let mut input = String::new();
            for _ in 0..NUM_OF_LINES {
                let mut line = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                input.push_str(&line);
            }
            input
        }
    };

    solve(&input);
}

fn solve(input: &str) {
    let lines = input.lines().collect::<Vec<&str>>();

    let numbers = lines[1]
        .split_whitespace()
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    println!(
        "{:?}",
        numbers.iter().max().unwrap() + numbers.iter().min().unwrap()
    );
}
