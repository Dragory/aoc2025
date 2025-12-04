use std::env;
use std::fs;

type Diagram = Vec<DiagramRow>;
type DiagramRow = Vec<i8>;

static KERNEL_WIDTH: usize = 3;
static KERNEL_HEIGHT: usize = 3;
static KERNEL: [[i8; KERNEL_WIDTH]; KERNEL_HEIGHT] = [
    [1, 1, 1],
    [1, 0, 1],
    [1, 1, 1],
];

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

    let diagram = parse_diagram(&contents);
    match part.as_str() {
        "part1" => part1(&diagram),
        "part2" => part2(&diagram),
        x => panic!("Invalid part: {}", x),
    }
}

fn parse_diagram(contents: &str) -> Diagram {
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

fn part1(diagram: &Diagram) {
    let (cnt, _) = find_accessible_rolls(diagram);
    println!("Result: {}", cnt);
}

fn part2(diagram: &Diagram) {
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
    println!("Result: {}", result);
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
