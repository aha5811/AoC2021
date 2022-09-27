
pub fn main() {
    crate::util::test(1, 40, part1("day15_test"));
    crate::util::do_part(1, part1, "day15");
    crate::util::test(2, 1, part2("day15_test"));
    crate::util::do_part(2, part2, "day15");
}

pub fn part1(filename: &str) -> usize {
    let strings = crate::util::read_lines(filename);
    let b = read_board(strings);
    
    let mut path = Vec::<(usize, usize, usize)>::new();
    path.push((0, 0, 1)); // x y risk_sum

    let res = find_path(b, &path);
    res[&res.len() - 1].1
}

/*
struct Path {
    head: (usize, usize),
    tail: Tail
}

enum Tail {
    Path,
    Nil
}
*/

fn find_path(b: Board, path: &Vec<(usize, usize, usize)>) -> &Vec<(usize, usize, usize)> {

    // get current head ([len - 1])
    let (x, y, risk) = path[path.len() - 1];

    // if path = done return path
    if x == b.width - 1 && y == b.height - 1 {
       return path
    }

    // check all neighbours if possible
    let check = Vec::new();
    {
        if x > 0 {
            check.push((x - 1, y));
        }
        if x < b.width - 1 {
            check.push((x + 1, y))
        }
        if y > 0 { 
            check.push((x, y - 1));
        }
        if y < b.height - 1 {
            check.push((x, y + 1));
        }
    }
    // check if not already on path
    check.retain(|p| !visited(path, p));

    // for all remaining get all find_path and choose the least risky
    
    
}

fn visited(path: &Vec<(usize, usize, usize)>, p: &(usize, usize)) -> bool {
    path.iter().find(|(x, y, _)| *x == p.0 && *y == p.1) != None
}

fn read_board(strings: Vec<String>) -> Board {
    let mut w = 0;
    let mut es = Vec::<usize>::new();
    for string in strings {
        let mut ns = read_ns(string);
        w = ns.len();
        es.append(&mut ns);
    }

    Board { width: w, height: es.len() / w, risks: es }
}

fn read_ns(string: String) -> Vec<usize> {
    string.chars().map(|c| (c as i32 - 0x30) as usize).collect()
}

struct Board {
    width: usize,
    height: usize,
    risks: Vec<usize>
}

impl Board {

    fn get_r(&self, x: usize, y: usize) -> usize {
        self.risks[self.xy_to_p((x, y))]
    }

    fn p_to_xy(&self, p: usize) -> (usize, usize) {
        let x = p % self.width;
        let y = (p - x) / self.width;
        (x, y)
    }

    fn xy_to_p(&self, (x, y): (usize, usize)) -> usize {
        self.width * y + x
    }

}

pub fn part2(filename: &str) -> i32 {
    let _strings = crate::util::read_lines(filename);
    
    0
}
