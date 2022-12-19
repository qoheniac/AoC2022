use std::{
    fs::read_to_string,
    ops::{Add, AddAssign},
};

#[derive(Clone, Copy, PartialEq)]
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

#[derive(Clone, Copy)]
struct Rock<'a> {
    location: Point,
    shape: &'a Vec<Point>,
}

const FALL: Point = Point { x: 0, y: -1 };

fn is_empty(location: Point, resting: &Vec<Rock>) -> bool {
    location.y > 0
        && (0..7).contains(&location.x)
        && resting.iter().all(|rock| {
            rock.shape
                .iter()
                .all(|rel| rock.location + *rel != location)
        })
}

fn main() {
    let basic_shapes = [
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
    let mut shapes = basic_shapes.iter().cycle();

    let contents = read_to_string("input").unwrap();
    let mut jet = contents.trim().chars().cycle();

    let mut resting = Vec::new();
    let mut highest = 0;
    for _ in 0..2022 {
        let mut falling = Rock {
            location: Point {
                x: 2,
                y: highest + 4,
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
                resting.push(falling);
                highest = highest
                    .max(falling.location.y + falling.shape.iter().map(|rel| rel.y).max().unwrap());
                break;
            }
        }
    }
    println!("{}", highest);
}
