use std::{collections::HashSet, fs::read_to_string};

enum Direction {
    Down,
    Left,
    Right,
    Up,
}
use Direction::*;

// auxiliary function used to avoid negative numbers
fn mirror(i: usize, length: usize) -> usize {
    length.checked_sub(i + 1).unwrap()
}

struct Blizzard {
    start_position: [usize; 2], // row and column inside the valey
    direction: Direction,
}

// method returning the location of a blizzard for a given time and length of the valley
impl Blizzard {
    fn position(&self, time: usize, length: usize) -> [usize; 2] {
        let [i, j] = self.start_position;
        let m = |i| mirror(i, length);
        match self.direction {
            Left => [i, m((m(j) + time) % length)],
            Right => [i, (j + time) % length],
            Down => [(i + time) % length, j],
            Up => [m((m(i) + time) % length), j],
        }
    }
}

fn main() {
    let contents = read_to_string("input").unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let width = lines[0].len() - 2;
    let height = lines.len() - 2;

    // collect all blizzards in a vector
    let mut blizzards = Vec::new();
    for i in 0..height {
        let line = lines[i + 1]
            .strip_prefix("#")
            .unwrap()
            .strip_suffix("#")
            .unwrap();
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                blizzards.push(Blizzard {
                    start_position: [i, j],
                    direction: match c {
                        'v' => Down,
                        '<' => Left,
                        '>' => Right,
                        '^' => Up,
                        _ => panic!(),
                    },
                });
            }
        }
    }

    // collect states to consider in a set of [row, column, time] arrays
    let mut consider = HashSet::new();
    consider.insert([0, 0, 1]); // enter the valey without waiting
    'search: loop { // consider states in the order of time necessary to reach them
        let [i, j, t] = *consider.iter().min_by_key(|a| a[2]).unwrap();
        consider.remove(&[i, j, t]);
        consider.insert([0, 0, t + 1]); // enter the valey later

        // if a blizzard occupies this state consider the next one
        for blizzard in &blizzards {
            let length = match blizzard.direction {
                Down | Up => height,
                Left | Right => width,
            };
            if blizzard.position(t, length) == [i, j] {
                continue 'search;
            }
        }

        // stop if the state is located in front of the exit
        if [i, j] == [height - 1, width - 1] {
            println!("{}", t+1);
            break 'search;
        }

        // consider states located next to the current state
        consider.insert([i, j, t + 1]);
        if i < height - 1 {
            consider.insert([i + 1, j, t + 1]);
        }
        if j > 0 {
            consider.insert([i, j - 1, t + 1]);
        }
        if j < width - 1 {
            consider.insert([i, j + 1, t + 1]);
        }
        if i > 0 {
            consider.insert([i - 1, j, t + 1]);
        }
    }
}
