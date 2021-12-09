
pub fn main() {
    crate::util::test_i32("day08 #1 test", 26, part1("day08_test"));
    println!("day08 #1: {}", part1("day08"));
    crate::util::test_i32("day08 #2 test", 61229, part2("day08_test"));
    println!("day08 #2: {}", part2("day08"));
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

pub fn part2(filename: &str) -> i32 {
    let strings = crate::util::read_lines(filename);
    let ios = read_ios(strings);
    let iios = to_iios(ios);
   
    let mut ret: i32 = 0;

    for mut iio in iios {

        let sol = solve_wiring(&mut iio);
        // -> char[] 0~a 1~b 2~c 3~d 4~e 5~f 6~g
        // [1234560] -> 0=1 (a is connected to b), 1=2 (b is connected to c), ...

        let mut n = 0;
        for (i, o) in iio.out.iter().enumerate() {
            let d = to_digit(o, &sol) as i32;
            let multi = 10i32.pow(3 - i as u32);
            n += d * multi;
            // translate out to digits
        }

        ret += n;
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

fn solve_wiring(iio: &mut IIO) -> Vec<usize> {

    let mut all = Vec::<Vec<usize>>::new();
    add_no_dupes(&mut all, &mut iio.inp);
    add_no_dupes(&mut all, &mut iio.out);
    // all ten digits

    // 1 = length 2
    // 4 = length 4
    // 7 = length 3
    // 8 = length 7

    // get 1 and 7, subtract to get a, 1 is (cf)
    // get 1 and 4, subtract to get (bd)
    // in all with len 5, only 5 has (bd) -> 5
    // in all with len 6, only 0 has not (bd) -> 0
    // diff between 8 and 0 is d -> b
    // subtract abd from 5, the rest (fg) shares f with (cf) -> f -> c
    // a b c d f
    // remove all from 5 -> g
    // len 6 = 6 or 9, 6 does not contain c -> 6, 6 diff 5 is e

    let cf = find_len(&all, 2); // 1
    let bcdf = find_len(&all, 4); // 4
    let acf = find_len(&all, 3); // 7
    let eight = find_len(&all, 7);
    let a = subtract(&acf, &cf);
    let bd = subtract(&bcdf, &cf);
    let fives = find_lens(&all, 5); // 5 2 3
    let sixes = find_lens(&all, 6); // 6 9 0
    let five = find_with(&fives, &bd);
    let six_nine = find_withs(&sixes, &bd); // 6 9
    let zero = subtracts(&sixes, &six_nine)[0].clone();
    let d = subtract(&eight, &zero);
    let b = subtract(&bd, &d);
    let bdfg = subtract(&five, &a);
    let fg = subtract(&bdfg, &bd);
    let c = subtract(&cf, &fg);
    let f = subtract(&cf, &c);
    let g = subtract(&fg, &f);
    let nine_s = find_withs(&six_nine, &c);
    let six = subtracts(&six_nine, &nine_s)[0].clone();
    let e = subtract(&six, &five);

    let mut ret = Vec::new();
    ret.push(a[0]);
    ret.push(b[0]);
    ret.push(c[0]);
    ret.push(d[0]);
    ret.push(e[0]);
    ret.push(f[0]);
    ret.push(g[0]);
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

fn find_with(vs: &Vec<Vec<usize>>, pattern: &Vec<usize>) -> Vec<usize> {
    for v in vs {
        if has_with(v, pattern) {
            return v.clone()
        }
    }
    
    Vec::new() // won't happen
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

fn find_len(vs: &Vec<Vec<usize>>, l: usize) -> Vec<usize> {
    for v in vs {
        if v.len() == l {
            return v.clone();
        }
    }

    Vec::new() // won't happen
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

fn to_digit(ds: &Vec<usize>, sol: &Vec<usize>) -> usize {
    // ds [1,2,3,4]
    // sol [0, 1, 2, 3, 4, 5, 6, 7, 8]
    let mut u = Vec::<usize>::new();
    for d in ds {
        u.push(sol[*d]);
    }
    u.sort();

    for (i, au) in strings().iter().enumerate() {
        if is_same(&u, au) {
            return i
        }
    }

    0
}

const STRINGS: [&str; 10] = [
    "abc_efg", // 0
    "__c__f_", // 1
    "a_cde_g", // 2
    "a_cd_fg", // 3
    "_bcd_f_", // 4
    "ab_d_fg", // 5
    "ab_defg", // 6
    "a_c__f_", // 7
    "abcdefg", // 8
    "abcd_fg" // 9
];

fn strings() -> Vec<Vec<usize>> {
    let mut ret = Vec::<Vec<usize>>::new();

    for string in STRINGS {
        let mut u = Vec::<usize>::new();
        for cc in string.chars() {
            if cc != '_' {
                u.push(CHARS.find(cc).unwrap() as usize);
            }
        }
        u.sort();
        ret.push(u);
    }

    ret
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
