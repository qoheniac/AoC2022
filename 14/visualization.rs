use std::{io::{prelude::*, stdout},fs::{File, read_to_string}};
use image::{Frame, RgbaImage, Rgba, codecs::gif::GifEncoder};

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
}

fn make_frame(cave: &Vec<Vec<Tile>>) -> Frame {
    let width = cave[0].len();
    let height = cave.len();
    let mut img = RgbaImage::new(width as u32, height as u32);
    for j in 0..height {
        for i in 0..width {
            img.put_pixel(i as u32, j as u32, match cave[j][i] {
                Tile::Air => Rgba([242,233,234,255]),
                Tile::Rock => Rgba([183,176,156,255]),
                Tile::Sand => Rgba([133,103,28,255]),
            });
        }
    }
    Frame::new(img)
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
    limits.ymax = limits.ymax + 2;
    limits.xmin = limits.xmin - limits.height();
    limits.xmax = limits.xmax + limits.height();

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
    for i in 0..limits.width() {
        cave[limits.height() - 1][i] = Tile::Rock;
    }

    let output_file = File::create("visualization.gif").unwrap();
    let mut encoder = GifEncoder::new(output_file);
    encoder.encode_frame(make_frame(&cave)).unwrap();

    let mut count = 0;
    'producing: loop {
        let [mut x, mut y] = source;
        loop {
            let [i, j] = [x - limits.xmin, y - limits.ymin];
            if cave[j + 1][i] != Tile::Air {
                if cave[j + 1][i - 1] == Tile::Air {
                    x = x - 1;
                } else if cave[j + 1][i + 1] == Tile::Air {
                    x = x + 1;
                } else {
                    cave[j][i] = Tile::Sand;
                    count += 1;
                    if [x, y] == source {
                        break 'producing;
                    }
                    if count % 1000 == 0 {
                    print!("\r{:3.0}%", 100 * count / 23921);
                    stdout().flush().unwrap();
                    encoder.encode_frame(make_frame(&cave)).unwrap();
                    }
                    break;
                }
            }
            y = y + 1;
        }
    }
    print!("\r    \r")
}
