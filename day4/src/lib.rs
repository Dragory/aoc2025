use std::collections::HashSet;

pub type Diagram = Vec<DiagramRow>;
type DiagramRow = Vec<i8>;

static KERNEL_WIDTH: usize = 3;
static KERNEL_HEIGHT: usize = 3;
static KERNEL: [[i8; KERNEL_WIDTH]; KERNEL_HEIGHT] = [
    [1, 1, 1],
    [1, 0, 1],
    [1, 1, 1],
];

pub fn parse_diagram(contents: &str) -> Diagram {
    let mut diagram: Diagram = vec![];
    for line in contents.lines() {
        let mut row: DiagramRow = vec![];
        for char in line.chars() {
            row.push(if char == '@' { 1 } else { 0 });
        }
        diagram.push(row);
    }
    return diagram;
}

pub fn part1(diagram: &Diagram) -> usize {
    let (cnt, _) = find_accessible_rolls(diagram);
    return cnt;
}

pub fn part2(diagram: &Diagram) -> usize {
    let mut result = 0;
    let mut current_diagram = diagram.clone();
    loop {
        let (cnt, new_diagram) = find_accessible_rolls(&current_diagram);
        if cnt == 0 {
            break;
        }
        result += cnt;
        current_diagram = new_diagram;
    }
    return result;
}

fn find_accessible_rolls(diagram: &Diagram) -> (usize, Diagram) {
    let mut accessible_roll_cnt = 0;
    let mut new_diagram: Diagram = vec![];
    for (y, row) in diagram.iter().enumerate() {
        let mut new_row: DiagramRow = vec![];
        for (x, cell) in row.iter().enumerate() {
            if *cell == 0 {
                new_row.push(0);
                continue;
            }
            if count_kernel_overlap(diagram, x, y) < 4 {
                accessible_roll_cnt += 1;
                new_row.push(0);
            } else {
                new_row.push(1);
            }
        }
        new_diagram.push(new_row);
    }
    return (accessible_roll_cnt, new_diagram);
}

fn count_kernel_overlap(lines: &Diagram, x: usize, y: usize) -> usize {
    let mut result: usize = 0;
    for kernel_y in 0..KERNEL_HEIGHT {
        let Some(line_index) = (y + kernel_y).checked_sub(KERNEL_HEIGHT / 2).filter(|v| *v < lines.len()) else {
            continue;
        };
        for kernel_x in 0..KERNEL_WIDTH {
            let Some(cell_index) = (x + kernel_x).checked_sub(KERNEL_WIDTH / 2).filter(|x| *x < lines[line_index].len()) else {
                continue;
            };
            if KERNEL[kernel_y][kernel_x] == 1 && lines[line_index][cell_index] == 1 {
                result += 1;
            }
        }
    }
    return result;
}

#[derive(PartialEq, Eq, Hash)]
struct Roll {
    x: isize,
    y: isize,
}

pub fn part1_set(input: &str) -> i32 {
    let rolls = input_to_set(input);
    let mut result = 0;
    for roll in &rolls {
        if calculate_surrounding_rolls(&rolls, &roll) < 4 {
            result += 1;
        }
    }
    return result;
}

pub fn part2_set(input: &str) -> i32 {
    let mut rolls = input_to_set(input);
    let mut result = 0;
    loop {
        let mut new_rolls: HashSet<Roll> = HashSet::new();
        let mut this_result = 0;
        for roll in &rolls {
            if calculate_surrounding_rolls(&rolls, &roll) < 4 {
                this_result += 1;
            } else {
                new_rolls.insert(Roll { x: roll.x, y: roll.y });
            }
        }
        if this_result == 0 {
            break;
        }
        result += this_result;
        rolls = new_rolls;
    }
    return result;
}

fn input_to_set(input: &str) -> HashSet<Roll> {
    let mut rolls: HashSet<Roll> = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '@' {
                rolls.insert(Roll {
                    x: (x as isize),
                    y: (y as isize),
                });
            }
        }
    }
    return rolls;
}

fn calculate_surrounding_rolls(rolls: &HashSet<Roll>, roll: &Roll) -> i32 {
    let mut cnt = 0;
    for x_mod in -1..=1 {
        for y_mod in -1..=1 {
            if x_mod == 0 && y_mod == 0 {
                continue;
            }
            if rolls.contains(&Roll { x: roll.x - x_mod, y: roll.y - y_mod }) {
                cnt += 1;
            }
        }
    }
    return cnt;
}


#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input/input.txt");

    #[test]
    fn part1_test() {
        let diagram = parse_diagram(INPUT);
        assert_eq!(part1(&diagram), 1491);
    }

    #[test]
    fn part2_test() {
        let diagram = parse_diagram(INPUT);
        assert_eq!(part2(&diagram), 8722);
    }

    #[test]
    fn part1_set_test() {
        assert_eq!(part1_set(INPUT), 1491);
    }

    #[test]
    fn part2_set_test() {
        assert_eq!(part2_set(INPUT), 8722);
    }
}
