use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::{Instant};
use std::fmt::Display;

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

pub fn test<T: PartialEq + Display>(n: u8, exp: T, res: T) {
    let pre = format!("test part{}", n);
    _test(pre, exp, res);
}

pub fn test_2<T: PartialEq + Display>(n: u8, s: &str, exp: T, res: T) {
    let pre = format!("part{0} test ({1})", n, s);
    _test(pre, exp, res);
}

fn _test<T: PartialEq + Display>(pre: String, exp: T, res: T) {
    if exp == res {
        println!("{} ok", pre);
    } else {
        println!("{0} failed! expected {1} but was {2}", pre, exp, res);
        panic!();
    }
}

pub fn do_day(f: fn(), s: &str) {
    println!("----- {} -----", s);
    f();
    println!("");
}

pub fn do_part<T: Display, A>(n: u8, f: fn(arg: A) -> T, arg: A) {
    let start = Instant::now();
    let res = f(arg);
    _do_part(start, res, n)
}

pub fn do_part_2<T: Display, A1, A2>(n: u8, f: fn(arg1: A1, arg2: A2) -> T, arg1: A1, arg2: A2) {
    let start = Instant::now();
    let res = f(arg1, arg2);
    _do_part(start, res, n)
}

fn _do_part<T: Display>(i: Instant, res: T, n: u8) {
    println!("part{0}: {1} ({2:?})", n, res, i.elapsed());
}
