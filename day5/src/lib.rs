use std::cmp;

pub fn part1(input: &str) -> usize {
    let parsed_input = parse_input(input);
    let mut result = 0;
    'idloop: for id in parsed_input.ids {
        for range in &parsed_input.ranges {
            if range[0] > id {
                continue 'idloop;
            }
            if range[1] >= id {
                result += 1;
                continue 'idloop;
            }
        }
    }
    return result;
}

pub fn part2(input: &str) -> i64 {
    let parsed_input = parse_input(input);
    let mut merged_ranges: Vec<[i64; 2]> = vec![];
    'range_loop: for range in &parsed_input.ranges {
        for merged_range in merged_ranges.iter_mut().rev() {
            if merged_range[1] < range[0] {
                continue 'range_loop;
            }
            if merged_range[0] <= range[1] && merged_range[1] >= range[0] {
                merged_range[0] = cmp::min(merged_range[0], range[0]);
                merged_range[1] = cmp::max(merged_range[1], range[1]);
                continue 'range_loop;
            }
        }
        merged_ranges.push(range.clone());
    }
    let mut result: i64 = 0;
    for range in merged_ranges {
        result += range[1] - range[0] + 1;
    }
    return result;
}

#[derive(Debug)]
struct ParsedInput {
    ranges: Vec<[i64; 2]>,
    ids: Vec<i64>,
}

fn parse_input(input: &str) -> ParsedInput {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut ranges: Vec<[i64; 2]> = vec![];
    for line in parts[0].lines() {
        let bounds: Vec<i64> = line.split("-").map(|s| s.parse::<i64>().unwrap()).collect();
        ranges.push([bounds[0], bounds[1]]);
    }
    ranges.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut ids: Vec<i64> = vec![];
    for line in parts[1].lines() {
        ids.push(line.parse::<i64>().unwrap());
    }
    return ParsedInput {
        ranges,
        ids,
    };
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn part1_test() {
        let input = fs::read_to_string("input/testinput.txt").unwrap();
        assert_eq!(super::part1(&input), 3);
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(super::part1(&input), 758);
    }

    #[test]
    fn part2_test() {
        let input = fs::read_to_string("input/testinput.txt").unwrap();
        assert_eq!(super::part2(&input), 14);
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(super::part2(&input), 343143696885053);
    }
}
