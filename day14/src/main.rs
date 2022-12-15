use std::fs;

// TODO
// THis solution is quite a verbose terrible mess and should be refactored xD

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}

fn main() {
    let (res1, res2) = process_data();
    println!("Result 1: {}", res1);
    println!("Result 2: {}", res2);
}

fn process_data() -> (i32, i32) {
    let splitter = "\n";
    let input_data = fs::read_to_string("input_data.txt").unwrap();
    let rock_blocks: Vec<Vec<Coord>> = input_data
        .split(splitter)
        .filter(|input_str| !input_str.is_empty())
        .map(|input_str| calculate_rock_block(input_str))
        .collect();

    let mut rock_grid_a = calculate_rock_grid(&rock_blocks, false);

    let start: i32 = 500;

    rock_grid_a[0][start as usize] = -1;
    let mut sanddrops_before_void_a = 0;

    let mut i_a = 0;
    loop {
        let mut next_coord = Coord { x: start, y: 0 };
        println!("Iteration: {}", i_a + 1);
        loop {
            let (new_sand_block, is_next_coord) = simulate_sand_flowing(&rock_grid_a, next_coord);

            if new_sand_block.x == 0 && new_sand_block.y == 0 {
                println!("Bottom found!");
                sanddrops_before_void_a = i_a;
                break;
            }

            if !is_next_coord {
                rock_grid_a[new_sand_block.y as usize][new_sand_block.x as usize] = 2;
                break;
            }
            next_coord = new_sand_block;
        }
        if sanddrops_before_void_a > 0 {
            break;
        }
        i_a += 1;
    }

    let mut rock_grid_b = calculate_rock_grid(&rock_blocks, true);

    rock_grid_b[0][start as usize] = -1;
    let mut sanddrops_before_void_b = 0;

    let mut i_b = 0;
    let mut is_full = false;
    loop {
        let mut next_coord = Coord { x: start, y: 0 };
        println!("Iteration: {}", i_b + 1);
        loop {
            let (new_sand_block, is_next_coord) = simulate_sand_flowing(&rock_grid_b, next_coord);

            if new_sand_block.x == 0 && new_sand_block.y == 0 {
                println!("Bottom found!");
                sanddrops_before_void_b = i_b;
                break;
            }

            if !is_next_coord {
                if rock_grid_b[new_sand_block.y as usize][new_sand_block.x as usize] == -1 {
                    sanddrops_before_void_b = i_b + 1;
                    is_full = true;
                    break;
                }
                rock_grid_b[new_sand_block.y as usize][new_sand_block.x as usize] = 2;
                break;
            }
            next_coord = new_sand_block;
        }
        if sanddrops_before_void_b > 0 {
            break;
        }
        if is_full {
            break;
        }
        i_b += 1;
    }

    let res1 = sanddrops_before_void_a;
    let res2 = sanddrops_before_void_b;
    for line in rock_grid_b[..].iter() {
        println!("{:?}", line[450..600].to_vec());
    }

    (res1, res2)
}

fn simulate_sand_flowing(rock_grid: &Vec<Vec<i32>>, sand_origin: Coord) -> (Coord, bool) {
    let mut first_blocking_found = Coord { x: 0, y: 0 };

    for (index, row) in rock_grid[sand_origin.y as usize..].iter().enumerate() {
        if row[sand_origin.x as usize] > 0 {
            first_blocking_found = Coord {
                x: sand_origin.x,
                y: index as i32 + sand_origin.y as i32,
            };
            break;
        }
    }

    if first_blocking_found.y == 0 && first_blocking_found.x == 0 {
        return (Coord { x: 0, y: 0 }, true);
    }

    let blocking_to_right =
        rock_grid[first_blocking_found.y as usize][first_blocking_found.x as usize + 1] > 0;

    let blocking_to_left =
        rock_grid[first_blocking_found.y as usize][first_blocking_found.x as usize - 1] > 0;

    if blocking_to_left && blocking_to_right {
        return (
            Coord {
                x: first_blocking_found.x,
                y: first_blocking_found.y - 1,
            },
            false,
        );
    }

    if !blocking_to_left {
        return (
            Coord {
                x: first_blocking_found.x - 1,
                y: first_blocking_found.y + 1,
            },
            true,
        );
    }

    return (
        Coord {
            x: first_blocking_found.x + 1,
            y: first_blocking_found.y + 1,
        },
        true,
    );
}

fn calculate_rock_block(input_str: &str) -> Vec<Coord> {
    let coordinate_tuple_strings = input_str.split(" -> ").collect::<Vec<&str>>();

    coordinate_tuple_strings
        .iter()
        .map(|tuple_string| {
            let coordinates = tuple_string.split(",").collect::<Vec<&str>>();
            let x = coordinates[0].parse::<i32>().unwrap();
            let y = coordinates[1].parse::<i32>().unwrap();
            Coord { x, y }
        })
        .collect()
}

fn calculate_rock_grid(rock_blocks: &Vec<Vec<Coord>>, make_rock_bottom: bool) -> Vec<Vec<i32>> {
    let mut rock_grid = vec![vec![0; 1000]; 183];

    let highest_y = rock_blocks
        .iter()
        .map(|block| block.iter().map(|coord| coord.y).max().unwrap())
        .max()
        .unwrap();

    for rock_block in rock_blocks {
        for index in 1..rock_block.len() {
            let first_block = &rock_block[index - 1];
            let second_block = &rock_block[index];

            // calculate straight line between first and second block
            // and set all coordinates to 1

            let x_diff = second_block.x - first_block.x;
            let y_diff = second_block.y - first_block.y;

            if x_diff == 0 {
                // vertical line
                let y_start = if first_block.y < second_block.y {
                    first_block.y
                } else {
                    second_block.y
                };

                let y_end = if first_block.y > second_block.y {
                    first_block.y
                } else {
                    second_block.y
                };

                for y in y_start..y_end {
                    rock_grid[y as usize][first_block.x as usize] = 1;
                }
            } else if y_diff == 0 {
                // horizontal line
                let x_start = if first_block.x < second_block.x {
                    first_block.x
                } else {
                    second_block.x
                };

                let x_end = if first_block.x > second_block.x {
                    first_block.x
                } else {
                    second_block.x
                };

                for x in x_start..x_end + 1 {
                    rock_grid[first_block.y as usize][x as usize] = 1;
                }
            }
        }
    }

    if make_rock_bottom {
        for x in 0..rock_grid[0].len() {
            rock_grid[highest_y as usize + 2][x] = 1;
        }
    }

    rock_grid
}

#[test]
fn test_day() {
    let (res1, res2) = process_data();
    assert_eq!(res1, 692);
    assert_eq!(res2, 31706);
}
