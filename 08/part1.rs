use std::{cell::Cell, fs::read_to_string};

struct Tree {
    height: u32,
    seen: Cell<bool>,
}

fn main() {
    let contents = read_to_string("input").unwrap();
    let forrest: Vec<Vec<Tree>> = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|character| Tree {
                    height: character.to_digit(10).unwrap(),
                    seen: Cell::new(false),
                })
                .collect()
        })
        .collect();
    let column_number = forrest[0].len();

    // look from left
    for row in &forrest {
        let mut height = -1;
        for tree in row {
            if tree.height as i32 > height {
                tree.seen.set(true);
                height = tree.height as i32;
                if height == 9 {
                    break;
                }
            }
        }
    }

    // look from top
    for column in 0..column_number {
        let mut height = -1;
        for tree in forrest.iter().map(|row| &row[column]) {
            if tree.height as i32 > height {
                tree.seen.set(true);
                height = tree.height as i32;
                if height == 9 {
                    break;
                }
            }
        }
    }

    // look from right
    for row in &forrest {
        let mut height = -1;
        for tree in row.iter().rev() {
            if tree.height as i32 > height {
                tree.seen.set(true);
                height = tree.height as i32;
                if height == 9 {
                    break;
                }
            }
        }
    }

    // look from bottom
    for column in 0..column_number {
        let mut height = -1;
        for tree in forrest.iter().rev().map(|row| &row[column]) {
            if tree.height as i32 > height {
                tree.seen.set(true);
                height = tree.height as i32;
                if height == 9 {
                    break;
                }
            }
        }
    }

    // count visible trees
    let mut sum = 0;
    for row in forrest {
        for tree in row {
            if tree.seen.get() {
                sum += 1;
            }
        }
    }
    println!("{}", sum)
}
