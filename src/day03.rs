pub fn part1(filename: &str) -> i32 {
    let strings = crate::util::read_lines(filename);
    
    // length of strings
    let len = strings.iter().next().unwrap().len();

    let mut cnt1s = vec!(0; len);
    for s in &strings {
        for (i, c) in s.chars().enumerate() {
            if c == '1' {
                cnt1s[i] += 1
            }
        }
    }

    let mut gamma = "".to_owned();
    let mut epsilon = "".to_owned();
    {
        let thh = strings.len() / 2;

        for i in 0..len {
            let more1 = cnt1s[i] >= thh;
            gamma.push_str(if more1 { "1" } else { "0" });
            epsilon.push_str(if more1 { "0" } else { "1" });
        }
    }

    to_i32(&gamma) * to_i32(&epsilon)
}

fn to_i32(s: &str) -> i32 {
    isize::from_str_radix(s, 2).unwrap() as i32
}

fn filter(strings: Vec<String>, pos: usize, most: bool) -> Vec<String> {
    let mut ret1 = Vec::new();
    let mut ret0 = Vec::new();

    for s in strings {
        let c = s.chars().nth(pos as usize).unwrap();
        if c == '1' {
            ret1.push(s)
        } else {
            ret0.push(s)
        }
    }

    if (ret1.len() >= ret0.len()) ^ !most {
        ret1
    } else {
        ret0
    }
}

fn get_filtered(strings: &Vec<String>, len: usize, most: bool) -> i32 {
    let mut filtered = strings.clone();
    for pos in 0.. len {
        filtered = filter(filtered, pos, most);
        if filtered.len() == 1 {
            break;
        }
    }

    to_i32(filtered.iter().next().unwrap())
}

pub fn part2(filename: &str) -> i32 {
    let strings = crate::util::read_lines(filename);
    let len = strings.iter().next().unwrap().len() as usize;

    let oxy_r = get_filtered(&strings, len, true); // oxygen generator rating
    let co2_r = get_filtered(&strings, len, false); // CO2 scrubber rating
    
    oxy_r * co2_r
}