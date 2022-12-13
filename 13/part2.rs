use std::{cmp::Ordering, fs::read_to_string};

#[derive(PartialEq, Eq)]
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

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        if *self < *other {
            Ordering::Less
        } else if *self == *other {
            Ordering::Equal
        } else {
            Ordering::Greater
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
    let mut packets: Vec<PacketData> = contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_data(line))
        .collect();
    packets.sort();
    let divider1 = parse_data("[[2]]");
    let divider2 = parse_data("[[6]]");
    let mut decoder_key = 0;
    for (index, packet) in packets.iter().enumerate() {
        if decoder_key == 0 && packet > &divider1 {
            decoder_key = index + 1;
        }
        if packet > &divider2 {
            decoder_key *= index + 2;
            break;
        }
    }
    println!("{}", decoder_key)
}
