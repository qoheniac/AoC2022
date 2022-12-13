use std::{cmp::Ordering, fs::read_to_string};

#[derive(PartialEq)]
enum Packet {
    Integer(u32),
    List(Vec<Self>),
}

impl Packet {
    fn new(packet_string: &str) -> Self {
        let mut chars = packet_string.chars();
        if chars.next().unwrap() == '[' {
            if chars.next().unwrap() == ']' {
                Self::List(Vec::new())
            } else {
                let length = packet_string.len();
                let list_body = &packet_string[1..length - 1];
                let mut list = Vec::new();
                let mut depth = 0;
                let mut left = 0;
                for (index, character) in list_body.chars().enumerate() {
                    match character {
                        '[' => depth += 1,
                        ']' => depth -= 1,
                        ',' => {
                            if depth == 0 {
                                list.push(Self::new(&list_body[left..index]));
                                left = index + 1;
                            }
                        }
                        _ => (),
                    }
                }
                list.push(Self::new(&list_body[left..]));
                Self::List(list)
            }
        } else {
            Self::Integer(packet_string.parse().unwrap())
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Integer(self_num), Self::Integer(other_num)) => self_num.partial_cmp(other_num),
            (Self::Integer(self_num), Self::List(_)) => {
                Self::List(vec![Self::Integer(*self_num)]).partial_cmp(other)
            }
            (Self::List(_), Self::Integer(other_num)) => {
                self.partial_cmp(&Self::List(vec![Self::Integer(*other_num)]))
            }
            (Self::List(self_list), Self::List(other_list)) => self_list.partial_cmp(other_list),
        }
    }
}

fn main() {
    let contents = read_to_string("input").unwrap();
    println!(
        "{}",
        contents
            .split("\n\n")
            .enumerate()
            .map(|(index, pair)| {
                let mut packets = pair.lines();
                let packet1 = Packet::new(packets.next().unwrap());
                let packet2 = Packet::new(packets.next().unwrap());
                (packet1 < packet2) as usize * (index + 1)
            })
            .sum::<usize>()
    )
}
