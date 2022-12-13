use std::{cmp::Ordering, fs::read_to_string};

#[derive(PartialEq)]
enum PacketData {
    Integer(u32),
    List(Vec<PacketData>),
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            PacketData::Integer(self_num) => match other {
                PacketData::Integer(other_num) => self_num.partial_cmp(other_num),
                PacketData::List(_) => {
                    PacketData::List(vec![PacketData::Integer(*self_num)]).partial_cmp(other)
                }
            },
            PacketData::List(self_list) => match other {
                PacketData::Integer(other_num) => {
                    self.partial_cmp(&PacketData::List(vec![PacketData::Integer(*other_num)]))
                }
                PacketData::List(other_list) => {
                    let mut self_elements = self_list.iter();
                    let mut other_elements = other_list.iter();
                    loop {
                        let self_next = self_elements.next();
                        let other_next = other_elements.next();
                        match self_next {
                            Some(self_element) => match other_next {
                                Some(other_element) => {
                                    match self_element.partial_cmp(other_element) {
                                        Some(Ordering::Equal) => (),
                                        other_option => break other_option,
                                    }
                                }
                                None => break Some(Ordering::Greater),
                            },
                            None => match other_next {
                                Some(_) => break Some(Ordering::Less),
                                None => break Some(Ordering::Equal),
                            },
                        }
                    }
                }
            },
        }
    }
}

fn parse_data(data_string: &str) -> PacketData {
    let mut chars = data_string.chars();
    if chars.next().unwrap() == '[' {
        if chars.next().unwrap() == ']' {
            PacketData::List(Vec::new())
        } else {
            let length = data_string.len();
            let list_body = &data_string[1..length - 1];
            let mut list = Vec::new();
            let mut depth = 0;
            let mut left = 0;
            for (index, character) in list_body.chars().enumerate() {
                match character {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    ',' => {
                        if depth == 0 {
                            list.push(parse_data(&list_body[left..index]));
                            left = index + 1;
                        }
                    }
                    _ => (),
                }
            }
            list.push(parse_data(&list_body[left..]));
            PacketData::List(list)
        }
    } else {
        PacketData::Integer(data_string.parse().unwrap())
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
                let packet1 = parse_data(packets.next().unwrap());
                let packet2 = parse_data(packets.next().unwrap());
                (packet1 < packet2) as usize * (index + 1)
            })
            .sum::<usize>()
    )
}
