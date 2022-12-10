use std::fs;

fn main() {
    let res1 = process_data();
    println!("Result 1: {}", res1);
}

fn process_data() -> i32 {
    let input_data = fs::read_to_string("input_data.txt").unwrap();
    let input_rows: Vec<&str> = input_data
        .split("\n")
        .filter(|input_str| !input_str.is_empty())
        .collect();

    let register_positions: Vec<i32> = parse_register_additions(input_rows)
        .iter()
        .scan(1, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect();

    let signal_strengths = calculate_signal_strenghts(&register_positions);

    let sum_of_signal_strengths = signal_strengths[19]
        + signal_strengths[59]
        + signal_strengths[99]
        + signal_strengths[139]
        + signal_strengths[179]
        + signal_strengths[219];

    for chunk in register_positions.chunks(40) {
        draw_chunk(chunk);
    }

    sum_of_signal_strengths
}

fn parse_register_additions(input_rows: Vec<&str>) -> Vec<i32> {
    let mut cycles: Vec<i32> = vec![0];

    for row in input_rows {
        cycles.push(0);
        if row != "noop" {
            cycles.push(row.split(" ").last().unwrap().parse::<i32>().unwrap());
        }
    }

    cycles
}

fn calculate_signal_strenghts(register_values: &Vec<i32>) -> Vec<i32> {
    let mut signal_strengths: Vec<i32> = Vec::new();
    for (i, register_value) in register_values.iter().enumerate() {
        signal_strengths.push(register_value * ((i + 1) as i32));
    }
    signal_strengths
}

fn draw_chunk(chunk: &[i32]) {
    for (i, register_position) in chunk.iter().enumerate() {
        let index = i as i32;
        let sprite_position: Vec<i32> = vec![index, index - 1, index + 1];
        if sprite_position.contains(register_position) {
            print!("#");
        } else {
            print!(".");
        }
    }
    println!();
}

#[test]
fn test_day() {
    let res1 = process_data();
    assert_eq!(res1, 14760);
}
