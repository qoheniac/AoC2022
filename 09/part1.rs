use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let mut head_position = [0i32, 0i32];
    let mut tail_position = [0i32, 0i32];
    let mut where_tail_was = HashSet::from([tail_position.clone()]);
    let contents = read_to_string("input").unwrap();
    for line in contents.lines() {
        let motion: Vec<&str> = line.split_whitespace().collect();
        let head_velocity = match motion[0] {
            "R" => [1, 0],
            "U" => [0, 1],
            "L" => [-1, 0],
            "D" => [0, -1],
            _ => panic!(),
        };
        let head_speed: u8 = motion[1].parse().unwrap();
        for _ in 0..head_speed {
            head_position[0] += head_velocity[0];
            head_position[1] += head_velocity[1];
            for [i, j] in [[0, 1], [1, 0]] {
                if (head_position[i] - tail_position[i]).abs() == 2 {
                    tail_position[i] += head_velocity[i];
                    tail_position[j] = head_position[j];
                }
            }
            where_tail_was.insert(tail_position.clone());
        }
    }
    println!("{}", where_tail_was.len())
}
