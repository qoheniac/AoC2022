use std::fs;

fn priority(item: char) -> u32 {
    let mut priority = item.to_digit(36).unwrap() - 9;
    if item.is_ascii_uppercase() {
        priority += 26
    }
    priority
}

fn main() {
    let mut sum = 0;
    let contents = fs::read_to_string("input").unwrap();
    for line in contents.lines() {
        let mid_index = line.len() / 2;
        let compartment1 = &line[..mid_index];
        let compartment2 = &line[mid_index..];
        for item in compartment1.chars() {
            if compartment2.contains(item) {
                sum += priority(item);
                break;
            }
        }
    }
    println!("{}", sum);
}
