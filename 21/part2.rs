use std::{collections::HashMap, fs::read_to_string};

#[derive(Clone, Copy)]
enum Job<'a> {
    Number(Option<u64>),
    Computation(&'a str, &'a str, &'a str),
}
use Job::*;

fn apply_op(op: &str, num1: u64, num2: u64) -> u64 {
    match op {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => panic!(),
    }
}

fn yell<'a>(monkey: &'a str, jobs: &mut HashMap<&'a str, Job<'a>>) -> Option<u64> {
    let job = *jobs.get(monkey).unwrap();
    match job {
        Number(opt) => opt,
        Computation(op, monkey1, monkey2) => {
            let mut opt = None;
            if let Some(num1) = yell(monkey1, jobs) {
                if let Some(num2) = yell(monkey2, jobs) {
                    opt = Some(apply_op(op, num1, num2));
                    jobs.insert(monkey, Number(opt));
                }
            }
            opt
        }
    }
}

fn find_humn_number<'a>(monkey: &'a str, ans: u64, jobs: &mut HashMap<&'a str, Job<'a>>) -> u64 {
    let job = *jobs.get(monkey).unwrap();
    if let Computation(op, monkey1, monkey2) = job {
        let mut next_monkey = monkey1;
        let other_ans = if let Some(num) = yell(monkey1, jobs) {
            next_monkey = monkey2;
            num
        } else {
            yell(monkey2, jobs).unwrap()
        };
        let next_ans = match op {
            "+" => ans - other_ans, // ans = next + other (or v.v.)
            "-" => {
                if next_monkey == monkey1 {
                    ans + other_ans // ans = next - other
                } else {
                    other_ans - ans // ans = other - next
                }
            }
            "*" => ans / other_ans, // ans = next * other (or v.v.)
            "/" => {
                if next_monkey == monkey1 {
                    ans * other_ans // ans = next / other
                } else {
                    other_ans / ans // ans = other / next
                }
            }
            _ => panic!(),
        };
        if next_monkey == "humn" {
            next_ans
        } else {
            find_humn_number(next_monkey, next_ans, jobs)
        }
    } else {
        panic!();
    }
}

fn main() {
    let contents = read_to_string("input").unwrap();
    let mut jobs = HashMap::new();
    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let name = parts[0].strip_suffix(':').unwrap();
        jobs.insert(
            name,
            if parts.len() == 2 {
                Number(Some((*parts[1]).parse::<u64>().unwrap()))
            } else {
                let op = if name == "root" { "-" } else { parts[2] };
                Computation(op, parts[1], parts[3])
            },
        );
    }
    jobs.insert("humn", Number(None));
    println!("{}", find_humn_number("root", 0, &mut jobs))
}
