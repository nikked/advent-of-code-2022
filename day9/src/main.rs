use std::fs;
use std::iter;
fn main() {
    let (res1, res2) = process_data();
    println!("Result 1: {}", res1);
    println!("Result 2: {}", res2);
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn process_data() -> (u32, u32) {
    let input_data = fs::read_to_string("input_data.txt").unwrap();
    let _input_rows: Vec<&str> = input_data
        .split("\n")
        .filter(|input_str| !input_str.is_empty())
        .collect();

    let directions: Vec<Direction> = parse_input_rows_to_movement(_input_rows);

    (
        calculate_count_of_unique_tail_positions(&directions, 2),
        calculate_count_of_unique_tail_positions(&directions, 10),
    )
}

fn calculate_count_of_unique_tail_positions(directions: &Vec<Direction>, rope_length: i32) -> u32 {
    let tail_positions = simulate_rope_movement(&directions, rope_length);
    let tail_positions_set = tail_positions
        .iter()
        .collect::<std::collections::HashSet<_>>();
    tail_positions_set.len() as u32
}

fn parse_input_rows_to_movement(input_rows: Vec<&str>) -> Vec<Direction> {
    let mut rope_movement: Vec<Direction> = Vec::new();

    for row in input_rows {
        let mut chars = row.chars();
        let direction_char = chars.next().unwrap();
        let distance = chars.as_str().trim().parse::<usize>().unwrap();

        let direction = match direction_char {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unknown direction"),
        };

        let movement_vec: Vec<Direction> = iter::repeat(direction).take(distance).collect();
        rope_movement.extend(movement_vec);
    }
    rope_movement
}

fn simulate_rope_movement(directions: &Vec<Direction>, rope_length: i32) -> Vec<(i32, i32)> {
    let mut knot_positions: Vec<(i32, i32)> =
        iter::repeat((0, 0)).take(rope_length as usize).collect();
    let mut visited_positions_tail: Vec<(i32, i32)> = vec![];

    for direction in directions {
        match direction {
            Direction::Up => {
                knot_positions[0].1 += 1;
            }
            Direction::Down => {
                knot_positions[0].1 -= 1;
            }
            Direction::Left => {
                knot_positions[0].0 -= 1;
            }
            Direction::Right => {
                knot_positions[0].0 += 1;
            }
        }

        for index in 1..knot_positions.len() {
            knot_positions[index] =
                calculate_new_knot_position(knot_positions[index - 1], knot_positions[index]);
        }

        visited_positions_tail.push(knot_positions.last().unwrap().clone());
    }
    visited_positions_tail
}

fn calculate_new_knot_position(
    new_prev_position: (i32, i32),
    current_knot_position: (i32, i32),
) -> (i32, i32) {
    let distance_x = (new_prev_position.0 - current_knot_position.0) as f32;
    let distance_y = (new_prev_position.1 - current_knot_position.1) as f32;
    let distance = (distance_x.powi(2) + distance_y.powi(2)).sqrt();

    if distance >= 1.0 {
        let new_tail_position = (
            new_prev_position.0 - (distance_x / distance).round() as i32,
            new_prev_position.1 - (distance_y / distance).round() as i32,
        );

        return new_tail_position;
    }
    current_knot_position
}

#[test]
fn test_day() {
    let (res1, res2) = process_data();
    assert_eq!(res1, 6209);
    assert_eq!(res2, 2460);
}

#[test]
fn test_parse_input_rows_to_movement() {
    let input_rows = vec!["U 1", "R 2", "D 3", "L 4"];
    let rope_movement = parse_input_rows_to_movement(input_rows);
    assert_eq!(rope_movement.len(), 10);
    assert_eq!(
        rope_movement,
        vec![
            Direction::Up,
            Direction::Right,
            Direction::Right,
            Direction::Down,
            Direction::Down,
            Direction::Down,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left
        ]
    );
}
