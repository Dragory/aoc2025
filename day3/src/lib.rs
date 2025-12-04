pub fn part1(lines: &Vec<&str>) -> i64 {
    lines.iter()
        .map(|line| parse_bank(line).map(|bank| find_largest(bank, 2)).unwrap_or(0))
        .sum()
}

pub fn part2(lines: &Vec<&str>) -> i64 {
    lines.iter()
        .map(|line| parse_bank(line).map(|bank| find_largest(bank, 12)).unwrap_or(0))
        .sum()
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

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../input/testinput.txt");
    const MAIN_INPUT: &str = include_str!("../input/input.txt");

    #[test]
    fn part1_test() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        assert_eq!(part1(&lines), 357_i64);
    }

    #[test]
    fn part2_test() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        assert_eq!(part2(&lines), 3121910778619_i64);
    }
    
    #[test]
    fn part1_main() {
        let lines: Vec<&str> = MAIN_INPUT.lines().collect();
        assert_eq!(part1(&lines), 17311_i64);
    }
    
    #[test]
    fn part2_main() {
        let lines: Vec<&str> = MAIN_INPUT.lines().collect();
        assert_eq!(part2(&lines), 171419245422055_i64);
    }
}
