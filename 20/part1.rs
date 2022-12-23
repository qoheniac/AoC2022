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
    let contents = read_to_string("input").unwrap();
    let mut file: Vec<Entry> = contents
        .lines()
        .map(|line| Entry::new(line.parse().unwrap()))
        .collect();
    // let mut file: Vec<Entry> = [1, 2, -3, 3, -2, 0, 4]
    //     .iter()
    //     .map(|value| Entry::new(*value))
    //     .collect();
    let length = file.len() as i16;
    for _ in 0..length {
        let (old_index, value) = file
            .iter()
            .enumerate()
            .find_map(|(index, entry)| (!entry.was_moved).then_some((index, entry.value)))
            .unwrap();
        let mut new_index_unwrapped = old_index as i16 + value;
        if value.is_negative() && !new_index_unwrapped.is_positive() {
            new_index_unwrapped -= 1;
        } else if new_index_unwrapped >= length {
            new_index_unwrapped += 1;
        }
        let new_index = (new_index_unwrapped).rem_euclid(length) as usize;
        if new_index > old_index {
            file[old_index..new_index + 1].rotate_left(1);
        } else {
            file[new_index..old_index + 1].rotate_right(1);
        }
        file[new_index].was_moved = true;
    }
    let i0 = file
        .iter()
        .enumerate()
        .find_map(|(index, entry)| (entry.value == 0).then_some(index))
        .unwrap();
    let mut sum = 0;
    for k in 1..4 {
        sum += file[((i0 + 1000 * k) % length as usize)].value;
    }
    println!("{}", sum)
}
