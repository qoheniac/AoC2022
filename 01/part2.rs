use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let mut top_calories = vec!(0, 0, 0);
    for elf in contents.split("\n\n") {
        let mut sum = 0;
        for line in elf.lines() {
            let calories: i32 = line.parse().unwrap();
            sum += calories;
        }
        top_calories.push(sum);
        top_calories.sort();
        top_calories.remove(0);
    }
    println!("{}", top_calories.iter().sum::<i32>())
}
