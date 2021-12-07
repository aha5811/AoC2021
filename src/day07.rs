use std::cmp;

pub fn part1(filename: &str) -> i32 {
    compute(filename, cost1)
}

pub fn part2(filename: &str) -> i32 {
    compute(filename, cost2)
}

fn cost1(a: i32, b: i32) -> i32 {
    (a - b).abs()
}

fn cost2(a: i32, b: i32) -> i32 {
    let mut ret = 0;

    let l = cost1(a, b);
    for i in 1..=l {
        ret += i;
    }

    ret
}

fn compute(filename: &str, cost: fn(i32, i32) -> i32) -> i32 {
    let mut strings = crate::util::read_lines(filename);
    let ns = crate::util::read_ns(strings.remove(0));

    let mut min = 0;
    let mut max = 0;

    for n in &ns {
        min = cmp::min(min, *n);
        max = cmp::max(max, *n);
    }

    let mut minfuel = i32::MAX;

    for i in min..=max {
        let mut fuel = 0;
        for n in &ns {
            fuel += cost(i, *n);
        }
        minfuel = cmp::min(minfuel, fuel);
    }

    minfuel
}
