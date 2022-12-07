use std::collections::HashSet;
use std::fs;
fn main() {
    let (res1, res2) = process_data();
    println!("Result 1: {}", res1);
    println!("Result 2: {}", res2);
}

fn process_data() -> (u32, u32) {
    let input_data = fs::read_to_string("input_data.txt").unwrap();

    let res1 = find_index_of_start_of_packet_marker(&input_data, 4);
    let res2 = find_index_of_start_of_packet_marker(&input_data, 14);

    (res1, res2)
}

fn find_index_of_start_of_packet_marker(input: &String, slice_size: usize) -> u32 {
    for i in 0..input.len() {
        let char_set: HashSet<char> = input[i..i + slice_size].chars().collect();
        if char_set.len() == slice_size {
            return (i + slice_size) as u32;
        }
    }
    panic!("No start of packet marker found")
}

#[test]
fn test_day() {
    let (res1, res2) = process_data();
    assert_eq!(res1, 1651);
    assert_eq!(res2, 3837);
}
