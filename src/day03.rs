pub fn part1(filename: &str) -> i32 {
    let strings = crate::util::read_lines(filename);
    
    let len = strings.iter().next().unwrap().len();

    let mut cnt1s = vec!(0; len);
    for s in &strings {
        for (i, c) in s.chars().enumerate() {
            if c == '1' {
                cnt1s[i] += 1
            }
        }
    }

    let thh = strings.len() / 2;

    let mut gamma = "".to_owned();
    let mut epsilon = "".to_owned();
    for i in 0..len {
        let gd;
        let ed;
        if cnt1s[i] >= thh {
            gd = "1";
            ed = "0";
        } else {
            gd = "0";
            ed = "1";
        }
        gamma.push_str(gd);
        epsilon.push_str(ed);
    }

    to_i32(&gamma) * to_i32(&epsilon)
}

fn to_i32(s: &str) -> i32 {
    isize::from_str_radix(s, 2).unwrap() as i32
}

fn filter(strings: Vec<String>, p: i32, most: bool) -> Vec<String> {
    let size = strings.len();

    if size == 1 {
        return strings;
    }

    let thh = size as f32 / 2.;

    let mut cnt1 = 0;
    for s in &strings {
        let cc = s.chars().nth(p as usize).unwrap();
        if cc == '1' {
            cnt1 += 1
        }
    }

    let filter;
    if cnt1 as f32 >= thh {
        filter = if most { '1' } else { '0' }
    } else {
        filter = if most { '0' } else { '1' }
    }

    let mut ret = Vec::new();
    
    for s in strings {
        let cc = s.chars().nth(p as usize).unwrap();
        if cc == filter {
            ret.push(s)
        }
    }

    ret
}

fn get_filtered(strings: &Vec<String>, len: i32, most: bool) -> i32 {
    let mut filtered = strings.clone();

    for p in 0.. len {
        filtered = filter(filtered, p, most);
    }

    to_i32(filtered.iter().next().unwrap())
}

pub fn part2(filename: &str) -> i32 {
    let strings = crate::util::read_lines(filename);
    let len = strings.iter().next().unwrap().len() as i32;

    get_filtered(&strings, len, true) // oxygen generator rating
    *
    get_filtered(&strings, len, false) // CO2 scrubber rating
}