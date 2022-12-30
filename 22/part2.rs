use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
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

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Open,
    Wall,
}
use Tile::*;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Edge {
    East,
    North,
    South,
    West,
}
use Edge::*;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Face {
    Back,
    Bottom,
    Front,
    Left,
    Right,
    Top,
}
use Face::*;

const SIZE: usize = 50;
type Grid = [[Tile; SIZE]; SIZE];
type Connections = HashMap<(Face, Edge), (Face, Edge)>;

struct Cube<'a> {
    grids: HashMap<Face, Grid>,
    connections: Connections,
    transformations: HashMap<Face, Box<dyn Fn(Point) -> Point + 'a>>,
}

enum Direction {
    CCW,
    CW,
}
use Direction::*;

enum Instruction {
    Move(u8),
    Turn(Direction),
}
use Instruction::*;

fn connections_from_arr<const N: usize>(arr: [((Face, Edge), (Face, Edge)); N]) -> Connections {
    let mut connections = HashMap::new();
    for ((face1, edge1), (face2, edge2)) in arr.iter() {
        connections.insert((*face1, *edge1), (*face2, *edge2));
        connections.insert((*face2, *edge2), (*face1, *edge1));
    }
    connections
}

fn cw090(p: Point) -> Point {
    let max = SIZE as i32 - 1;
    Point {
        x: max - p.y,
        y: p.x,
    }
}

fn cw180(p: Point) -> Point {
    let max = SIZE as i32 - 1;
    Point {
        x: max - p.x,
        y: max - p.y,
    }
}

fn cw270(p: Point) -> Point {
    let max = SIZE as i32 - 1;
    Point {
        x: p.y,
        y: max - p.x,
    }
}

fn cw360(p: Point) -> Point {
    p
}

fn chain<F, G>(f: F, g: G) -> impl Fn(Point) -> Point
where
    F: Fn(Point) -> Point,
    G: Fn(Point) -> Point,
{
    move |p| f(g(p))
}

fn transformation(previous_edge: Edge, edge: &Edge) -> impl Fn(Point) -> Point {
    match (previous_edge, edge) {
        (East, West) | (North, South) | (South, North) | (West, East) => cw360,
        (East, South) | (North, East) | (South, West) | (West, North) => cw270,
        (East, East) | (North, North) | (South, South) | (West, West) => cw180,
        (East, North) | (North, West) | (South, East) | (West, South) => cw090,
    }
}

fn identify_neighbor(
    cube: &mut Cube,
    face_index_map: &mut HashMap<Face, usize>,
    face_indices: &Vec<[usize; 2]>,
    face: Face,
    edge: Edge,
    idx_max: i32,
) {
    let origin = Point { x: 0, y: 0 };
    let shift = cube.transformations.get(&face).unwrap()(origin);
    let trans = if shift == cw090(origin) {
        cw090
    } else if shift == cw180(origin) {
        cw180
    } else if shift == cw270(origin) {
        cw270
    } else {
        cw360
    };

    let dir = trans(match edge {
        East => Point { x: 1, y: 0 },
        North => Point { x: 0, y: 1 },
        South => Point { x: 0, y: -1 },
        West => Point { x: -1, y: 0 },
    }) - shift;

    let [face_i, face_j] = face_indices[*face_index_map.get(&face).unwrap()];
    if let Ok(neighbor_index) = face_indices.binary_search(&[
        (face_i as i32 - dir.y).rem_euclid(idx_max) as usize,
        (face_j as i32 + dir.x).rem_euclid(idx_max) as usize,
    ]) {
        let (neighbor_face, neighbor_edge) = cube.connections.get(&(face, edge)).unwrap();
        face_index_map
            .entry(*neighbor_face)
            .or_insert(neighbor_index);
        cube.transformations
            .entry(*neighbor_face)
            .or_insert(Box::new(chain(trans, transformation(edge, neighbor_edge))));
    }
}

