use std::{collections::HashSet, fs::read_to_string};

fn main() {
    const ROPE_LENGTH: usize = 10;
    let mut rope_positions = [[0i32, 0i32]; ROPE_LENGTH];
    let mut where_tail_was = HashSet::from([rope_positions[ROPE_LENGTH - 1].clone()]);
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
            rope_positions[0][0] += head_velocity[0];
            rope_positions[0][1] += head_velocity[1];
            for k in 0..(ROPE_LENGTH - 1) {
                for [i, j] in [[0, 1], [1, 0]] {
                    if (rope_positions[k][i] - rope_positions[k + 1][i]).abs() == 2 {
                        rope_positions[k + 1][i] =
                            (rope_positions[k][i] + rope_positions[k + 1][i]) / 2;
                        if (rope_positions[k][j] - rope_positions[k + 1][j]).abs() == 2 {
                            rope_positions[k + 1][j] =
                                (rope_positions[k][j] + rope_positions[k + 1][j]) / 2;
                        } else {
                            rope_positions[k + 1][j] = rope_positions[k][j];
                        }
                    }
                }
            }
            where_tail_was.insert(rope_positions[ROPE_LENGTH - 1].clone());
        }
    }
    println!("{}", where_tail_was.len())
}
