use std::fs;

fn main() {
    let (rucksack_sum, team_sum) = calculate_rucksack_data();
    println!("Rucksack sum: {}", rucksack_sum);
    println!("Team sum: {}", team_sum);
}

fn calculate_rucksack_data() -> (i32, i32) {
    let rucksack_file_data = fs::read_to_string("rucksack_data.txt").expect("Unable to read file");

    let ruck_sack_vec: Vec<&str> = rucksack_file_data
        .split('\n')
        .into_iter()
        .filter(|row| !row.is_empty())
        .collect();

    // A
    let rucksack_row_sums: i32 = ruck_sack_vec
        .clone()
        .into_iter()
        .map(calculate_row_sum)
        .sum();

    // B
    let team_scores: i32 = ruck_sack_vec
        .chunks(3)
        .map(|elf_team| {
            get_chars_score(find_first_common_char_in_three_strings(
                elf_team[0],
                elf_team[1],
                elf_team[2],
            ))
        })
        .sum();

    (rucksack_row_sums, team_scores)
}

fn calculate_row_sum(row: &str) -> i32 {
    let first_half = row[0..row.len() / 2].to_string();
    let second_half = row[row.len() / 2..].to_string();
    let common_char = find_first_common_char_in_strings(&first_half, &second_half);
    get_chars_score(common_char)
}

fn get_chars_score(c: char) -> i32 {
    let priorities = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    (priorities.find(c).unwrap() + 1) as i32
}

fn find_first_common_char_in_strings(first: &String, second: &String) -> char {
    for c in first.chars() {
        if second.contains(c) {
            return c;
        }
    }
    panic!("No common chars found");
}

fn find_first_common_char_in_three_strings(first: &str, second: &str, third: &str) -> char {
    for c in first.chars() {
        if second.contains(c) && third.contains(c) {
            return c;
        }
    }
    panic!("No common chars found in triplet")
}

#[test]
fn test_calculate_row_sum() {
    let (individual_sum, group_sum) = calculate_rucksack_data();
    assert_eq!(individual_sum, 7908);
    assert_eq!(group_sum, 2838);
}
