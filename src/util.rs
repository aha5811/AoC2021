use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn to_i(v: Vec<String>) -> Vec<i32> {
    let mut ret = Vec::new();
    for line in v {
        ret.push(line.parse().unwrap());
    }
    ret
}

pub fn read_lines(file: &str) -> Vec<String> {
    let mut str = "src/".to_owned();
    str.push_str(&file);
    return _read_lines(str);
}

fn _read_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let mut ret = Vec::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        ret.push(line);
    }
    ret
}
