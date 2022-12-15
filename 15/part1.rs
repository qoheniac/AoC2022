use std::{collections::HashSet, fs::read_to_string};

const SENSOR_PREFIX: &str = "Sensor at ";
const BEACON_PREFIX: &str = "closest beacon is at ";

fn strip_next<'a>(split: &'a mut std::str::Split<&str>, prefix: &str) -> &'a str {
    split.next().unwrap().strip_prefix(prefix).unwrap()
}

fn main() {
    let contents = read_to_string("input").unwrap();
    let mut beacons = HashSet::new();
    let mut sensors = HashSet::new();
    for line in contents.lines() {
        let mut devices = line.split(": ");
        let mut sensor_loc = strip_next(&mut devices, SENSOR_PREFIX).split(", ");
        let sensor_x: i32 = strip_next(&mut sensor_loc, "x=").parse().unwrap();
        let sensor_y: i32 = strip_next(&mut sensor_loc, "y=").parse().unwrap();
        let mut beacon_loc = strip_next(&mut devices, BEACON_PREFIX).split(", ");
        let beacon_x: i32 = strip_next(&mut beacon_loc, "x=").parse().unwrap();
        let beacon_y: i32 = strip_next(&mut beacon_loc, "y=").parse().unwrap();
        beacons.insert([beacon_x, beacon_y]);
        sensors.insert([sensor_x, sensor_y, beacon_x, beacon_y]);
    }
}
