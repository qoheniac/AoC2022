use std::{fs::read_to_string, ops::Range};

const SENSOR_PREFIX: &str = "Sensor at ";
const BEACON_PREFIX: &str = "closest beacon is at ";
const ROW: i32 = 2000000;

fn strip_next<'a>(split: &'a mut std::str::Split<&str>, prefix: &str) -> &'a str {
    split.next().unwrap().strip_prefix(prefix).unwrap()
}

fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x2 - x1).abs() + (y2 - y1).abs()
}

fn main() {
    let contents = read_to_string("input").unwrap();
    let mut sensors = Vec::new();
    for line in contents.lines() {
        let mut devices = line.split(": ");
        let mut sensor_loc = strip_next(&mut devices, SENSOR_PREFIX).split(", ");
        let sensor_x: i32 = strip_next(&mut sensor_loc, "x=").parse().unwrap();
        let sensor_y: i32 = strip_next(&mut sensor_loc, "y=").parse().unwrap();
        let mut beacon_loc = strip_next(&mut devices, BEACON_PREFIX).split(", ");
        let beacon_x: i32 = strip_next(&mut beacon_loc, "x=").parse().unwrap();
        let beacon_y: i32 = strip_next(&mut beacon_loc, "y=").parse().unwrap();
        let beacon_distance = distance(sensor_x, sensor_y, beacon_x, beacon_y);
        sensors.push([sensor_x, sensor_y, beacon_distance]);
    }

    let mut beacon_free_ranges: Vec<Range<i32>> = Vec::new();
    for [x, y, d] in &sensors {
        let max = d - (y - ROW).abs();
        if !max.is_negative() {
            beacon_free_ranges.push(x - max..x + max);
        }
    }

    let mut disjunct_ranges: Vec<Range<i32>> = Vec::new();
    'outer: loop {
        if let Some(range) = beacon_free_ranges.pop() {
            for other in &mut beacon_free_ranges {
                if other.start <= range.end && range.start <= other.end {
                    other.start = other.start.min(range.start);
                    other.end = other.end.max(range.end);
                    continue 'outer;
                }
            }
            disjunct_ranges.push(range);
        } else {
            break;
        }
    }
    println!(
        "{}",
        disjunct_ranges
            .iter()
            .map(|range| range.len())
            .sum::<usize>()
    )
}
