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

const DIR: &str = "src/inputs/";

pub fn read_lines(filename: &str) -> Vec<String> {
    let mut str = DIR.to_owned();
    str.push_str(filename);
    _read_lines(str)
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

pub fn read_ns(string: String) -> Vec<i32> {
    return string.split(',').map(|n| n.parse::<i32>().unwrap()).collect()
}

pub fn test_i32(string: &str, exp: i32, res: i32) {
    if exp == res {
        println!("{} ok", string);
    } else {
        println!("{0} failed! expected {1:?} but was {2:?}", string, exp, res)
    }
}

pub fn test_i128(string: &str, exp: i128, res: i128) {
    if exp == res {
        println!("{} ok", string);
    } else {
        println!("{0} failed! expected {1:?} but was {2:?}", string, exp, res)
    }
}
