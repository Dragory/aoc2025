use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let Some(part) = args.get(1) else {
        println!("Missing part selection");
        std::process::exit(1);
    };

    let Some(input_path) = args.get(2) else {
        println!("Missing input path");
        std::process::exit(1);
    };

    let Ok(input) = fs::read_to_string(input_path) else {
        println!("Failed to read input file");
        std::process::exit(1);
    };

    let lines: Vec<&str> = input.lines().collect();

    match part.as_str() {
        "part1" => {
            println!("Result: {}", day3::part1(&lines));
        },
        "part2" => {
            println!("Result: {}", day3::part2(&lines));
        },
        x => {
            println!("Invalid part {}", x);
            std::process::exit(1);
        },
    };
}
