use std::{collections::VecDeque, fs::read_to_string};

fn main() {
    let contents = read_to_string("input").unwrap();
    let mut characters = contents.chars();
    let mut last_four = VecDeque::new();
    for _ in 0..4 {
        last_four.push_back(characters.next().unwrap());
    }
    let mut count = 4;
    loop {
        let mut is_start_of_packet = true;
        'sop_detection: for i in 0..3 {
            for j in i + 1..4 {
                if last_four[i] == last_four[j] {
                    is_start_of_packet = false;
                    count += 1;
                    break 'sop_detection;
                }
            }
        }
        if is_start_of_packet {
            break;
        }
        if let Some(c) = characters.next() {
            last_four.pop_front();
            last_four.push_back(c);
        } else {
            break;
        }
    }
    println!("{}", count)
}
