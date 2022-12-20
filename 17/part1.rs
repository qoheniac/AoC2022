use std::{
    fs::read_to_string,
    ops::{Add, AddAssign},
};

// auxiliary struct for adding 2D vectors
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

const FALL: Point = Point { x: 0, y: -1 };

// rock struct holding its absolute location and relative coordinates defining its shape
#[derive(Clone, Copy)]
struct Rock<'a> {
    location: Point,
    shape: &'a Vec<Point>,
}

// auxiliary function to check if a location is occupied by a rock
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

    // initialize maximum height occupied and vector of resting rocks
    let mut highest = 0;
    let mut resting = Vec::new();

    // spawn new rocks
    for _ in 0..2022 {
        let mut falling = Rock {
            location: Point {
                x: 2,
                y: highest + 4,
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

            // rock falls or comes to rest possibly changing the value of highest
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
