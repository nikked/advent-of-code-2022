use regex::Regex;
use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq)]
struct DirInfo {
    file_sizes_in_dir_total: u32,
    children: Vec<String>,
}

fn main() {
    let (res1, res2) = process_data();
    println!("Result 1: {}", res1);
    println!("Result 2: {}", res2);
}

fn process_data() -> (u32, u32) {
    let splitter = "$ ";
    let input_data = fs::read_to_string("input_data.txt").unwrap();
    let input_rows: Vec<&str> = input_data
        .split(splitter)
        .filter(|input| !input.is_empty())
        .collect();

    let dir_sizes = get_directory_size_map(&parse_commands(input_rows));

    // A
    let dir_size_threshold = 100000;
    let sum_of_dirs_under_threshold = dir_sizes
        .values()
        .filter(|size| size <= &&dir_size_threshold)
        .sum();

    // B
    let total_file_space = 70000000;
    let total_file_space_needed = 30000000;
    let total_file_space_available = total_file_space - dir_sizes.get("/").unwrap();
    let need_to_free_up = total_file_space_needed - total_file_space_available;

    let mut smallest_dir_size_that_frees_up_space = total_file_space;
    for size in dir_sizes.values() {
        if size >= &need_to_free_up && size < &smallest_dir_size_that_frees_up_space {
            smallest_dir_size_that_frees_up_space = *size;
        }
    }

    (
        sum_of_dirs_under_threshold,
        smallest_dir_size_that_frees_up_space,
    )
}

fn parse_commands(commands: Vec<&str>) -> HashMap<String, DirInfo> {
    let mut node_hashmap: HashMap<String, DirInfo> = HashMap::new();
    let mut current_dir_path: Vec<String> = vec![];

    let change_to_dir_regex = Regex::new(r"cd \w*").unwrap();
    let go_up_dir_regex = Regex::new(r"cd \.\.").unwrap();

    for command in commands {
        // Parse cd command
        if command.starts_with("cd") {
            if go_up_dir_regex.is_match(command) {
                current_dir_path.pop();
            } else if change_to_dir_regex.is_match(command) {
                let target_dir = command.split("cd ").last().unwrap().replace("\n", "");
                current_dir_path.push(target_dir.to_string());
            }
        } else {
            // Parse ls command
            let node = parse_ls_command(command, &current_dir_path);
            node_hashmap.insert(current_dir_path.join("/"), node);
        }
    }
    node_hashmap
}

fn parse_ls_command(command: &str, current_dir_path: &Vec<String>) -> DirInfo {
    let mut children_vec: Vec<String> = vec![];
    let mut total_size: u32 = 0;
    for line in command.split("\n") {
        if line == "ls" || line.is_empty() {
            continue;
        }

        if line.starts_with("dir") {
            let child = line.split("dir ").last().unwrap();
            children_vec.push(format!(
                "{}/{}",
                current_dir_path.join("/"),
                child.to_string()
            ));
        } else {
            let size: u32 = line.split(" ").next().unwrap().parse().unwrap();
            total_size += size;
        }
    }

    DirInfo {
        file_sizes_in_dir_total: total_size,
        children: children_vec,
    }
}

fn get_directory_size_map(node_hashmap: &HashMap<String, DirInfo>) -> HashMap<String, u32> {
    let mut recursive_sizes = HashMap::new();
    for key in node_hashmap.keys() {
        recursive_sizes.insert(key.to_string(), count_size_of_directory(key, node_hashmap));
    }
    recursive_sizes
}

fn count_size_of_directory(node_name: &String, node_hashmap: &HashMap<String, DirInfo>) -> u32 {
    let mut size = node_hashmap.get(node_name).unwrap().file_sizes_in_dir_total;
    for child in &node_hashmap.get(node_name).unwrap().children {
        size += count_size_of_directory(&child, node_hashmap);
    }
    size
}

#[test]
fn test_day() {
    let (res1, res2) = process_data();
    assert_eq!(res1, 1077191);
    assert_eq!(res2, 5649896);
}

#[test]
fn test_parse_node_from_command() {
    let arg = "ls\n193196 fgpnnvm\n22126 gwftf.wcr\n92461 hdh\n50807 qbdmzjd.jvg\ndir qrrmhwn\ndir rdcsmpfm\ndir rgl\n36742 zsgdbd.dmm\n";
    let res = parse_ls_command(arg, &vec![]);
    assert_eq!(
        res,
        DirInfo {
            file_sizes_in_dir_total: 193196 + 22126 + 92461 + 50807 + 36742,
            children: vec![
                "/qrrmhwn".to_string(),
                "/rdcsmpfm".to_string(),
                "/rgl".to_string(),
            ],
        },
    )
}
