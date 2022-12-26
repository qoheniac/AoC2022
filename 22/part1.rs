use std::{
    fs::read_to_string,
    ops::{Add, AddAssign, Mul},
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

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, other: i32) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

#[derive(Clone, PartialEq)]
enum Tile {
    Open,
    Void,
    Wall,
}
use Tile::*;

struct Board {
    width: usize,
    height: usize,
    map: Vec<Vec<Tile>>,
}

impl Board {
    fn get_tile(&self, p: Point) -> &Tile {
        let j = p.x as usize;
        let i = (self.height as i32 - p.y - 1) as usize;
        if i < self.height && j < self.width {
            &self.map[i][j]
        } else {
            &Void
        }
    }
}

enum Direction {
    Left,
    Right,
}
use Direction::*;

enum Instruction {
    Move(u8),
    Turn(Direction),
}
use Instruction::*;

fn main() {
    let contents = read_to_string("input").unwrap();

    // parse map
    let mut parts = contents.split("\n\n");
    let map_string = parts.next().unwrap();
    let width = map_string.lines().map(|line| line.len()).max().unwrap();
    let mut map = Vec::new();
    for line in map_string.lines() {
        let mut row: Vec<Tile> = line
            .chars()
            .map(|c| match c {
                '.' => Open,
                ' ' => Void,
                '#' => Wall,
                _ => panic!(),
            })
            .collect();
        row.resize(width, Void);
        map.push(row);
    }
    let board = Board {
        width: width,
        height: map.len(),
        map: map,
    };

    // parse path
    let description = parts.next().unwrap().trim();
    let mut path = Vec::new();
    let mut next = 0;
    for (index, direction) in description.match_indices(|s| "LR".contains(s)) {
        if next != index {
            path.push(Move((&description[next..index]).parse().unwrap()));
        }
        path.push(Turn(match direction {
            "L" => Left,
            "R" => Right,
            _ => panic!(),
        }));
        next = index + 1;
    }
    if next < description.len() {
        path.push(Move((&description[next..]).parse().unwrap()));
    }
    let path = path;

    // initialize state
    let mut location = Point {
        x: board.map[0].iter().position(|tile| *tile == Open).unwrap() as i32,
        y: (board.height - 1) as i32,
    };
    let mut velocity = Point { x: 1, y: 0 };

    // walk path
    for instruction in path {
        match instruction {
            Turn(direction) => {
                velocity = match direction {
                    Left => Point {
                        x: -velocity.y,
                        y: velocity.x,
                    },
                    Right => Point {
                        x: velocity.y,
                        y: -velocity.x,
                    },
                }
            }
            Move(mut steps) => {
                let mut time = 1;
                while steps > 0 && time != 0 {
                    time = match board.get_tile(location + velocity) {
                        Open => 1,
                        Void => {
                            let mut t = 0;
                            while board.get_tile(location + velocity * (t - 1)) != &Void {
                                t -= 1;
                            }
                            if board.get_tile(location + velocity * t) == &Open {
                                t
                            } else {
                                0
                            }
                        }
                        Wall => 0,
                    };
                    location += velocity * time;
                    steps -= 1;
                }
            }
        };
    }

    // print password
    println!(
        "{}",
        1000 * (board.height as i32 - location.y)
            + 4 * (1 + location.x)
            + if velocity.y == 0 {
                1 - velocity.x
            } else {
                2 + velocity.y
            }
    )
}
