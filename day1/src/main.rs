use std::fs;
fn main() {
    let (max_calories_single_elf, max_calories_three_elves) = get_max_calories();
    println!("Max calories");
    println!("Single elf: {}", max_calories_single_elf);
    println!("Three elves: {}", max_calories_three_elves);
}

fn get_max_calories() -> (u32, u32) {
    let all_elves_calories_str: String = read_calory_data_file();
    let elves_calories_str: Vec<&str> = all_elves_calories_str.split("\n\n").collect();

    let mut elves_calories: Vec<u32> = elves_calories_str
        .iter()
        .map(|calory_str| calculate_elves_calories(calory_str))
        .collect();

    elves_calories.sort();
    elves_calories.reverse();

    (elves_calories[0], elves_calories[0..3].iter().sum())
}

fn read_calory_data_file() -> String {
    let contents =
        fs::read_to_string("calory_data.txt").expect("Something went wrong reading the file");
    contents
}

fn calculate_elves_calories(calory_string: &str) -> u32 {
    let calories: Vec<&str> = calory_string.split("\n").collect();
    calories
        .iter()
        .fold(0, |acc, calory| acc + calory.parse::<u32>().unwrap_or(0))
}

#[test]
fn test_day_1() {
    let (max_calories_single_elf, max_calories_three_elves) = get_max_calories();
    assert_eq!(max_calories_single_elf, 66306);
    assert_eq!(max_calories_three_elves, 195292);
}
