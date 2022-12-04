use std::fs;

struct Range {
    min: u32,
    max: u32,
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

    // A
    let res1: u32 = _input_rows
        .iter()
        .map(|item| parse_range_members(item))
        .map(|item| pair_member_fully_contains(item))
        .map(|item| if item { 1 } else { 0 })
        .sum();

    // B
    let res2: u32 = _input_rows
        .iter()
        .map(|item| parse_range_members(item))
        .map(|item| ranges_overlap(item))
        .map(|item| if item { 1 } else { 0 })
        .sum();

    (res1, res2)
}

fn parse_range_members(row: &str) -> (Range, Range) {
    fn get_min_max_from_range(range: &str) -> Range {
        let range_members: Vec<&str> = range.split("-").collect();
        Range {
            min: range_members[0].parse::<u32>().unwrap(),
            max: range_members[1].parse::<u32>().unwrap(),
        }
    }
    let ranges: Vec<&str> = row.split(",").collect();
    let first_member = get_min_max_from_range(ranges[0]);
    let second_member = get_min_max_from_range(ranges[1]);
    (first_member, second_member)
}

fn pair_member_fully_contains(range_members: (Range, Range)) -> bool {
    fn range_is_inside_another_range(inside_range: &Range, outside_range: &Range) -> bool {
        inside_range.min >= outside_range.min && inside_range.max <= outside_range.max
    }
    range_is_inside_another_range(&range_members.0, &range_members.1)
        || range_is_inside_another_range(&range_members.1, &range_members.0)
}

fn ranges_overlap(ranges: (Range, Range)) -> bool {
    let range1 = ranges.0;
    let range2 = ranges.1;
    let lowest_max = if range1.max < range2.max {
        range1.max
    } else {
        range2.max
    };
    let highest_min = if range1.min > range2.min {
        range1.min
    } else {
        range2.min
    };

    highest_min <= lowest_max
}

#[test]
fn test_day2() {
    let (res1, res2) = process_data();
    assert_eq!(res1, 500);
    assert_eq!(res2, 815);
}
