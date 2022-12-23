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

fn geodes_crackable(blueprint: &Blueprint, robots: Robots, avails: Resources, time: u8) -> u8 {
    if time == 0 {
        return 0;
    }
    let mut production = HashMap::new();
    let mut geode_rate = 0;
    for (&robot, &number) in &robots {
        if let Some(material) = robot.crack() {
            production.insert(material, number);
        } else {
            geode_rate = number;
        }
    }

    let mut future_geodes = 0;
    let mut time_needed_geode = time + 1;
    'robots: for robot in [GeodeBot, ObsBot, ClayBot, OreBot] {
        if let Some(material) = robot.crack() {
            if let Some(number) = robots.get(&robot) {
                if blueprint
                    .values()
                    .all(|costs| !matches!(costs.get(&material), Some(need) if number<need))
                {
                    continue;
                }
            }
        }
        let costs = blueprint.get(&robot).unwrap();
        let mut time_needed = 1;
        for (material, &need) in costs {
            let mut have = 0;
            if let Some(amount) = avails.get(material) {
                have += amount;
            }
            if have < need {
                if let Some(&rate) = production.get(material) {
                    time_needed = time_needed.max(1 + (need - have + rate - 1) / rate);
                } else {
                    time_needed = 1 + time;
                }
                if time_needed >= time {
                    future_geodes = future_geodes.max(geode_rate * time);
                    continue 'robots;
                }
            }
        }
        if robot == GeodeBot {
            time_needed_geode = time_needed;
        } else if time_needed >= time_needed_geode {
            continue;
        }
        let mut future_robots = robots.clone();
        future_robots
            .entry(robot)
            .and_modify(|number| *number += 1)
            .or_insert(1);
        let mut future_avails = avails.clone();
        for (&material, rate) in &production {
            let gain = rate * time_needed;
            future_avails
                .entry(material)
                .and_modify(|have| *have += gain)
                .or_insert(gain);
        }
        for (&material, need) in costs {
            future_avails
                .entry(material)
                .and_modify(|have| *have -= need);
        }
        let later_future_geodes =
            geodes_crackable(blueprint, future_robots, future_avails, time - time_needed);
        future_geodes = future_geodes.max(geode_rate * time_needed + later_future_geodes);
    }
    future_geodes
}

fn main() {
    let mut result = 1;
    let contents = read_to_string("input").unwrap();
    for (i, line) in contents.lines().enumerate().take(3) {
        print!("\r{:3}% ({})", 100 * i / 3, result);
        stdout().flush().unwrap();
        let vals: Vec<u8> = line
            .split(|c: char| !c.is_numeric())
            .filter_map(|s| s.parse().ok())
            .collect();
        result *= geodes_crackable(
            &HashMap::from([
                (OreBot, HashMap::from([(Ore, vals[1])])),
                (ClayBot, HashMap::from([(Ore, vals[2])])),
                (ObsBot, HashMap::from([(Ore, vals[3]), (Clay, vals[4])])),
                (GeodeBot, HashMap::from([(Ore, vals[5]), (Obs, vals[6])])),
            ]),
            HashMap::from([(OreBot, 1)]),
            HashMap::new(),
            32,
        ) as u16;
    }
    println!("\x1B[2K\r{}", result)
}
