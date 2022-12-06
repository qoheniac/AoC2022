use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let mut content_blocks = contents.split("\n\n");

    // parse starting stacks
    let starting_stacks = content_blocks.next().unwrap();
    let mut starting_rows = starting_stacks.lines().rev();
    let numbers_row = starting_rows.next().unwrap();
    let mut stack_indices = Vec::new();
    let mut stacks = Vec::new();
    let mut index: usize = 0;
    for character in numbers_row.chars() {
        if character.is_numeric() {
            stack_indices.push(index);
            stacks.push(Vec::new());
        }
        index += 1;
    }
    for row in starting_rows {
        let row_chars: Vec<char> = row.chars().collect();
        for i in 0..stack_indices.len() {
            let character = row_chars[stack_indices[i]];
            if character.is_alphabetic() {
                stacks[i].push(character);
            }
        }
    }

    // parse and do rearrangements
    let rearrangements = content_blocks.next().unwrap();
    for rearrangement in rearrangements.lines() {
        let words: Vec<&str> = rearrangement.split_whitespace().collect();
        let number: u32 = words[1].parse().unwrap();
        let from = words[3].parse::<usize>().unwrap() - 1;
        let dest = words[5].parse::<usize>().unwrap() - 1;
        for _ in 0..number {
            let crt = stacks[from].pop().unwrap();
            stacks[dest].push(crt);
        }
    }

    // costruct output
    let mut output = String::new();
    for stack in stacks {
        output.push(*stack.last().unwrap());
    }
    println!("{}", output)
}
