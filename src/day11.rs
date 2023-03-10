use crate::day::Part;

pub fn run(part: Part, file_path: &str) {
    match part {
        Part::Part1 => part1(file_path),
        Part::Part2 => part2(file_path),
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Monkey {
    items: Vec<i32>,
    operation: fn(old: i32) -> i32,
    test_divisible_by: i32,
    target_monkey_if_true: usize,
    target_monkey_if_false: usize,
}

fn get_monkeys() -> Vec<Monkey> {
    // There's not enough monkeys to need to parse this automatically
    vec![
        Monkey {
            items: vec![59, 74, 65, 86],
            operation: |old| old * 19,
            test_divisible_by: 7,
            target_monkey_if_true: 6,
            target_monkey_if_false: 2,
        },
        Monkey {
            items: vec![62, 84, 72, 91, 68, 78, 51],
            operation: |old| old + 1,
            test_divisible_by: 2,
            target_monkey_if_true: 2,
            target_monkey_if_false: 0,
        },
        Monkey {
            items: vec![78, 84, 96],
            operation: |old| old + 8,
            test_divisible_by: 19,
            target_monkey_if_true: 6,
            target_monkey_if_false: 5,
        },
        Monkey {
            items: vec![97, 86],
            operation: |old| old * old,
            test_divisible_by: 3,
            target_monkey_if_true: 1,
            target_monkey_if_false: 0,
        },
        Monkey {
            items: vec![50],
            operation: |old| old + 6,
            test_divisible_by: 13,
            target_monkey_if_true: 3,
            target_monkey_if_false: 1,
        },
        Monkey {
            items: vec![73, 65, 69, 65, 51],
            operation: |old| old * 17,
            test_divisible_by: 11,
            target_monkey_if_true: 4,
            target_monkey_if_false: 7,
        },
        Monkey {
            items: vec![69, 82, 97, 93, 82, 84, 58, 63],
            operation: |old| old + 5,
            test_divisible_by: 5,
            target_monkey_if_true: 5,
            target_monkey_if_false: 7,
        },
        Monkey {
            items: vec![81, 78, 82, 76, 79, 80],
            operation: |old| old + 3,
            test_divisible_by: 17,
            target_monkey_if_true: 3,
            target_monkey_if_false: 4,
        },
    ]
}

fn calc_inspections(monkeys: &mut Vec<Monkey>) -> Vec<usize> {
    let mut inspect_count = Vec::with_capacity(monkeys.len());
    for _ in 0..monkeys.len() {
        inspect_count.push(0);
    }

    for _ in 0..20 {
        for monkey_index in 0..monkeys.len() {
            // I hate rust
            for item_index in 0..monkeys[monkey_index].items.len() {
                // What the fuck is this code even...
                // Just because of Rust's stupid fuckin borrowing shit
                let (item, next_index) = {
                    let monkey = &monkeys[monkey_index];
                    let new = (monkey.operation)(monkey.items[item_index]) / 3;
                    let next_index = if (new % monkey.test_divisible_by) == 0 {
                        monkey.target_monkey_if_true
                    } else {
                        monkey.target_monkey_if_false
                    };
                    (new, next_index)
                };

                let next = &mut monkeys[next_index];
                next.items.push(item);
            }
            inspect_count[monkey_index] += monkeys[monkey_index].items.len();
            monkeys[monkey_index].items.clear();
        }
    }

    inspect_count
}

fn part1(_file_path: &str) {
    println!("Ignoring input file. Going with hardcoded values.");
    let mut monkeys = get_monkeys();
    let inspect_count = calc_inspections(&mut monkeys);

    let mut inspect_count_with_index: Vec<(usize, usize)> = Vec::with_capacity(inspect_count.len());
    for (i, count) in inspect_count.iter().enumerate() {
        println!("Monkey {} inspected items {} times.", i, count);
        inspect_count_with_index.push((i, count.clone()));
    }
    inspect_count_with_index.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let high_a = inspect_count_with_index[0];
    let high_b = inspect_count_with_index[1];
    println!(
        "Monkey {} and {} inspected the most items.",
        high_a.0, high_b.0
    );
    println!(
        "Their counts multiplied: {} * {} = {}",
        high_a.1,
        high_b.1,
        high_a.1 * high_b.1
    );
}

fn part2(_file_path: &str) {
    println!("Ignoring input file. Going with hardcoded values.");
}

#[cfg(test)]
mod tests {
    use crate::day11::{calc_inspections, Monkey};

    #[test]
    fn test_example() {
        let mut monkeys = vec![
            Monkey {
                items: vec![79, 98],
                operation: |old| old * 19,
                test_divisible_by: 23,
                target_monkey_if_true: 2,
                target_monkey_if_false: 3,
            },
            Monkey {
                items: vec![54, 65, 75, 74],
                operation: |old| old + 6,
                test_divisible_by: 19,
                target_monkey_if_true: 2,
                target_monkey_if_false: 0,
            },
            Monkey {
                items: vec![79, 60, 97],
                operation: |old| old * old,
                test_divisible_by: 13,
                target_monkey_if_true: 1,
                target_monkey_if_false: 3,
            },
            Monkey {
                items: vec![74],
                operation: |old| old + 3,
                test_divisible_by: 17,
                target_monkey_if_true: 0,
                target_monkey_if_false: 1,
            },
        ];
        let inspect_count = calc_inspections(&mut monkeys);
        assert_eq!(inspect_count, vec![101, 95, 7, 105,]);
    }
}
