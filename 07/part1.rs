use std::{fs, collections::HashMap};

struct Dir {
    parent: Option<usize>,
    children: HashMap<String, usize>,
    content: HashMap<String, u32>,
    size: u32,
}

fn main() {

    // initialize file system with empty root directory
    let mut cwd = 0;
    let mut fs = vec![Dir {
        parent: None,
        children: HashMap::new(),
        content: HashMap::new(),
        size: 0,
    }];

    // loop over terminal output
    let contents = fs::read_to_string("input").unwrap();
    for line in contents.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words[0] == "$" {

            // change directory
            if words[1] == "cd" {
                cwd = match words[2] {
                    "/" => 0,
                    ".." => fs[cwd].parent.unwrap(),
                    dir => *fs[cwd].children.get(dir).unwrap(),
                }
            }
        } else if words[0] == "dir" {

            // add directory
            let name = words[1];
            if ! fs[cwd].children.contains_key(name) {
                let index = fs.len();
                fs[cwd].children.insert(name.to_string(), index);
                fs.push(Dir {
                    parent: Some(cwd),
                    children: HashMap::new(),
                    content: HashMap::new(),
                    size: 0,
                });
            }
        } else {

            // add file and increase size of parent directories
            let name = words[1];
            if ! fs[cwd].content.contains_key(name) {
                let size: u32 = words[0].parse().unwrap();
                fs[cwd].content.insert(name.to_string(), size);
                let mut index = cwd;
                loop {
                    fs[index].size += size;
                    match fs[index].parent {
                        Some(i) => index = i,
                        None => break,
                    }
                }
            }
        }
    }

    // calculate output
    let mut sum = 0;
    for dir in fs {
        if dir.size <= 100000 {
            sum += dir.size;
        }
    }
    println!("{}", sum)
}
