
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

    let mut f = vec![0; 9];

    for n in ns {
        f[n as usize] += 1
    }

    for _ in 0..days {
        let f_minus_1 = f[0];
        for p in 0..8 {
            f[p] = f[p + 1];
        }
        f[8] = f_minus_1;
        f[6] += f_minus_1;
    }

    let mut ret = 0;
    
    for n in f {
        ret += n
    }

    ret
}
