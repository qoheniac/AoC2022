use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

type Coord = i32;
type Point = [Coord; 2];

// return [NE, E, SE]
fn east(&[i, j]: &Point) -> [Point; 3] {
    [[i - 1, j + 1], [i, j + 1], [i + 1, j + 1]]
}

// return [NW, N, NE]
fn north(&[i, j]: &Point) -> [Point; 3] {
    [[i - 1, j - 1], [i - 1, j], [i - 1, j + 1]]
}

// return [SW, S, SE]
fn south(&[i, j]: &Point) -> [Point; 3] {
    [[i + 1, j - 1], [i + 1, j], [i + 1, j + 1]]
}

// return [NW, W, SW]
fn west(&[i, j]: &Point) -> [Point; 3] {
    [[i - 1, j - 1], [i, j - 1], [i + 1, j - 1]]
}

fn main() {
    let compass = [north, south, west, east]; // order of considered directions

    // read elf positions
    let contents = read_to_string("input").unwrap();
    let mut elves = HashSet::new();
    for (i, line) in contents.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert([i as Coord, j as Coord]);
            }
        }
    }

    // main loop
    let mut round = 0;
    loop {
        let mut from_to = HashMap::new(); // collect valid movements
        let mut to_from = HashMap::new(); // remember first elf to consider some destination
        for elf in &elves {
            let [i, j] = elf;

            // see what neighbor positions are blocked by elves
            let mut blocked_neighbors = HashSet::new();
            for di in [-1, 0, 1] {
                for dj in [-1, 0, 1] {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    let neighbor = [i + di, j + dj];
                    if elves.contains(&neighbor) {
                        blocked_neighbors.insert(neighbor);
                    }
                }
            }

            // if non are blocked do nothing
            if blocked_neighbors.is_empty() {
                continue;
            }

            // consider movements
            'directions: for i in 0..4 {
                let neighbors = compass[(round + i) % 4](elf);

                // skip invalid directions
                for neighbor in neighbors {
                    if blocked_neighbors.contains(&neighbor) {
                        continue 'directions;
                    }
                }

                // check for conflicts and stop considering directions
                let to = neighbors[1];
                if let Some(other_elf) = to_from.get(&to) {
                    // conflict
                    from_to.remove(other_elf);
                } else {
                    // no conflict yet
                    from_to.insert(elf.clone(), to);
                    to_from.insert(to, *elf);
                }
                break;
            }
        }

        // quit when elves come to rest
        round += 1;
        if from_to.is_empty() {
            println!("{}", round);
            break;
        }

        // perform movements
        for (from, to) in from_to {
            elves.remove(&from);
            elves.insert(to);
        }
    }
}
