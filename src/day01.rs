
pub fn main() {
    println!("day01 #1: {}", part1("day01"));
    crate::util::test_i32("day01 #2 test", 5, part2("day01_2_test"));
    println!("day01 #2: {}", part2("day01"));
}

pub fn part1(filename: &str) -> i32 {
    cnt_inc(crate::util::to_i32(crate::util::read_lines(filename)))
}

fn cnt_inc(ns: Vec<i32>) -> i32 {
    let mut cnt = 0;

    let mut last = -1;

    for n in ns {
        if last != -1 && n > last {
            cnt = cnt + 1
        }
        last = n
    }

    cnt
}

pub fn part2(filename: &str) -> i32 {
    let ns = crate::util::to_i32(crate::util::read_lines(filename));

    let wsize = 2; // window size 3

    // generate list of all window sums

    let mut a = vec![0; ns.len() + wsize]; // bigger

    for (p, n) in ns.iter().enumerate() {
        for i in 0..=wsize {
            a[p + i] = a[p + i] + n
        }
    }

    let a = &a[wsize .. a.len() - wsize]; // cut invalid values

    cnt_inc(a.to_vec()) // use part1
}
