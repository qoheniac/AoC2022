use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let contents = read_to_string("input").unwrap();
    let cubes: HashSet<[i8; 3]> = contents
        .lines()
        .map(|line| {
            let mut coords = line.split(",");
            [(); 3].map(|_| coords.next().unwrap().parse().unwrap())
        })
        .collect();
    let mut surface = 0;
    for cube in &cubes {
        for dim in 0..3 {
            for delta in [-1, 1] {
                let mut neighbor = *cube;
                neighbor[dim] += delta;
                if !cubes.contains(&neighbor) {
                    surface += 1;
                }
            }
        }
    }
    println!("{}", surface)
}
