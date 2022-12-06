use std::fs;

fn main() {
    let mut count = 0;
    let contents = fs::read_to_string("input").unwrap();
    for line in contents.lines() {
        let mut ranges = line.split(",");
        let mut range1 = ranges.next().unwrap().split("-");
        let lower1: i32 = range1.next().unwrap().parse().unwrap();
        let upper1: i32 = range1.next().unwrap().parse().unwrap();
        let mut range2 = ranges.next().unwrap().split("-");
        let lower2: i32 = range2.next().unwrap().parse().unwrap();
        let upper2: i32 = range2.next().unwrap().parse().unwrap();
        if (lower1 - lower2) * (upper2 - upper1) >= 0 {
            count += 1;
        }
    }
    println!("{}", count)
}
