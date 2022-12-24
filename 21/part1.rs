use std::{
    collections::HashMap,
    fs::read_to_string,
    ops::{Add, Div, Mul, Sub},
};

type Operation = fn(u64, u64) -> u64;

#[derive(Clone, Copy)]
enum Job<'a> {
    Number(u64),
    Computation(Operation, &'a str, &'a str),
}
use Job::*;

fn parse_operation(symbol: &str) -> Operation {
    match symbol {
        "+" => u64::add,
        "-" => u64::sub,
        "*" => u64::mul,
        "/" => u64::div,
        _ => panic!(),
    }
}

fn yell<'a>(monkey: &'a str, jobs: &mut HashMap<&'a str, Job<'a>>) -> u64 {
    let job = *jobs.get(monkey).unwrap();
    match job {
        Number(num) => num,
        Computation(op, monkey1, monkey2) => {
            let num1 = yell(monkey1, jobs);
            let num2 = yell(monkey2, jobs);
            let num = op(num1, num2);
            jobs.insert(monkey, Number(num));
            num
        }
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
                Number((*parts[1]).parse::<u64>().unwrap())
            } else {
                Computation(parse_operation(parts[2]), parts[1], parts[3])
            },
        );
    }
    println!("{}", yell("root", &mut jobs))
}
