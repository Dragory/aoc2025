use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let part = &args[1];
    if part == "part1" {
        part1(&args[2..]);
    } else if part == "part2" {
        part2(&args[2..]);
    } else {
        panic!("Invalid part {}", part);
    }
}

fn part1(lines: &[String]) {
    let result: i64 = lines.iter()
        .map(|line| parse_bank(line).map(|bank| find_largest(bank, 2)).unwrap_or(0))
        .sum();
    println!("Result: {}", result);
}

fn part2(lines: &[String]) {
    let result: i64 = lines.iter()
        .map(|line| parse_bank(line).map(|bank| find_largest(bank, 12)).unwrap_or(0))
        .sum();
    println!("Result: {}", result);
}

fn parse_bank(line: &str) -> Option<Vec<u32>> {
    
    if line.bytes().len() == 0 {
        return None;
    }
    return Some(line.chars().filter_map(|c| c.to_digit(10)).collect());
}

fn find_largest(bank: Vec<u32>, batteries: i32) -> i64 {
    let mut result: i64 = 0;
    let mut anchor_idx: usize = 0;
    for battery_num in 1..=batteries {
        let mut largest: u32 = 0;
        let last_idx = bank.len() - 1 - ((batteries - battery_num) as usize);
        for i in anchor_idx..=last_idx {
            if bank[i] > largest {
                largest = bank[i];
                anchor_idx = i + 1;
            }
        }
        result += 10_i64.pow((batteries - battery_num) as u32) * (largest as i64);
    }
    return result;
}
