pub fn day01_1(s: &str) -> i32 {
    return day01(&super::util::to_i(super::util::read_lines(s)));
}

fn day01(ns: &Vec<i32>) -> i32 {
    let mut cnt = 0;
    let mut last = -1;
    for curr in ns {
        if last != -1 && curr > &last {
            cnt = cnt + 1
        }
        last = *curr;
    }
    cnt   
}

pub fn day01_2(s: &str) -> i32 {
    let ns = super::util::to_i(super::util::read_lines(s));

    let mut a = vec![0; ns.len() + 2];

    for (i, n) in ns.iter().enumerate() {
        a[i] = a[i] + n;
        a[i + 1] = a[i + 1] + n;
        a[i + 2] = a[i + 2] + n;
    }

    let a = &a[2..a.len() - 2];

    return day01(&a.to_vec());
}