fn main() {
    // define cube
    let mut cube = Cube {
        grids: HashMap::new(),
        connections: connections_from_arr([
            ((Back, East), (Left, West)),
            ((Back, North), (Top, North)),
            ((Back, South), (Bottom, South)),
            ((Back, West), (Right, East)),
            ((Bottom, East), (Right, South)),
            ((Bottom, North), (Front, South)),
            ((Bottom, West), (Left, South)),
            ((Front, East), (Right, West)),
            ((Front, North), (Top, South)),
            ((Front, West), (Left, East)),
            ((Left, North), (Top, West)),
            ((Right, North), (Top, East)),
        ]),
        transformations: HashMap::from([(Top, Box::new(cw360) as Box<dyn Fn(Point) -> Point>)]),
    };

    // read input
    let contents = read_to_string("input").unwrap();
    let mut parts = contents.split("\n\n");

    // parse map
    let map_string = parts.next().unwrap();
    let width = map_string.lines().map(|line| line.len()).max().unwrap();
    let height = map_string.lines().count();
    let mut face_indices = Vec::new();
    let i_max = height / SIZE;
    let j_max = width / SIZE;
    for i in 0..i_max {
        for j in 0..j_max {
            if let Some(c) = map_string
                .lines()
                .nth(i * SIZE)
                .unwrap()
                .chars()
                .nth(j * SIZE)
            {
                if ".#".contains(c) {
                    face_indices.push([i, j]);
                }
            }
        }
    }

    // initialize identification and identify first face as top
    let mut face_index_map = HashMap::new();
    let mut walls_added_to = HashSet::new();
    face_index_map.insert(Top, 0);

    while walls_added_to.len() < face_indices.len() {
        // pick an identified face
        let (&face, &index) = face_index_map
            .iter()
            .filter(|(f, _)| !walls_added_to.contains(*f))
            .nth(0)
            .unwrap();

        // add walls
        let mut grid = [[Open; SIZE]; SIZE];
        let [face_i, face_j] = face_indices[index];
        for (i, line) in map_string
            .lines()
            .skip(face_i * SIZE)
            .take(SIZE)
            .enumerate()
        {
            for (j, c) in line.chars().skip(face_j * SIZE).take(SIZE).enumerate() {
                if c == '#' {
                    grid[i][j] = Wall;
                }
            }
        }
        cube.grids.insert(face, grid);
        walls_added_to.insert(face);

        // identify neighbors
        for edge in [East, North, South, West] {
            identify_neighbor(
                &mut cube,
                &mut face_index_map,
                &face_indices,
                face,
                edge,
                i_max.max(j_max) as i32,
            );
        }
    }

    // parse path
    let description = parts.next().unwrap().trim();
    let mut path = Vec::new();
    let mut next = 0;
    for (index, direction) in description.match_indices(|s| "LR".contains(s)) {
        if next != index {
            path.push(Move((&description[next..index]).parse().unwrap()));
        }
        path.push(Turn(match direction {
            "L" => CCW,
            "R" => CW,
            _ => panic!(),
        }));
        next = index + 1;
    }
    if next < description.len() {
        path.push(Move((&description[next..]).parse().unwrap()));
    }
    let path = path;

    // initialize state
    let mut face = Top;
    let mut location = Point {
        x: cube.grids.get_mut(&face).unwrap()[0]
            .iter()
            .position(|tile| *tile == Open)
            .unwrap() as i32,
        y: (SIZE - 1) as i32,
    };
    let mut velocity = Point { x: 1, y: 0 };
    let max = SIZE as i32 - 1;

    // walk path
    for instruction in path {
        match instruction {
            Turn(direction) => {
                velocity = match direction {
                    CCW => Point {
                        x: -velocity.y,
                        y: velocity.x,
                    },
                    CW => Point {
                        x: velocity.y,
                        y: -velocity.x,
                    },
                }
            }
            Move(steps) => {
                for _ in 0..steps {
                    let (old_f, old_l, old_v) = (face, location, velocity);
                    location = location + velocity;
                    let mut edge_option = None;
                    let edge;

                    // get entered face and edge of out of bounds
                    if location.x < 0 {
                        (face, edge) = *cube.connections.get(&(face, West)).unwrap();
                        edge_option = Some(edge);
                    } else if location.x > max {
                        (face, edge) = *cube.connections.get(&(face, East)).unwrap();
                        edge_option = Some(edge);
                    } else if location.y > max {
                        (face, edge) = *cube.connections.get(&(face, North)).unwrap();
                        edge_option = Some(edge);
                    } else if location.y < 0 {
                        (face, edge) = *cube.connections.get(&(face, South)).unwrap();
                        edge_option = Some(edge);
                    }

                    // if out of bounds change state based on entrance edge
                    if let Some(edge) = edge_option {
                        let mut d1 = (old_l.x - old_l.y).abs();
                        let mut d2 = (max - old_l.x - old_l.y).abs();
                        if velocity.y == 0 {
                            (d1, d2) = (d2, d1);
                        }
                        location = match edge {
                            East => Point { x: max, y: d2 },
                            North => Point { x: d1, y: max },
                            South => Point { x: d2, y: 0 },
                            West => Point { x: 0, y: d1 },
                        };
                        velocity = match edge {
                            East => Point { x: -1, y: 0 },
                            North => Point { x: 0, y: -1 },
                            South => Point { x: 0, y: 1 },
                            West => Point { x: 1, y: 0 },
                        }
                    }

                    // restore previous state if tile is a wall and stop movement
                    let transformed = cube.transformations.get(&face).unwrap()(location);
                    if cube.grids.get(&face).unwrap()
                        [(max - transformed.y.rem_euclid(SIZE as i32)) as usize]
                        [transformed.x.rem_euclid(SIZE as i32) as usize]
                        == Wall
                    {
                        (face, location, velocity) = (old_f, old_l, old_v);
                        break;
                    }
                }
            }
        }
    }

    // print password
    location = cube.transformations.get(&face).unwrap()(location);
    location.x = location.x.rem_euclid(SIZE as i32);
    location.y = location.y.rem_euclid(SIZE as i32);
    velocity = cube.transformations.get(&face).unwrap()(velocity);
    velocity -= cube.transformations.get(&face).unwrap()(Point { x: 0, y: 0 });
    let [i, j] = face_indices[*face_index_map.get(&face).unwrap()];
    println!(
        "{}",
        1000 * (SIZE as i32 - location.y + (i * SIZE) as i32)
            + 4 * (1 + location.x + (j * SIZE) as i32)
            + if velocity.y == 0 {
                1 - velocity.x
            } else {
                2 + velocity.y
            }
    )
}
