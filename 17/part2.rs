use std::{
    collections::HashSet,
    fs::read_to_string,
    ops::{Add, AddAssign},
};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
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
const N: u64 = 1000000000000;

fn is_empty(location: Point, resting: &HashSet<Rock>) -> bool {
    location.y > 0
        && (0..7).contains(&location.x)
        && resting.iter().all(|rock| {
            rock.shape
                .iter()
                .all(|rel| rock.location + *rel != location)
        })
}

fn height(n: i32, contents: &str, basic_shapes: &Vec<Vec<Point>>) -> i32 {
    let mut shapes = basic_shapes.iter().cycle();
    let mut jet = contents.trim().chars().cycle();
    let mut resting = HashSet::new();
    let mut highest = [0; 7];
    for _ in 0..n {
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
            if rock.location.y + rock.shape.iter().map(|rel| rel.y).max().unwrap() < *highest.iter().min().unwrap() {
                resting.remove(&rock);
            }
        }
    }
    *highest.iter().max().unwrap()
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
    let min_length = (contents.trim().len() * basic_shapes.len()) as i32;
    let mut length = min_length;
    loop {
        let height1 = height(length, &contents, &basic_shapes);
        println!("{}", height1 * 2);
        let height2 = height(length * 2, &contents, &basic_shapes);
        println!("{}", height2);
        if height2 == height1 * 2 {
            println!("repitition!"); // happens at height2 = 4264562 (after 27 steps)
            break;
        }
        length += min_length;
    }
}

