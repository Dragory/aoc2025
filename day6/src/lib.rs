use regex::Regex;

#[derive(Debug)]
struct Calculation<'a> {
    numbers: Vec<i64>,
    operator: &'a str,
}

pub fn part1<'a>(input: &str) -> i64 {
    let split_regex = Regex::new(r"\s+").unwrap();

    let calculations = input
        .lines()
        .fold(Vec::<Calculation>::new(), |list, line| {
            split_regex
                .split(line.trim())
                .enumerate()
                .fold(list, |mut list, (idx, str)| {
                    if idx >= list.len() {
                        list.push(Calculation { numbers: vec![], operator: "?" });
                    }
                    match str {
                        "*" | "+" => list[idx].operator = str,
                        _ => list[idx].numbers.push(str.parse::<i64>().unwrap()),
                    };
                    list
                })
        });
    
    calculations.iter()
        .fold(0_i64, |result, calc| {
            match calc.operator {
                "+" => result + calc.numbers.iter().sum::<i64>(),
                "*" => result + calc.numbers.iter().product::<i64>(),
                _ => result,
            }
        })
}

trait HasColumns {
    fn columns(&self) -> Vec<String>;
}

impl HasColumns for String {
    fn columns(&self) -> Vec<String> {
        let mut columns: Vec<String> = vec![];
        for line in self.lines() {
            for (idx, char) in line.chars().enumerate() {
                if idx >= columns.len() {
                    columns.push(String::from(""));
                }
                columns[idx].push(char);
            }
        }
        return columns;
    }
}

pub fn part2<'a>(input: &str) -> i64 {
    let split_regex = Regex::new(r"\s+").unwrap();
    let is_blank_line = Regex::new(r"^\s+$").unwrap();

    let lines: Vec<&str> = input.lines().collect();
    let numbers = lines[..lines.len() - 1].join("\n");
    let operators: Vec<&str> = split_regex
        .split(lines[lines.len() - 1].trim())
        .collect();

    let mut calculations: Vec<Calculation> = vec![];
    let mut current_calc = Calculation { numbers: vec![], operator: "?" };
    let number_cols = numbers.columns();
    for col in &number_cols {
        if is_blank_line.is_match(col) {
            current_calc.operator = operators[calculations.len()];
            calculations.push(current_calc);
            current_calc = Calculation { numbers: vec![], operator: "?" };
            continue;
        }
        current_calc.numbers.push(col.trim().parse::<i64>().unwrap());
    }
    current_calc.operator = operators[calculations.len()];
    calculations.push(current_calc);

    calculations.iter()
        .fold(0_i64, |result, calc| {
            match calc.operator {
                "+" => result + calc.numbers.iter().sum::<i64>(),
                "*" => result + calc.numbers.iter().product::<i64>(),
                _ => result,
            }
        })
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn part1_test() {
        let input = fs::read_to_string("input/testinput.txt").unwrap();
        assert_eq!(super::part1(&input), 4277556);
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(super::part1(&input), 5381996914800);
    }

    #[test]
    fn part2_test() {
        let input = fs::read_to_string("input/testinput.txt").unwrap();
        assert_eq!(super::part2(&input), 3263827);
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(super::part2(&input), 9627174150897);
    }
}
