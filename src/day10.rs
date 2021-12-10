
pub fn main() {
    crate::util::test(1, 26397, part1("day10_test"));
    crate::util::do_part(1, part1, "day10");
    crate::util::test(2, 288957, part2("day10_test"));
    crate::util::do_part(2, part2, "day10");
}

pub fn part1(filename: &str) -> i128 {
    let strings = crate::util::read_lines(filename);
    
    let mut ret = 0;
    for string in strings {
        ret += get_state(string).1;
    }
    ret
}

const PAIRS: [(char, char); 4] = [ ('(', ')'), ('[', ']'), ('{', '}'), ('<', '>') ];

const ES: [(char, i128); 4] = [ (')', 3), (']', 57), ('}', 1197), ('>', 25137) ];

fn get_state(string: String) -> (Vec<char>, i128) {
    // we return the open stack for part2

    let mut open: Vec<char> = Vec::new();

    let openers: Vec<char> = PAIRS.iter().map(|(o, _)| *o).collect();
    // let closers: Vec<char> = PAIRS.iter().map(|(_, c)| *c).collect();

    for c in string.chars().collect::<Vec<char>>() {
        if openers.iter().find(|&&x| x == c) != None {
            open.push(c);
        } else {
            if matches(open[open.len() - 1], c) {
                open.pop();
            } else {
                return (open, get_char_error_score(c))
            }
        }
    }

    (open, 0)
}

fn matches(o: char, c: char) -> bool {
    PAIRS.iter().find(|&&(a, b)| a == o && b == c) != None
}

fn get_char_error_score(c: char) -> i128 {
    _get_score(c, ES)
}

fn _get_score(c: char, scores: [(char, i128); 4]) -> i128 {
    *scores.iter().find(|&&(cc, _)| cc == c).map(|(_, score)| score).unwrap()
}

pub fn part2(filename: &str) -> i128 {
    let strings = crate::util::read_lines(filename);

    let mut scores = Vec::new();

    for string in strings {
        let (open, error) = get_state(string);
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
        if open.len() == 0 {
            break;
        }
        let curr_open = open.pop().unwrap();
        let c = *PAIRS.iter().find(|&&(o, _)| o == curr_open).map(|(_, c)| c).unwrap();
        ret *= 5;
        ret += get_char_closing_score(c)
    }
    ret
}

fn get_char_closing_score(c: char) -> i128 {
    _get_score(c, CS)
}
