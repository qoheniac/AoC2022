use std::{
    fs::read_to_string,
    io::{prelude::*, stdout},
    ops::Range,
};

const SENSOR_PREFIX: &str = "Sensor at ";
const BEACON_PREFIX: &str = "closest beacon is at ";
const MAX: i64 = 4000000;

fn strip_next<'a>(split: &'a mut std::str::Split<&str>, prefix: &str) -> &'a str {
    split.next().unwrap().strip_prefix(prefix).unwrap()
}

fn distance(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    (x2 - x1).abs() + (y2 - y1).abs()
}

fn main() {
    let contents = read_to_string("input").unwrap();
    let mut sensors = Vec::new();
    for line in contents.lines() {
        let mut devices = line.split(": ");
        let mut sensor_loc = strip_next(&mut devices, SENSOR_PREFIX).split(", ");
        let sensor_x: i64 = strip_next(&mut sensor_loc, "x=").parse().unwrap();
        let sensor_y: i64 = strip_next(&mut sensor_loc, "y=").parse().unwrap();
        let mut beacon_loc = strip_next(&mut devices, BEACON_PREFIX).split(", ");
        let beacon_x: i64 = strip_next(&mut beacon_loc, "x=").parse().unwrap();
        let beacon_y: i64 = strip_next(&mut beacon_loc, "y=").parse().unwrap();
        let beacon_distance = distance(sensor_x, sensor_y, beacon_x, beacon_y);
        sensors.push([sensor_x, sensor_y, beacon_distance]);
    }

    for row in 0..MAX + 1 {
        if row % (MAX / 100) == 0 {
            print!("\r{:3.0}%", 100 * row / MAX);
            stdout().flush().unwrap();
        }

        let mut beacon_free_ranges: Vec<Range<i64>> = Vec::new();
        for [x, y, d] in &sensors {
            let max = d - (y - row).abs();
            if !max.is_negative() {
                beacon_free_ranges.push(x - max..x + max + 1);
            }
        }

        let mut disjunct_ranges: Vec<Range<i64>> = Vec::new();
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
        if disjunct_ranges.len() == 2 {
            let column = disjunct_ranges[0].end.min(disjunct_ranges[1].end);
            if (0..MAX + 1).contains(&column) {
                println!("\r{}", column * MAX + row);
                break;
            }
        }
    }
}
