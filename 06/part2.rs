use std::{collections::VecDeque, fs::read_to_string};

fn main() {
    let contents = read_to_string("input").unwrap();
    let mut characters = contents.chars();
    let mut last_fourteen = VecDeque::new();
    for _ in 0..14 {
        last_fourteen.push_back(characters.next().unwrap());
    }
    let mut count = 14;
    loop {
        let mut is_start_of_packet = true;
        'sop_detection: for i in 0..14 {
            for j in (i + 1)..14 {
                if last_fourteen[i] == last_fourteen[j] {
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
            last_fourteen.pop_front();
            last_fourteen.push_back(c);
        } else {
            break;
        }
    }
    println!("{}", count)
}
