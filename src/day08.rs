
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

    for iio in &iios {

        let sol = solve_wiring(iio);
        // -> char[] 0~a 1~b 2~c 3~d 4~e 5~f 6~g
        // [1234560] -> 0=1 (a is connected to b), 1=2 (b is connected to c), ...

        for o in &iio.out {
            ret += to_digit(o, &sol) as i32
            // translate out to digits
        }
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

fn solve_wiring(iio: &IIO) -> Vec<usize> {

    // TODO

    let mut ret = Vec::<usize>::new();
    for i in 0..=6 {
        ret.push(i)
    }
    ret
}

fn to_digit(ds: &Vec<usize>, sol: &Vec<usize>) -> usize {
    // ds [1,2,3,4]
    // sol [0, 1, 2, 3, 4, 5, 6, 7, 8]
    let mut u = Vec::<usize>::new();
    for d in ds {
        u.push(sol[*d]);
    }

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
