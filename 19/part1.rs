use std::{collections::HashMap, fs::read_to_string};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Material {
    Ore,
    Clay,
    Obs,
}
use Material::*;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Robot {
    OreBot,
    ClayBot,
    ObsBot,
    GeodeBot,
}
use Robot::*;
impl Robot {
    fn crack(self) -> Option<Material> {
        match self {
            OreBot => Some(Ore),
            ClayBot => Some(Clay),
            ObsBot => Some(Obs),
            GeodeBot => None,
        }
    }
}

type Robots = HashMap<Robot, u8>;
type Resources = HashMap<Material, u8>;
type Blueprint = HashMap<Robot, Resources>;

fn geodes_crackable(blueprint: &Blueprint, robots: &Robots, mut avails: Resources, time: u8) -> u8 {
    let mut now_cracked = 0;
    if time == 0 {
        return now_cracked;
    }
    for (&robot, &number) in robots {
        if let Some(material) = robot.crack() {
            avails
                .entry(material)
                .and_modify(|amount| *amount += number)
                .or_insert(number);
        } else {
            now_cracked += number;
        }
    }
    let mut later_cracked = geodes_crackable(blueprint, robots, avails.clone(), time - 1);
    for (&robot, costs) in blueprint {
        if costs
            .iter()
            .all(|(material, need)| matches!(avails.get(material), Some(have) if have >= need))
        {
            for (&material, need) in costs {
                let mut next_avails = avails.clone();
                next_avails.entry(material).and_modify(|have| *have -= need);
                let next_robots = &mut robots.clone();
                next_robots
                    .entry(robot)
                    .and_modify(|number| *number += 1)
                    .or_insert(1);
                later_cracked = later_cracked.max(geodes_crackable(
                    blueprint,
                    next_robots,
                    next_avails,
                    time - 1,
                ));
            }
        }
    }
    now_cracked + later_cracked
}

fn main() {
    let contents = read_to_string("input").unwrap();
    let mut blueprints: HashMap<u8, Blueprint> = HashMap::new();
    for line in contents.lines() {
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
    }
    let test_blueprint = HashMap::from([
        (OreBot, HashMap::from([(Ore, 4)])),
        (ClayBot, HashMap::from([(Ore, 2)])),
        (ObsBot, HashMap::from([(Ore, 3), (Clay, 14)])),
        (GeodeBot, HashMap::from([(Ore, 2), (Obs, 7)])),
    ]);
    println!(
        "{}",
        geodes_crackable(
            &test_blueprint,
            &HashMap::from([(OreBot, 1)]),
            HashMap::new(),
            24
        )
    )
}
