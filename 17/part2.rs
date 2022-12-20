use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
    io::{prelude::*, stdout},
    ops::{Add, AddAssign},
};

// auxiliary struct for adding 2D vectors
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

// constants
const FALL: Point = Point { x: 0, y: -1 };
const FINAL_COUNT: usize = 1000000000000;
const WIN_WIDTH: usize = 5; // width of windows used for repetition detection

// rock struct holding its absolute location and relative coordinates defining its shape
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Rock<'a> {
    location: Point,
    shape: &'a Vec<Point>,
}

// auxiliary function to check if a location is occupied by a rock
fn is_empty(location: Point, resting: &HashSet<Rock>) -> bool {
    location.y > 0
        && (0..7).contains(&location.x)
        && resting.iter().all(|rock| {
            rock.shape
                .iter()
                .all(|rel| rock.location + *rel != location)
        })
}

fn main() {
    let basic_shapes = vec![
        vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 3, y: 0 }, // ####
        ],
        vec![
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 }, // .#.
            Point { x: 2, y: 1 }, // ###
            Point { x: 1, y: 2 }, // .#.
        ],
        vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 }, // ..#
            Point { x: 2, y: 1 }, // ..#
            Point { x: 2, y: 2 }, // ###
        ],
        vec![
            Point { x: 0, y: 0 }, // #
            Point { x: 0, y: 1 }, // #
            Point { x: 0, y: 2 }, // #
            Point { x: 0, y: 3 }, // #
        ],
        vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 }, // ##
            Point { x: 1, y: 1 }, // ##
        ],
    ];
    let mut shapes = basic_shapes.iter().cycle();

    // read jet directions
    let contents = read_to_string("input").unwrap();
    let mut jet = contents.trim().chars().cycle();

    // least common multiple used as minimum repetition scale
    let lcm = contents.trim().len() * basic_shapes.len();

    // initialization
    let mut highest = [0; 7]; // per-colon maximum height occupied
    let mut resting = HashSet::new(); // resting rocks
    let mut heights = VecDeque::new(); // remember highest after each lcm cycle
    let mut count = 0; // spawned rocks
    let mut max_count = None; // stop spawning at this count if set
    let mut cycles = 0; // repetition length in units of lcm cycles

    // repeatedly spawn lcm new rocks and output progress
    'main_loop: loop {
        print!("\r{}", count / lcm);
        stdout().flush().unwrap();

        // spawn new rocks
        for _ in 0..lcm {
            count += 1;
            let mut falling = Rock {
                location: Point {
                    x: 2,
                    y: highest.iter().max().unwrap() + 4,
                },
                shape: shapes.next().unwrap(),
            };

            // move rock
            loop {
                // jet pushes left or right if possible
                let push = Point {
                    x: match jet.next() {
                        Some('>') => 1,
                        _ => -1,
                    },
                    y: 0,
                };
                if falling
                    .shape
                    .iter()
                    .all(|rel| is_empty(falling.location + *rel + push, &resting))
                {
                    falling.location += push;
                }

                // rock falls or comes to rest possibly changing the values of highest
                if falling
                    .shape
                    .iter()
                    .all(|rel| is_empty(falling.location + *rel + FALL, &resting))
                {
                    falling.location += FALL;
                } else {
                    resting.insert(falling);
                    for rel in falling.shape {
                        let point = falling.location + *rel;
                        highest[point.x as usize] = highest[point.x as usize].max(point.y);
                    }
                    break;
                }
            }

            // remove unreachable rocks
            let rocks: Vec<Rock> = resting.iter().cloned().collect();
            for rock in rocks {
                if rock.location.y + rock.shape.iter().map(|rel| rel.y).max().unwrap()
                    < *highest.iter().min().unwrap()
                {
                    resting.remove(&rock);
                }
            }

            // if max_count was set and reached break
            if let Some(c) = max_count {
                if count == c {
                    break 'main_loop;
                }
            }
        }

        // remember maximum height and search old heights for a repeating pattern
        heights.push_front(*highest.iter().max().unwrap());
        let mut windows = heights.as_slices().0.windows(WIN_WIDTH).map(|win| {
            win.windows(2)
                .map(|pair| pair[0] - pair[1])
                .collect::<Vec<i64>>()
        }); // fixed-width windows holding 	successive height changes
        if let Some(new_window) = windows.next() {
            for (i, ref_window) in windows.enumerate() {
                if new_window == ref_window {
                    cycles = i + 1;

                    // if rocks to be spwaned are no multiple of the repetition length, spawn more
                    let remaining = (FINAL_COUNT - count) % (cycles * lcm);
                    if remaining == 0 {
                        break 'main_loop;
                    }
                    max_count = Some(count + remaining);
                    break;
                }
            }
        }
    }

    // extrapolate and output maximum height using repetition length
    println!(
        "\r{}",
        *highest.iter().max().unwrap()
            + (heights[0] - heights[cycles]) * ((FINAL_COUNT - count) / (cycles * lcm)) as i64
    )
}
