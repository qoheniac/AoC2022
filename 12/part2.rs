use std::{
    collections::HashSet,
    fs::read_to_string,
    io::{prelude::*, stdout},
};

fn elevation(c: char) -> u32 {
    match c {
        'S' => 0,
        'E' => 25,
        other => other.to_digit(36).unwrap() - 10,
    }
}

fn main() {
    // read grid
    let contents = read_to_string("input").unwrap();
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let height = grid.len();
    let width = grid[0].len();

    // select start point and provide some rough progress information
    let mut shortest = usize::MAX;
    for i_start in 0..height {
        print!("\r{:3.0}%", 100 * i_start / (height - 1));
        stdout().flush().unwrap();
        for j_start in 0..width {
            if grid[i_start][j_start] == 'a' {
                // initialize Dijkstra algorithm
                let mut distance = vec![vec![usize::MAX; width]; height];
                let mut to_visit: HashSet<[usize; 2]> = HashSet::new();
                distance[i_start][j_start] = 0;
                to_visit.insert([i_start, j_start]);

                'dijkstra: loop {
                    if to_visit.is_empty() {
                        break;
                    }

                    // visit location with shortest path to get there
                    let p_short = to_visit
                        .iter()
                        .min_by_key(|p| distance[p[0]][p[1]])
                        .cloned()
                        .unwrap();
                    to_visit.remove(&p_short);
                    let [i_short, j_short] = p_short;
                    let e_short = elevation(grid[i_short][j_short]);

                    // loop over reachable destinations
                    for (m, n) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
                        let i = (i_short as i32 + m) as usize;  // -1 becomes usize::MAX and thus …
                        let j = (j_short as i32 + n) as usize;  // … larger than width or height
                        if i < height && j < width && elevation(grid[i][j]) <= e_short + 1 {
                            let alt = distance[i_short][j_short] + 1;

                            // final destination reached
                            if grid[i][j] == 'E' {
                                if alt < shortest {
                                    shortest = alt;
                                }
                                break 'dijkstra;
                            }

                            // update distance
                            if distance[i][j] == usize::MAX {
                                distance[i][j] = alt;
                                to_visit.insert([i, j]);
                            } else if alt < distance[i][j] {
                                distance[i][j] = alt;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("\r    \r{}", shortest)
}
