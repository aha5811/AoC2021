
pub fn main() {
    crate::util::test_2(1, "18", 26, part1("day06_test", 18));
    crate::util::test_2(1, "80", 5934, part1("day06_test", 80));
    crate::util::do_part_2(1, part1, "day06", 80);
    crate::util::test_2(2, "256", 26984457539, part2("day06_test", 256));
    crate::util::do_part_2(2, part2, "day06", 256);
}

pub fn part1(filename: &str, days: usize) -> i32 {
    let mut strings = crate::util::read_lines(filename);
    let mut ns = crate::util::read_ns(strings.remove(0));

    for _ in 0..days {
        for p in 0..ns.len() {
            ns[p] -= 1;
            if ns[p] == -1 {
                ns[p] = 6;
                ns.push(8);
            }
        }
    }

    ns.len() as i32
}

pub fn part2(filename: &str, days: usize) -> i128 {
    let mut strings = crate::util::read_lines(filename);
    let ns = crate::util::read_ns(strings.remove(0));

    // idea: all fish of a certain age behave exactly the same, so group fish by age

    // init the fish buckets

    let mut f = vec![0; 9];

    for n in ns {
        f[n as usize] += 1
    }

    // age

    for _ in 0..days {
        let f_minus_1 = f[0];
        for p in 0..8 {
            f[p] = f[p + 1];
        }
        f[8] = f_minus_1;
        f[6] += f_minus_1;
    }

    // sum

    let mut ret = 0;
    
    for n in f {
        ret += n
    }

    ret
}
