
pub fn main() {
    crate::util::test(1, 37, part1("day07_test"));
    crate::util::do_part(1, part1, "day07");
    crate::util::test(2, 168, part2("day07_test"));
    crate::util::do_part(2, part2, "day07");
}

use std::cmp;

pub fn part1(filename: &str) -> i32 {
    compute(filename, dist)
}

pub fn part2(filename: &str) -> i32 {
    compute(filename, incr)
}

fn dist(a: i32, b: i32) -> i32 {
    (a - b).abs()
}

fn incr(a: i32, b: i32) -> i32 {
    let l = dist(a, b);
    (l * (l + 1)) / 2 // gauss
}

fn compute(filename: &str, cost: fn(i32, i32) -> i32) -> i32 {
    let mut strings = crate::util::read_lines(filename);
    let ps = crate::util::read_ns(strings.remove(0)); // crab positions

    let pmin = *ps.iter().min().unwrap();
    let pmax = *ps.iter().max().unwrap();

    let mut minfuel = i32::MAX;

    for g in pmin..=pmax { // goal
        let mut fuel = 0;
        for p in &ps {
            fuel += cost(g, *p);
        }
        minfuel = cmp::min(minfuel, fuel);
    }

    minfuel
}
