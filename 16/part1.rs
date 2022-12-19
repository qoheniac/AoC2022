use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    iter::once,
};

struct Valve<'a> {
    flow_rate: u32,
    tunnels: Vec<&'a str>,
}

// recursively calculate maximum pressure that can be released in some time starting at some valve
fn best_pressure(
    loc_key: &str,
    remaining: u32,
    open_valve_keys: HashSet<&str>,
    valves: &HashMap<&str, Valve>,
    useful_valves: &HashSet<&str>,
    distances: &HashMap<&str, HashMap<&str, u32>>,
) -> u32 {
    let mut result = 0;
    for dest_key in useful_valves.iter() {
        if dest_key == &loc_key || open_valve_keys.contains(dest_key) {
            continue;
        }
        let distance = *distances.get(loc_key).unwrap().get(dest_key).unwrap();
        if distance < remaining {
            let next_remaining = remaining - distance - 1;
            let mut next_open_valve_keys = open_valve_keys.clone();
            next_open_valve_keys.insert(dest_key);
            let mut dest_pressure = valves.get(dest_key).unwrap().flow_rate * next_remaining;
            if next_remaining > 2 {
                dest_pressure += best_pressure(
                    dest_key,
                    next_remaining,
                    next_open_valve_keys,
                    valves,
                    useful_valves,
                    distances,
                );
            }
            result = result.max(dest_pressure);
        }
    }
    result
}

fn main() {
    // parse input file
    let contents = read_to_string("input").unwrap();
    let mut valves = HashMap::new();
    for line in contents.lines() {
        let mut parts = line.split("; ");
        let mut valve_parts = parts.next().unwrap().split("=");
        let valve_key = valve_parts.next().unwrap().split(" ").nth(1).unwrap();
        let flow_rate = valve_parts.next().unwrap().parse().unwrap();
        let test = parts.next().unwrap();
        let tunnels = test
            .strip_prefix("tunnels lead to valves ")
            .unwrap_or_else(|| test.strip_prefix("tunnel leads to valve ").unwrap())
            .split(", ")
            .collect();
        valves.insert(valve_key, Valve { flow_rate, tunnels });
    }

    // find valves able to release pressure
    let useful_valves: HashSet<&str> = valves
        .iter()
        .filter_map(|(k, v)| if v.flow_rate > 0 { Some(*k) } else { None })
        .collect();

    // pre-calculate distances between useful valves
    let mut distances = HashMap::new();
    for loc_key in useful_valves.iter().chain(once(&"AA")) {
        let mut distances_from_loc = HashMap::new();
        for dest_key in &useful_valves {
            let mut known_distances = HashMap::from([(loc_key, 0)]);
            let mut to_visit = HashSet::new();
            to_visit.insert(loc_key);
            'dijkstra: loop {
                let visit_key = to_visit
                    .iter()
                    .min_by_key(|&k| known_distances.get(k).unwrap())
                    .unwrap()
                    .clone();
                to_visit.remove(visit_key);
                let visit_dist = known_distances.get(visit_key).unwrap().clone();
                for neighbor_key in &valves.get(visit_key).unwrap().tunnels {
                    if let Some(dist) = known_distances.get(neighbor_key) {
                        if visit_dist + 1 < *dist {
                            known_distances.insert(neighbor_key, visit_dist + 1);
                        }
                    } else {
                        known_distances.insert(neighbor_key, visit_dist + 1);
                        to_visit.insert(neighbor_key);
                    }
                    if neighbor_key == dest_key {
                        break 'dijkstra;
                    }
                }
            }
            let distance = known_distances.get(dest_key).unwrap();
            distances_from_loc.insert(*dest_key, *distance);
        }
        distances.insert(*loc_key, distances_from_loc);
    }

    // calculate and output maximum pressure that can be released in 30 min
    println!(
        "{}",
        best_pressure(
            "AA",
            30,
            HashSet::new(),
            &valves,
            &useful_valves,
            &distances
        )
    )
}
