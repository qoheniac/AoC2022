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
    let mut rucksacks = contents.lines();
    loop {
        let rucksack1;
        match rucksacks.next() {
            Some(rucksack) => rucksack1 = rucksack,
            None => break,
        }
        let rucksack2 = rucksacks.next().unwrap();
        let rucksack3 = rucksacks.next().unwrap();
        for item in rucksack1.chars() {
            if rucksack2.contains(item) & rucksack3.contains(item) {
                sum += priority(item);
                break;
            }
        }
    }
    println!("{}", sum);
}
