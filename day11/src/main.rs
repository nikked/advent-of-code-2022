#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u128>,
    items_inspected: u128,
    operation: fn(u128) -> u128,
    test_function: fn(u128) -> u128,
}

fn main() {
    let (res1, res2) = process_data();
    println!("Result 1: {}", res1);
    println!("Result 2: {}", res2);
}

fn process_data() -> (u128, u128) {
    let res1 = run_simulation(20, |x| x / 3);

    // The magic number is calculated by multiplying the divisors of the monkeys together:
    // 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 = 9699690
    let res2 = run_simulation(10000, |x| x % 9_699_690);

    (res1, res2)
}

fn run_simulation(number_of_rounds: u128, worry_level_divisor: fn(u128) -> u128) -> u128 {
    let mut monkeys = get_monkeys();

    for _ in 0..number_of_rounds {
        monkeys = simulate_round(&mut monkeys, worry_level_divisor).to_vec();
    }

    let mut items_inspected: Vec<u128> = monkeys.iter().map(|m| m.items_inspected).collect();
    items_inspected.sort();

    let two_highest_multiplied =
        items_inspected[items_inspected.len() - 1] * items_inspected[items_inspected.len() - 2];

    two_highest_multiplied
}

fn simulate_round(
    monkeys: &mut Vec<Monkey>,
    worry_level_divisor: fn(u128) -> u128,
) -> &mut Vec<Monkey> {
    for monkey_index in 0..monkeys.len() {
        *monkeys = simulate_monkey(monkey_index, &monkeys, worry_level_divisor);
    }
    monkeys
}

fn simulate_monkey(
    monkey_index: usize,
    monkeys: &Vec<Monkey>,
    _worry_level_divisor: fn(u128) -> u128,
) -> Vec<Monkey> {
    let monkey = monkeys[monkey_index].clone();
    let mut monkeys_clone = monkeys.clone();
    let amount_of_items = monkey.items.len();
    for item in monkey.items {
        let new_worry_level = _worry_level_divisor((monkey.operation)(item));
        let target_monkey_index = (monkey.test_function)(new_worry_level);
        let mut target_monkey = monkeys_clone[target_monkey_index as usize].clone();
        target_monkey.items.push(new_worry_level);
        monkeys_clone[target_monkey_index as usize] = target_monkey;
    }
    monkeys_clone[monkey_index as usize] = Monkey {
        items: vec![],
        operation: monkey.operation,
        test_function: monkey.test_function,
        items_inspected: monkey.items_inspected + amount_of_items as u128,
    };
    monkeys_clone
}

fn get_monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![97, 81, 57, 57, 91, 61],
            operation: |old| old * 7,
            test_function: |value| if value % 11 == 0 { 5 } else { 6 },
            items_inspected: 0,
        },
        Monkey {
            items: vec![88, 62, 68, 90],
            operation: |old| old * 17,
            test_function: |value| if value % 19 == 0 { 4 } else { 2 },
            items_inspected: 0,
        },
        Monkey {
            items: vec![74, 87],
            operation: |old| old + 2,
            test_function: |value| if value % 5 == 0 { 7 } else { 4 },
            items_inspected: 0,
        },
        Monkey {
            items: vec![53, 81, 60, 87, 90, 99, 75],
            operation: |old| old + 1,
            test_function: |value| if value % 2 == 0 { 2 } else { 1 },
            items_inspected: 0,
        },
        Monkey {
            items: vec![57],
            operation: |old| old + 6,
            test_function: |value| if value % 13 == 0 { 7 } else { 0 },
            items_inspected: 0,
        },
        Monkey {
            items: vec![54, 84, 91, 55, 59, 72, 75, 70],
            operation: |old| old.pow(2),
            test_function: |value| if value % 7 == 0 { 6 } else { 3 },
            items_inspected: 0,
        },
        Monkey {
            items: vec![95, 79, 79, 68, 78],
            operation: |old| old + 3,
            test_function: |value| if value % 3 == 0 { 1 } else { 3 },
            items_inspected: 0,
        },
        Monkey {
            items: vec![61, 97, 67],
            operation: |old| old + 4,
            test_function: |value| if value % 17 == 0 { 0 } else { 5 },
            items_inspected: 0,
        },
    ]
}

#[test]
fn test_day() {
    let (res1, res2) = process_data();
    assert_eq!(res1, 56350);
    assert_eq!(res2, 13954061248);
}
