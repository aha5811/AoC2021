
pub fn main() {
    crate::util::test_2(1, "len 5", 97, test_part1("day14_test", 5));
    crate::util::test_2(1, "len 10", 3073, test_part1("day14_test", 10));
    crate::util::test_2(1, "10", 1588, part1("day14_test", 10));
    crate::util::do_part_2(1, part1, "day14", 10);
    // not working
    // crate::util::test(2, 2188189693529, part2("day14_test", 40));
    // crate::util::do_part_2(2, part2, "day14", 40);
}

fn compute_part1(filename: &str, steps: usize) -> String {
    let (mut template, pirs) = read_input(filename);

    for _ in 0..steps {
        template = apply(template, &pirs);
    }

    template
}

fn read_input(filename: &str) -> (String, Vec<(String, String)>) {
    let mut strings = crate::util::read_lines(filename);
    
    let template = strings.remove(0);

    strings.remove(0); // empty line

    let mut pirs = Vec::new();
    for string in strings {
        let sp: Vec<String> = string.split(" -> ").map(|s| s.to_owned()).collect();
        pirs.push((sp[0].clone(), sp[1].clone()));
    }

    (template, pirs)
}

fn test_part1(filename: &str, steps: usize) -> usize {
    compute_part1(filename, steps).len()
}

use std::collections::HashMap;
use std::cmp;

pub fn part1(filename: &str, steps: usize) -> u128 {
    let string = compute_part1(filename, steps);

    // cnt occurrences of chars
    let chars: Vec<char> = string.chars().collect();
    let mut m: HashMap<char, u128> = HashMap::new();
    for x in chars {
        *m.entry(x).or_default() += 1;
    }

    get_maxmin(m)
}

fn get_maxmin(m: HashMap<char, u128>) -> u128 {
    let mut max = 0;
    let mut min = u128::MAX;
    for (_, cnt) in m.iter() {
        max = cmp::max(max, *cnt);
        min = cmp::min(min, *cnt);
    }
    max - min
}

fn apply(s: String, pirs: &Vec<(String, String)>) -> String {
    let mut ret = "".to_owned();
    for i in 0..(s.len() - 1) {
        ret.push(s.chars().nth(i).unwrap());
        let look = &s[i ..= i + 1].to_owned();
        for (pair, res) in pirs {
            if pair == look {
                ret.push_str(res);
                break;
            }
        }
    }
    ret.push(s.chars().last().unwrap());
    ret
}

pub fn part2(filename: &str, steps: usize) -> u128 {
    let (template, pirs) = read_input(filename);
 
    let tchars: Vec<char> = template.chars().collect();
    let mut m: HashMap<char, u128> = HashMap::new();

    let mut two_chars = Vec::new();
    for i in 0..tchars.len() - 1 {
        let mut t = "".to_owned();
        t.push(tchars[i]);
        t.push(tchars[i + 1]);
        two_chars.push(t);
    }

    compute_rec(&two_chars, steps, &pirs, &mut m);

    let last = &two_chars[&two_chars.len() - 1].chars().last().unwrap();
    *m.entry(*last).or_default() += 1;

    get_maxmin(m)
}

fn compute_rec(tcs: &Vec<String>, steps: usize, pirs: &Vec<(String, String)>, m: &mut HashMap<char, u128>) {
    if steps == 0 {
        let _ = tcs.iter().map(|s| { *m.entry(s.chars().nth(0).unwrap()).or_default() += 1; });
    } else {
        for tc in tcs {
            compute_rec_single(&tc, steps, pirs, m);
        }
    }
}

fn compute_rec_single(tc: &String, steps: usize, pirs: &Vec<(String, String)>, m: &mut HashMap<char, u128>) {
    let insert = pirs.iter().find(|(p, _)| p == tc).map(|(_, c)| c).unwrap();

    let mut tc1 = "".to_owned();
    tc1.push(tc.chars().nth(0).unwrap());
    tc1.push_str(insert);

    let mut tc2 = "".to_owned();
    tc2.push_str(insert);
    tc2.push(tc.chars().last().unwrap());

    let mut tcs = Vec::new();
    tcs.push(tc1);
    tcs.push(tc2);

    compute_rec(&tcs, steps - 1, pirs, m);
}