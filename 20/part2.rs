use std::fs::read_to_string;

struct Entry {
    id: usize,
    value: i64,
    was_moved: bool,
}

impl Entry {
    fn new(id: usize, value: i64) -> Self {
        Self {
            id: id,
            value: value,
            was_moved: false,
        }
    }
}

fn main() {
    let contents = read_to_string("input").unwrap();
    let mut file: Vec<Entry> = contents
        .lines()
        .enumerate()
        .map(|(id, line)| Entry::new(id, 811589153 * line.parse::<i64>().unwrap()))
        .collect();
    let length = file.len();
    for _ in 0..10 {
        for id in 0..length {
            let (old_index, value) = file
                .iter()
                .enumerate()
                .find_map(|(index, entry)| (entry.id == id).then_some((index, entry.value)))
                .unwrap();
            let new_index = (old_index as i64 + value).rem_euclid(length as i64 - 1) as usize;
            if new_index > old_index {
                file[old_index..new_index + 1].rotate_left(1);
            } else {
                file[new_index..old_index + 1].rotate_right(1);
            }
            file[new_index].was_moved = true;
        }
    }
    let i0 = file
        .iter()
        .enumerate()
        .find_map(|(index, entry)| (entry.value == 0).then_some(index))
        .unwrap();
    let mut sum = 0;
    for k in 1..4 {
        sum += file[(i0 + 1000 * k) % length].value;
    }
    println!("{}", sum)
}
