use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn to_i32(strings: Vec<String>) -> Vec<i32> {
    let mut ret = Vec::new();
    for s in strings {
        ret.push(s.parse().unwrap());
    }
    ret
}

pub fn read_lines(filename: &str) -> Vec<String> {
    let mut str = "src/".to_owned();
    str.push_str(filename);
    return _read_lines(str);
}

fn _read_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let mut ret = Vec::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        ret.push(line.unwrap());
    }
    ret
}
