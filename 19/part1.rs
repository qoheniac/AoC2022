use std::{
    collections::HashMap,
    fs::read_to_string,
    io::{prelude::*, stdout},
};

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

// TODO: change this such that it chooses a robot to build next, see if that is possible in the
// remaining time by extrapolating current production, and then go directly to that state in the
// future, skipping intermediate steps
fn geodes_crackable(blueprint: &Blueprint, robots: Robots, avails: Resources, time: u8) -> u8 {
    let mut now_cracked = 0;
    if time == 0 {
        return now_cracked;
    }
    let mut avails_plus_production = avails.clone();
    for (&robot, &number) in &robots {
        if let Some(material) = robot.crack() {
            avails_plus_production
                .entry(material)
                .and_modify(|have| *have += number)
                .or_insert(number);
        } else {
            now_cracked += number;
        }
    }
    let mut later_cracked = 0;
    for robot in [GeodeBot, ObsBot, ClayBot, OreBot] {
        let costs = blueprint.get(&robot).unwrap();
        if costs
            .iter()
            .all(|(material, need)| matches!(avails.get(material), Some(have) if have >= need))
        {
            let mut next_avails = avails_plus_production.clone();
            for (&material, need) in costs {
                next_avails.entry(material).and_modify(|have| *have -= need);
            }
            let mut next_robots = robots.clone();
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
            if [GeodeBot, ObsBot].contains(&robot) {
                return now_cracked + later_cracked;
            }
        }
    }
    later_cracked = later_cracked.max(geodes_crackable(
        blueprint,
        robots,
        avails_plus_production,
        time - 1,
    ));
    now_cracked + later_cracked
}

fn main() {
    let mut result = 0;
    let contents = read_to_string("input").unwrap();
    let len = contents.lines().collect::<Vec<&str>>().len();
    for (i, line) in contents.lines().enumerate() {
        print!("\r{:3}%", 100 * i / len);
        stdout().flush().unwrap();
        let vals: Vec<u8> = line
            .split(|c: char| !c.is_numeric())
            .filter_map(|s| s.parse().ok())
            .collect();
        result += vals[0]
            * geodes_crackable(
                &HashMap::from([
                    (OreBot, HashMap::from([(Ore, vals[1])])),
                    (ClayBot, HashMap::from([(Ore, vals[2])])),
                    (ObsBot, HashMap::from([(Ore, vals[3]), (Clay, vals[4])])),
                    (GeodeBot, HashMap::from([(Ore, vals[5]), (Obs, vals[6])])),
                ]),
                HashMap::from([(OreBot, 1)]),
                HashMap::new(),
                24,
            );
    }
    println!("\r    \r{}", result)
}
