use std::fs;
fn main() {
    let (res1, res2) = process_data();
    println!("Result 1: {}", res1);
    println!("Result 2: {}", res2);
}

fn process_data() -> (u32, u32) {
    let grid: Vec<Vec<u32>> = fs::read_to_string("input_data.txt")
        .unwrap()
        .split("\n")
        .filter(|input_str| !input_str.is_empty())
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    // A
    let mut non_visible_cells = 0;
    for (y, row) in grid.iter().enumerate() {
        for x in 0..row.len() {
            let visible = is_cell_visible((x, y), &grid);
            if !visible {
                non_visible_cells += 1;
            }
        }
    }

    // B
    let mut max_scenic_score = 0;
    for (y, row) in grid.iter().enumerate() {
        for x in 0..row.len() {
            let scenic_score = calculate_scenic_score((x, y), &grid);
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    (non_visible_cells, max_scenic_score)
}

fn is_cell_visible(cell: (usize, usize), grid: &Vec<Vec<u32>>) -> bool {
    let cell_value = grid[cell.0][cell.1];

    let left_slice = &grid[cell.0][..cell.1];
    let right_slice = &grid[cell.0][cell.1 + 1..];

    let column: Vec<u32> = grid.iter().map(|row| row[cell.1]).collect();

    let top_slice = &column[..cell.0];
    let bottom_slice = &column[cell.0 + 1..];

    any_item_is_gte(left_slice, cell_value)
        && any_item_is_gte(right_slice, cell_value)
        && any_item_is_gte(top_slice, cell_value)
        && any_item_is_gte(bottom_slice, cell_value)
}

fn any_item_is_gte(items: &[u32], target: u32) -> bool {
    for item in items {
        if item >= &target {
            return true;
        }
    }

    false
}

fn calculate_scenic_score(cell: (usize, usize), grid: &Vec<Vec<u32>>) -> u32 {
    let cell_value = grid[cell.0][cell.1];

    let mut left_slice = grid[cell.0][..cell.1].to_vec();
    left_slice.reverse();
    let right_slice = grid[cell.0][cell.1 + 1..].to_vec();

    let column: Vec<u32> = grid.iter().map(|row| row[cell.1]).collect();

    let mut top_slice = column[..cell.0].to_vec();
    top_slice.reverse();
    let bottom_slice = column[cell.0 + 1..].to_vec();

    return amount_of_items_from_start_that_are_lte(left_slice, cell_value)
        * amount_of_items_from_start_that_are_lte(right_slice, cell_value)
        * amount_of_items_from_start_that_are_lte(top_slice, cell_value)
        * amount_of_items_from_start_that_are_lte(bottom_slice, cell_value);
}

fn amount_of_items_from_start_that_are_lte(items: Vec<u32>, target: u32) -> u32 {
    for (i, item) in items.iter().enumerate() {
        if item >= &target {
            return (i + 1) as u32;
        }
    }

    items.len() as u32
}

#[test]
fn test_day() {
    let (res1, res2) = process_data();
    assert_eq!(res1, 1803);
    assert_eq!(res2, 268912);
}
