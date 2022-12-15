use std::{collections::HashSet, fs::read_to_string};

const SENSOR_PREFIX: &str = "Sensor at";
const BEACON_PREFIX: &str = "closest beacon is at ";

fn main() {
    let contents = read_to_string("input").unwrap();
    let mut beacons = HashSet::new();
    let mut sensors = HashSet::new();
    for line in contents.lines() {
        let mut devices = line.split(": ");
        let mut sensor_loc = devices
            .next()
            .unwrap()
            .strip_prefix(SENSOR_PREFIX)
            .unwrap()
            .split(", ");
        let sensor_x: usize = sensor_loc
            .next()
            .unwrap()
            .strip_prefix("x=")
            .unwrap()
            .parse()
            .unwrap();
        let sensor_y: usize = sensor_loc
            .next()
            .unwrap()
            .strip_prefix("y=")
            .unwrap()
            .parse()
            .unwrap();
        let mut beacon_loc = devices
            .next()
            .unwrap()
            .strip_prefix(BEACON_PREFIX)
            .unwrap()
            .split(", ");
        let beacon_x: usize = beacon_loc
            .next()
            .unwrap()
            .strip_prefix("x=")
            .unwrap()
            .parse()
            .unwrap();
        let beacon_y: usize = beacon_loc
            .next()
            .unwrap()
            .strip_prefix("y=")
            .unwrap()
            .parse()
            .unwrap();
        beacons.insert([beacon_x, beacon_y]);
        sensors.insert([sensor_x, sensor_y, beacon_x, beacon_y]);
    }
}
