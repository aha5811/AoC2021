pub fn day01_1(filename: &str) -> i32 {
    return day01(&crate::util::to_i32(crate::util::read_lines(filename)));
}

fn day01(ns: &Vec<i32>) -> i32 {
    let mut cnt = 0;
    let mut last = -1;
    for n in ns {
        if last != -1 && n > &last {
            cnt = cnt + 1
        }
        last = *n
    }
    cnt
}

pub fn day01_2(filename: &str) -> i32 {
    let ns = crate::util::to_i32(crate::util::read_lines(filename));

    let wsize = 2; // window size 3

    let mut a = vec![0; ns.len() + wsize];

    for (p, n) in ns.iter().enumerate() {
        for i in 0..=wsize {
            a[p + i] = a[p + i] + n
        }
    }

    let a = &a[wsize .. a.len() - wsize];

    day01(&a.to_vec())
}
