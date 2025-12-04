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

    let Ok(contents) = fs::read_to_string(input_path) else {
        println!("Failed to read input file");
        std::process::exit(1);
    };

    let diagram = day4::parse_diagram(&contents);
    match part.as_str() {
        "part1" => {
            println!("Result: {}", day4::part1(&diagram));
        },
        "part2" => {
            println!("Result: {}", day4::part2(&diagram));
        },
        "part1_set" => {
            println!("Result: {}", day4::part1_set(contents.as_str()));
        },
        "part2_set" => {
            println!("Result: {}", day4::part2_set(contents.as_str()));
        },
        x => {
            println!("Invalid part: {}", x);
            std::process::exit(1);
        },
    }
}
