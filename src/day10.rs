
pub fn main() {
    crate::util::test(1, 26397, part1("day10_test"));
    crate::util::do_part(1, part1, "day10");
    crate::util::test(2, 288957, part2("day10_test"));
    crate::util::do_part(2, part2, "day10");
}

pub fn part1(filename: &str) -> i32 {
    let strings = crate::util::read_lines(filename);
    
    let mut ret = 0;
    for string in strings {
        ret += get_error(string).1;
    }
    ret
}

const PAIRS: [(char, char); 4] = [ ('(', ')'), ('[', ']'), ('{', '}'), ('<', '>') ];

const ES: [(char, i32); 4] = [ (')', 3), (']', 57), ('}', 1197), ('>', 25137) ];

fn get_error(string: String) -> (Vec<char>, i32) {
    // we return the open stack for part2

    let mut open: Vec<char> = Vec::new();

    let openers: Vec<char> = PAIRS.iter().map(|(o, _)| *o).collect();
    // let closers: Vec<char> = PAIRS.iter().map(|(_, c)| *c).collect();

    for c in string.chars().collect::<Vec<char>>() {
        if openers.iter().find(|&&x| x == c) != None {
            open.push(c);
        } else {
            if matches(open.pop().unwrap(), c) {
                // already popped
            } else {
                open.push(c); // re-push (there's no peek and pop+push is faster than seek)
                return (open, get_error_score(c))
            }
        }
    }

    (open, 0)
}

fn matches(o: char, c: char) -> bool {
    PAIRS.iter().find(|&&(a, b)| a == o && b == c) != None
}

fn get_error_score(c: char) -> i32 {
    for (cc, score) in ES {
        if c == cc {
            return score;
        }
    }
    0
}

pub fn part2(filename: &str) -> i128 {
    let strings = crate::util::read_lines(filename);

    let mut scores = Vec::new();

    for string in strings {
        let (open, error) = get_error(string);
        if error == 0 {
            scores.push(get_closing_score(open))
        }
    }

    // median

    scores.sort();
    scores[(scores.len() - 1) / 2]
}

const CS: [(char, i128); 4] = [ (')', 1), (']', 2), ('}', 3), ('>', 4) ];

fn get_closing_score(mut open: Vec<char>) -> i128 {
    let mut ret: i128 = 0;
    loop {
        let co = open.pop().unwrap();
        for (o, c) in PAIRS.iter() {
            if *o == co {
                ret *= 5;
                ret += get_closing_score_char(*c)
            }
        }
        if open.len() == 0 {
            break;
        }
    }
    ret
}

fn get_closing_score_char(c: char) -> i128 {
    for (cc, score) in CS {
        if c == cc {
            return score;
        }
    }
    0
}
