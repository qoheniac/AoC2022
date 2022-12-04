use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let mut max_calories = 0;
    for elf in contents.split("\n\n") {
        let mut sum = 0;
        for line in elf.lines() {
            let calories: i32 = line.parse().unwrap();
            sum += calories;
        }
        if sum > max_calories {
            max_calories = sum;
        }
    }
    println!("{}", max_calories)
}
