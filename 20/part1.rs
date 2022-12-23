use std::fs::read_to_string;

struct Entry {
    value: i16,
    was_moved: bool,
}

impl Entry {
    fn new(value: i16) -> Self {
        Self {
            value: value,
            was_moved: false,
        }
    }
}

fn main() {
    // read input
    let contents = read_to_string("input").unwrap();
    let mut file: Vec<Entry> = contents
        .lines()
        .map(|line| Entry::new(line.parse().unwrap()))
        .collect();
    let length = file.len() as i16;

    // mixing
    for _ in 0..length {
        let (old_index, value) = file
            .iter()
            .enumerate()
            .find_map(|(index, entry)| (!entry.was_moved).then_some((index, entry.value)))
            .unwrap();
        let new_index = (old_index as i16 + value).rem_euclid(length - 1) as usize;
        if new_index > old_index {
            file[old_index..new_index + 1].rotate_left(1);
        } else {
            file[new_index..old_index + 1].rotate_right(1);
        }
        file[new_index].was_moved = true;
    }

    // find zero
    let i0 = file
        .iter()
        .enumerate()
        .find_map(|(index, entry)| (entry.value == 0).then_some(index))
        .unwrap();

    // calculate result
    let mut sum = 0;
    for k in 1..4 {
        sum += file[((i0 + 1000 * k) % length as usize)].value;
    }
    println!("{}", sum)
}
