use std::{collections::HashSet, fs::read_to_string};

fn main() {
    // read lava voxel locations
    let contents = read_to_string("input").unwrap();
    let lava: HashSet<[u8; 3]> = contents
        .lines()
        .map(|line| {
            let mut coords = line.split(",");
            [(); 3].map(|_| 1 + coords.next().unwrap().parse::<u8>().unwrap())
        })
        .collect();

    // fill surrounding cube with water raising surface by one when touching water
    let limit = 1 + lava.iter().flatten().reduce(|acc, e| acc.max(e)).unwrap();
    let mut water = HashSet::new();
    water.insert([0, 0, 0]);
    let mut visit = water.clone();
    let mut surface = 0;
    while !visit.is_empty() {
        let mut visit_later = HashSet::new();
        for voxel in visit.drain() {
            for dim in 0..3 {
                for delta in [-1, 1] {
                    let mut neighbor = voxel;
                    neighbor[dim] = (neighbor[dim] as i8 + delta) as u8;
                    if lava.contains(&neighbor) {
                        surface += 1;
                    } else if !water.contains(&neighbor) && neighbor.iter().all(|c| c <= &limit) {
                        water.insert(neighbor);
                        visit_later.insert(neighbor);
                    }
                }
            }
        }
        visit = visit_later;
    }
    println!("{}", surface)
}
