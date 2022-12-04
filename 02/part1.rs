use std::{collections::HashMap, fs};

fn main() {
    let action_index: HashMap<&str, i32> =
        HashMap::from([("A", 0), ("B", 1), ("C", 2), ("X", 0), ("Y", 1), ("Z", 2)]);
    let contents = fs::read_to_string("input").unwrap();
    let mut score = 0;
    for round in contents.lines() {
        let mut actions = round.split_whitespace();
        let action_index_1 = action_index.get(actions.next().unwrap()).unwrap();
        let action_index_2 = action_index.get(actions.next().unwrap()).unwrap();
        score += action_index_2 + 1; // shape score
        score += match (action_index_2 - action_index_1).rem_euclid(3) {
            0 => 3, // draw
            1 => 6, // win
            _ => 0, // loose
        };
    }
    println!("{}", score)
}
