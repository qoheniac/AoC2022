use std::fs::read_to_string;

struct Monkey {
    items: Vec<u64>,
    operator: char,
    operand: String,
    divisor: u64,
    receivers: Vec<usize>,
    inspected: u64,
}

fn main() {
    let mut monkeys: Vec<Monkey> = read_to_string("input")
        .unwrap()
        .split("\n\n")
        .map(|monkey_input| {
            let mut lines = monkey_input.lines();
            lines.next().unwrap();

            let items: Vec<u64> = lines
                .next()
                .unwrap()
                .split(": ")
                .skip(1)
                .next()
                .unwrap()
                .split(", ")
                .map(|item| item.parse::<u64>().unwrap())
                .collect();

            let operation_data: Vec<&str> = lines
                .next()
                .unwrap()
                .split(" = ")
                .skip(1)
                .next()
                .unwrap()
                .split_whitespace()
                .skip(1)
                .collect();

            let divisor: u64 = lines
                .next()
                .unwrap()
                .split_whitespace()
                .skip(3)
                .next()
                .unwrap()
                .parse()
                .unwrap();

            let receivers: Vec<usize> = lines
                .take(2)
                .map(|line| {
                    line.split_whitespace()
                        .skip(5)
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap()
                })
                .collect();

            Monkey {
                items: items,
                operator: operation_data[0].chars().next().unwrap(),
                operand: operation_data[1].to_string(),
                divisor: divisor,
                receivers: receivers,
                inspected: 0,
            }
        })
        .collect();

    let lcm: u64 = monkeys.iter().map(|monkey| monkey.divisor).product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            loop {
                if let Some(mut item) = monkeys[i].items.pop() {
                    monkeys[i].inspected += 1;

                    // inspect
                    let operand;
                    if monkeys[i].operand == "old" {
                        operand = item;
                    } else {
                        operand = monkeys[i].operand.parse().unwrap();
                    }
                    item = match monkeys[i].operator {
                        '+' => item + operand,
                        '*' => item * operand,
                        _ => panic!(),
                    } % lcm;

                    // throw
                    let j;
                    if item.rem_euclid(monkeys[i].divisor) == 0 {
                        j = monkeys[i].receivers[0]
                    } else {
                        j = monkeys[i].receivers[1]
                    }
                    monkeys[j].items.push(item);
                } else {
                    break;
                }
            }
        }
    }

    monkeys.sort_unstable_by_key(|monkey| monkey.inspected);
    monkeys.reverse();
    println!("{}", monkeys[0].inspected * monkeys[1].inspected)
}
