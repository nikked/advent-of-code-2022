use std::fs;
fn main() {
    let (res1, res2) = process_data();
    println!("Result 1: {}", res1);
    println!("Result 2: {}", res2);
}

fn process_data() -> (u32, u32) {
    let splitter = "\n\n";
    let input_data = fs::read_to_string("input_data.txt").unwrap();
    let _input_rows: Vec<&str> = input_data
        .split(splitter)
        .filter(|input_str| !input_str.is_empty())
        .collect();

    let res1 = 32;
    let res2 = 42;

    (res1, res2)
}

#[test]
fn test_day() {
    let (res1, res2) = process_data();
    assert_eq!(res1, 32);
    assert_eq!(res2, 42);
}
