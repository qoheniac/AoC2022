use std::{collections::HashMap, fs::read_to_string};

#[derive(Eq, Hash, PartialEq)]
enum Material {
    Ore,
    Clay,
    Obs,
}
use Material::*;

#[derive(Eq, Hash, PartialEq)]
enum Robot {
    OreBot,
    ClayBot,
    ObsBot,
    GeodeBot,
}
use Robot::*;

type Resources = HashMap<Material, u8>;
type Blueprint = HashMap<Robot, Resources>;

fn main() {
    let contents = read_to_string("input").unwrap();
    let mut blueprints: HashMap<u8, Blueprint> = HashMap::new();
    contents.lines().for_each(|line| {
        let vals: Vec<u8> = line
            .split(|c: char| !c.is_numeric())
            .filter_map(|s| s.parse().ok())
            .collect();
        blueprints.insert(
            vals[0],
            HashMap::from([
                (OreBot, HashMap::from([(Ore, vals[1])])),
                (ClayBot, HashMap::from([(Ore, vals[2])])),
                (ObsBot, HashMap::from([(Ore, vals[3]), (Clay, vals[4])])),
                (GeodeBot, HashMap::from([(Ore, vals[5]), (Obs, vals[6])])),
            ]),
        );
    });
}
