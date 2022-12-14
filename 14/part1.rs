use std::fs::read_to_string;

#[derive(Clone, PartialEq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

struct Limits {
    xmin: usize,
    xmax: usize,
    ymin: usize,
    ymax: usize,
}

impl Limits {
    fn new([x, y]: [usize; 2]) -> Self {
        Self {
            xmin: x,
            xmax: x,
            ymin: y,
            ymax: y,
        }
    }
    fn width(&self) -> usize {
        self.xmax - self.xmin + 1
    }
    fn height(&self) -> usize {
        self.ymax - self.ymin + 1
    }
    fn update(&mut self, [x, y]: [usize; 2]) {
        if x < self.xmin {
            self.xmin = x;
        } else if x > self.xmax {
            self.xmax = x;
        }
        if y < self.ymin {
            self.ymin = y;
        } else if y > self.ymax {
            self.ymax = y;
        }
    }
    fn is_outside(&self, [x, y]: [usize; 2]) -> bool {
        x < self.xmin || x > self.xmax || y < self.ymin || y > self.ymax
    }
}

fn main() {
    let contents = read_to_string("input").unwrap();
    let paths: Vec<Vec<[usize; 2]>> = contents
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let mut coordinates = point
                        .split(",")
                        .map(|coordinate| coordinate.parse().unwrap());
                    [coordinates.next().unwrap(), coordinates.next().unwrap()]
                })
                .collect()
        })
        .collect();

    let mut limits = Limits::new(paths[0][0]);
    for path in &paths {
        for point in path {
            limits.update(*point);
        }
    }

    let source = [500, 0];
    limits.update(source);

    let mut cave: Vec<Vec<Tile>> = vec![vec![Tile::Air; limits.width()]; limits.height()];
    for path in &paths {
        let mut points = path.iter();
        let mut start = *points.next().unwrap();
        while let Some(&end) = points.next() {
            if start[0] == end[0] {
                let i = start[0] - limits.xmin;
                let top = start[1].min(end[1]) - limits.ymin;
                let bottom = start[1].max(end[1]) - limits.ymin + 1;
                for j in top..bottom {
                    cave[j][i] = Tile::Rock;
                }
            } else {
                let j = start[1] - limits.ymin;
                let left = start[0].min(end[0]) - limits.xmin;
                let right = start[0].max(end[0]) - limits.xmin + 1;
                for i in left..right {
                    cave[j][i] = Tile::Rock;
                }
            }
            start = end;
        }
    }

    let mut count = 0;
    'producing: loop {
        let [mut x, mut y] = source;
        'falling: loop {
            let [i, j] = [x - limits.xmin, y - limits.ymin];
            if limits.is_outside([x, y + 1]) {
                break 'producing;
            }
            if cave[j + 1][i] != Tile::Air {
                if limits.is_outside([x - 1, y + 1]) {
                    break 'producing;
                } else if cave[j + 1][i - 1] == Tile::Air {
                    x = x - 1;
                } else if limits.is_outside([x + 1, y + 1]) {
                    break 'producing;
                } else if cave[j + 1][i + 1] == Tile::Air {
                    x = x + 1;
                } else {
                    cave[j][i] = Tile::Sand;
                    count += 1;
                    break 'falling;
                }
            }
            y = y + 1;
        }
    }
    println!("{}", count)
}
