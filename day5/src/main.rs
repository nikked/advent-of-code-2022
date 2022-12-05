use regex::Regex;
use std::fs;
#[derive(Debug, PartialEq)]
struct StackMove {
    amount: u32,
    source: u32,
    target: u32,
}

fn main() {
    let (res1, res2) = process_data();
    println!("Result 1: {}", res1);
    println!("Result 2: {}", res2);
}

fn process_data() -> (String, String) {
    let input_data = fs::read_to_string("input_data.txt").unwrap();
    let splitter = "\n";
    let _input_rows: Vec<&str> = input_data
        .split(splitter)
        .filter(|input_str| !input_str.is_empty())
        .collect();

    let moves: Vec<StackMove> = _input_rows.iter().map(|row| parse_move_row(row)).collect();

    // A
    let mut stacks1 = get_initial_stack_situation();
    for stack_move in &moves {
        apply_move(&mut stacks1, stack_move, true);
    }
    let res_string1 = stacks1
        .iter()
        .fold(String::new(), |acc, m| acc + &m[0].to_string());

    // B
    let mut stacks2 = get_initial_stack_situation();
    for stack_move in &moves {
        apply_move(&mut stacks2, stack_move, false);
    }
    let res_string2 = stacks2
        .iter()
        .fold(String::new(), |acc, m| acc + &m[0].to_string());

    (res_string1, res_string2)
}

fn get_initial_stack_situation() -> Vec<Vec<char>> {
    vec![
        vec!['W', 'T', 'H', 'P', 'J', 'C', 'F'],
        vec!['H', 'B', 'J', 'Z', 'F', 'V', 'R', 'G'],
        vec!['R', 'T', 'P', 'H'],
        vec!['T', 'H', 'P', 'N', 'S', 'Z'],
        vec!['D', 'C', 'J', 'H', 'Z', 'F', 'V', 'N'],
        vec!['Z', 'D', 'W', 'F', 'G', 'M', 'P'],
        vec!['P', 'D', 'J', 'S', 'W', 'Z', 'V', 'M'],
        vec!['S', 'D', 'N'],
        vec!['M', 'F', 'S', 'Z', 'D'],
    ]
}

fn parse_move_row(move_row: &str) -> StackMove {
    let re = Regex::new(r"move (\d{1,2}) from (\d{1}) to (\d{1})").unwrap();
    let regex_result = re.captures_iter(move_row).next().unwrap();
    StackMove {
        amount: regex_result[1].parse::<u32>().unwrap(),
        source: regex_result[2].parse::<u32>().unwrap() - 1,
        target: regex_result[3].parse::<u32>().unwrap() - 1,
    }
}

fn apply_move(stack: &mut Vec<Vec<char>>, stack_move: &StackMove, reverse: bool) {
    let target_row = stack[(stack_move.target) as usize].to_owned();
    let source_row = stack[(stack_move.source) as usize].to_owned();

    let mut moved_blocks = source_row[..(stack_move.amount as usize)].to_owned();

    if reverse {
        moved_blocks.reverse();
    }

    stack[(stack_move.target as usize)] = [moved_blocks, target_row].concat();
    stack[(stack_move.source as usize)] = source_row[(stack_move.amount as usize)..].to_owned();
}

#[test]
fn test_day() {
    let (res1, res2) = process_data();
    assert_eq!(res1, "SPFMVDTZT");
    assert_eq!(res2, "ZFSJBPRFP");
}

#[test]
fn test_parse_move_row() {
    assert_eq!(
        parse_move_row("move 35 from 4 to 9"),
        StackMove {
            amount: 35,
            source: 3,
            target: 8
        }
    )
}
