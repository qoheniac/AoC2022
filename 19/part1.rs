use std::{collections::HashMap, fs::read_to_string};

enum MaterialCost {
    Ore(u8),
    Clay(u8),
    Obsidian(u8),
}

struct Blueprint {
    ore_robot_costs: Vec<MaterialCost>,
    clay_robot_costs: Vec<MaterialCost>,
    obsidian_robot_costs: Vec<MaterialCost>,
    geode_robot_costs: Vec<MaterialCost>,
}

fn main() {
    let contents = read_to_string("input").unwrap();
    let mut blueprints = HashMap::new();
    contents.lines().for_each(|line| {
        let numbers: Vec<u8> = line
            .split(|c: char| !c.is_numeric())
            .filter_map(|s| s.parse().ok())
            .collect();
        blueprints.insert(
            numbers[0],
            Blueprint {
                ore_robot_costs: vec![MaterialCost::Ore(numbers[1])],
                clay_robot_costs: vec![MaterialCost::Ore(numbers[2])],
                obsidian_robot_costs: vec![
                    MaterialCost::Ore(numbers[3]),
                    MaterialCost::Clay(numbers[4]),
                ],
                geode_robot_costs: vec![
                    MaterialCost::Ore(numbers[5]),
                    MaterialCost::Obsidian(numbers[6]),
                ],
            },
        );
    });
}
