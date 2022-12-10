use std::fs::read_to_string;

fn main() {
    let mut x: i32 = 1;
    let mut v = None;
    let mut cycle: u16 = 0;
    let mut sum = 0;
    let contents = read_to_string("input").unwrap();
    let mut lines = contents.lines();
    loop {
        cycle += 1;
        if (cycle + 20).rem_euclid(40) == 0 {
            sum += cycle as i32 * x;
        }
        if let Some(num) = v {
            x += num;
            v = None;
        } else {
            if let Some(line) = lines.next() {
                let mut cmd = line.split_whitespace();
                if cmd.next().unwrap() == "addx" {
                    let num: i32 = cmd.next().unwrap().parse().unwrap();
                    v = Some(num);
                }
            } else {
                break;
            }
        }
    }
    println!("{}", sum)
}
