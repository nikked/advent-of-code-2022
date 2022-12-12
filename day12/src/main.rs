use pathfinding::prelude::bfs;
use std::fs;

// X, Y
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn successors(&self, grid: &Vec<Vec<i32>>) -> Vec<Pos> {
        let &Pos(x, y) = self;
        let mut succ = Vec::new();
        let candidates = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
        let current_value = grid[y as usize][x as usize];

        for candidate in candidates {
            if candidate.0 < 0
                || candidate.1 < 0
                || candidate.0 >= grid[0].len() as i32
                || candidate.1 >= grid.len() as i32
            {
                continue;
            }
            let candidate_value = grid[candidate.1 as usize][candidate.0 as usize];

            if candidate_value <= current_value + 1 {
                succ.push(Pos(candidate.0, candidate.1));
            }
        }
        succ
    }
}

fn main() {
    let (res1, res2) = process_data();
    println!("Result 1: {}", res1);
    println!("Result 2: {}", res2);
}

fn process_data() -> (u32, u32) {
    let splitter = "\n";
    let input_data = fs::read_to_string("input_data.txt").unwrap();
    let _input_rows: Vec<&str> = input_data
        .split(splitter)
        .filter(|input_str| !input_str.is_empty())
        .collect();

    let grid = read_grid(_input_rows);
    static GOAL: Pos = Pos(136, 20);

    // A
    let start = Pos(0, 20);
    let result_a: Vec<Pos> = bfs(&start, |p| p.successors(&grid), |p| *p == GOAL).unwrap();
    let res1 = result_a.len() as u32 - 1;

    // B
    // find all squares in grid with value 0
    let mut start_positions = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if *value == 0 {
                start_positions.push(Pos(x as i32, y as i32));
            }
        }
    }

    let mut lowest_amount_of_steps = 100000 as u32;
    for start in start_positions {
        match bfs(&start, |p| p.successors(&grid), |p| *p == GOAL) {
            Some(result_b) => {
                let amount_of_steps = result_b.len() as u32 - 1;
                if amount_of_steps < lowest_amount_of_steps {
                    lowest_amount_of_steps = amount_of_steps;
                }
            }
            None => {}
        }
    }

    let res2 = lowest_amount_of_steps;

    (res1, res2)
}

fn read_grid(input_rows: Vec<&str>) -> Vec<Vec<i32>> {
    let mut grid: Vec<Vec<i32>> = Vec::new();

    for line in input_rows {
        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            let ci32 = c as i32 - 97;
            row.push(ci32);
        }
        grid.push(row);
    }

    grid
}

#[test]
fn test_day() {
    let (res1, res2) = process_data();
    assert_eq!(res1, 504);
    assert_eq!(res2, 42);
}
