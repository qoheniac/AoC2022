use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("input").unwrap();
    let mut supply_rep = vec![0];
    for snafu in contents.lines() {
        let mut snafu_rep = Vec::new();
        for c in snafu.chars().rev() {
            snafu_rep.push(match c {
                '=' => -2,
                '-' => -1,
                num => num.to_digit(3).unwrap() as i8,
            });
        }
        let mut carry = 0;
        for i in 0..snafu_rep.len().max(supply_rep.len()) {
            let mut mem = 2 + carry;
            if i < supply_rep.len() {
                mem += supply_rep[i];
            }
            if i < snafu_rep.len() {
                mem += snafu_rep[i];
            }
            let digit_rep = mem.rem_euclid(5) - 2;
            if i < supply_rep.len() {
                supply_rep[i] = digit_rep;
            } else {
                supply_rep.push(digit_rep);
            }
            carry = mem.div_euclid(5)
        }
        if carry == 1 {
            supply_rep.push(1);
        }
    }
    let mut supply = String::new();
    for &digit_rep in supply_rep.iter().rev() {
        supply.push(match digit_rep {
            -2 => '=',
            -1 => '-',
            num_rep => char::from_digit(num_rep as u32, 3).unwrap(),
        });
    }
    println!("{}", supply)
}
