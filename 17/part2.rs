use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
    io::{prelude::*, stdout},
    ops::{Add, AddAssign},
};

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

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Rock<'a> {
    location: Point,
    shape: &'a Vec<Point>,
}

const FALL: Point = Point { x: 0, y: -1 };
const WIN_WIDTH: usize = 5;
const FINAL_COUNT: usize = 1000000000000;

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
            Point { x: 3, y: 0 },
        ],
        vec![
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 1, y: 2 },
        ],
        vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 2, y: 1 },
            Point { x: 2, y: 2 },
        ],
        vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 0, y: 3 },
        ],
        vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
        ],
    ];
    let contents = read_to_string("input").unwrap();
    let lcm = contents.trim().len() * basic_shapes.len();
    let mut shapes = basic_shapes.iter().cycle();
    let mut jet = contents.trim().chars().cycle();
    let mut resting = HashSet::new();
    let mut highest = [0; 7];
    let mut heights = VecDeque::new();
    let mut count = 0;
    let mut max_count = None;
    let mut cycles = 0;
    'main_loop: loop {
        print!("\r{}", count / lcm);
        stdout().flush().unwrap();
        loop {
            count += 1;
            let mut falling = Rock {
                location: Point {
                    x: 2,
                    y: highest.iter().max().unwrap() + 4,
                },
                shape: shapes.next().unwrap(),
            };
            loop {
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
            let rocks: Vec<Rock> = resting.iter().cloned().collect();
            for rock in rocks {
                if rock.location.y + rock.shape.iter().map(|rel| rel.y).max().unwrap()
                    < *highest.iter().min().unwrap()
                {
                    resting.remove(&rock);
                }
            }

            if let Some(c) = max_count {
                if count == c {
                    break 'main_loop;
                }
            }

            if count % lcm == 0 {
                break;
            }
        }
        heights.push_front(*highest.iter().max().unwrap());
        let mut windows = heights.as_slices().0.windows(WIN_WIDTH).map(|win| {
            win.windows(2)
                .map(|pair| pair[0] - pair[1])
                .collect::<Vec<i64>>()
        });
        if let Some(new_window) = windows.next() {
            for (i, ref_window) in windows.enumerate() {
                if new_window == ref_window {
                    cycles = i + 1;
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
    println!(
        "\r{}",
        *highest.iter().max().unwrap()
            + (heights[0] - heights[cycles]) * ((FINAL_COUNT - count) / (cycles * lcm)) as i32
    )
}
