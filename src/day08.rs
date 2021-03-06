
pub fn main() {
    crate::util::test(1, 26, part1("day08_test"));
    crate::util::do_part(1, part1, "day08");
    crate::util::test(2, 61229, part2("day08_test"));
    crate::util::do_part(2, part2, "day08");
}

pub fn part1(filename: &str) -> i32 {
    let strings = crate::util::read_lines(filename);
    let ios = read_ios(strings);

    let mut ret = 0;

    // 1 = length 2
    // 4 = length 4
    // 7 = length 3
    // 8 = length 7

    for io in ios {
        for o in io.out {
            let l = o.len();
            if l == 2 || l == 4 || l == 3 || l == 7 {
                ret += 1
            }
        }
    }

    ret
}

fn read_ios(strings: Vec<String>) -> Vec::<IO> {
    let mut ret = Vec::<IO>::new();

    for string in strings {
        let s = s_split_to_v(&string, '|');
        ret.push(IO {
            inp: s_split_to_v(&s[0], ' '),
            out: s_split_to_v(&s[1], ' ')
        });
    }

    ret
}

fn s_split_to_v(string: &String, c: char) -> Vec<String> {
    string.split(c).map(|s| s.trim().to_owned()).collect()
}

struct IO {
    inp: Vec<String>, // [abcd, cdfa, ...] length 10
    out: Vec<String>  // ^ length 4
}

pub fn part2(filename: &str) -> usize {
    let strings = crate::util::read_lines(filename);
    let ios = read_ios(strings);
    let iios = to_iios(ios);
   
    let mut ret = 0;
    for mut iio in iios {
        ret += solve_wiring(&mut iio);
    }
    ret
}

struct IIO {
    inp: Vec<Vec<usize>>, // [[0,1,2,3], [1,2,3,4], ...] length 10
    out: Vec<Vec<usize>>  // ^ length 4
}

fn to_iios(ios: Vec<IO>) -> Vec<IIO> {
    let mut ret = Vec::<IIO>::new();
    for io in ios {
        ret.push(IIO {
            inp: to_iio(io.inp),
            out: to_iio(io.out)
        })
    }
    ret
}

const CHARS: &str = "abcdefg";

fn to_iio(strings: Vec<String>) -> Vec<Vec<usize>> {
    let mut ret = Vec::<Vec<usize>>::new();
    for string in strings {
        let mut u = Vec::<usize>::new();
        for cc in string.chars().collect::<Vec<char>>() {
            u.push(CHARS.find(cc).unwrap());
        }
        u.sort();
        ret.push(u);
    }
    ret
}

fn solve_wiring(iio: &mut IIO) -> usize {

    let mut all = Vec::<Vec<usize>>::new();
    add_no_dupes(&mut all, &mut iio.inp);
    add_no_dupes(&mut all, &mut iio.out);
    // all ten digits

    let cf = find_lens(&all, 2)[0].clone(); // 1
    let bcdf = find_lens(&all, 4)[0].clone(); // 4
    let acf = find_lens(&all, 3)[0].clone(); // 7
    let eight = find_lens(&all, 7)[0].clone();
    let fives = find_lens(&all, 5); // 5 2 3
    let sixes = find_lens(&all, 6); // 6 9 0

    let a = subtract(&acf, &cf);
    let bd = subtract(&bcdf, &cf);
    let five = find_withs(&fives, &bd)[0].clone(); // only 5 has b
    let six_n_nine = find_withs(&sixes, &bd); // only 6 & 9 have d
    let zero = subtracts(&sixes, &six_n_nine)[0].clone(); // 0 is the other one
    // let d = subtract(&eight, &zero);
    // let b = subtract(&bd, &d);
    let bdfg = subtract(&five, &a);
    let fg = subtract(&bdfg, &bd);
    let c = subtract(&cf, &fg);
    // let f = subtract(&cf, &c);
    // let g = subtract(&fg, &f);
    let nine = find_withs(&six_n_nine, &c)[0].clone(); // only 9 has c
    let six = subtracts1(&six_n_nine, &nine)[0].clone(); // 6 is the other one
    let e = subtract(&six, &five);
    let two_n_three = subtracts1(&fives, &five);
    let two = find_withs(&two_n_three, &e)[0].clone(); // only 2 has e
    let three = subtracts1(&two_n_three, &two)[0].clone(); // 3 is the other one

    // all digit patterns

    let digits = [ zero, cf, two, three, bcdf, five, six, acf, eight, nine ];

    let mut ret = 0;

    // iter over outs

    for (pos, opattern) in iio.out.iter().enumerate() {

        // find same pattern, its position is the result

        let mut n = 0;
        for (digit, pattern) in digits.iter().enumerate() {
            if is_same(opattern, pattern) {
                n = digit
            }
        }
        let multi = 10i32.pow(3 - pos as u32) as usize; // 1000, 100, 10, 1
        ret += n * multi;
    }

    ret
}

fn subtracts(v1s: &Vec<Vec<usize>>, v2s: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut ret = Vec::new();
    for v1 in v1s {
        let mut found = false;
        for v2 in v2s {
            if is_same(v1, v2) {
                found = true;
            }
        }
        if !found {
            ret.push(v1.clone())
        }
    }
    ret
}

fn subtracts1(vs: &Vec<Vec<usize>>, v: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut ret = Vec::new();
    for av in vs {
        if !is_same(av, v) {
            ret.push(av.clone())
        }
    }
    ret
}

fn find_withs(vs: &Vec<Vec<usize>>, pattern: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut ret = Vec::new();
    for v in vs {
        if has_with(v, pattern) {
            ret.push(v.clone())
        }
    }
    ret
}

fn has_with(v: &Vec<usize>, pattern: &Vec<usize>) -> bool {
    for p in pattern {
        if v.iter().find(|&&x| x == *p) == None {
            return false
        }
    }
    true
}

fn find_lens(vs: &Vec<Vec<usize>>, l: usize) -> Vec<Vec<usize>> {
    let mut ret = Vec::new();
    for v in vs {
        if v.len() == l {
            ret.push(v.clone())
        }
    }
    ret
}

fn subtract(u1s: &Vec<usize>, u2s: &Vec<usize>) -> Vec<usize> {
    let mut ret = Vec::new();
    for u in u1s {
        if u2s.iter().find(|&&x| x == *u) == None {
            ret.push(*u)
        }
    }
    ret
}

fn add_no_dupes(dst: &mut Vec<Vec<usize>>, src: &mut Vec<Vec<usize>>) {
    for s in src {
        let mut add = true;
        for d in dst.iter() {
            if is_same(&s, &d) {
                add = false;
            }
        }
        if add {
            dst.push(s.clone());
        }
    }
}

fn is_same(v1: &Vec<usize>, v2: &Vec<usize>) -> bool {
    if v1.len() == v2.len() {
        for i in 0..v1.len() {
            if v1[i] != v2[i] {
                return false
            }
        }
        return true
    }
    false
}
