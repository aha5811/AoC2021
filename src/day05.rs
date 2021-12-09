
pub fn main() {
    crate::util::test_i32("day05 #1 test", 5, part1("day05_test"));
    println!("day05 #1: {}", part1("day05"));
    crate::util::test_i32("day05 #2 test", 12, part2("day05_test"));
    println!("day05 #2: {}", part2("day05"));
}

use std::cmp;

pub fn part1(filename: &str) -> i32 {
    compute(filename, false)
}

pub fn part2(filename: &str) -> i32 {
    compute(filename, true)
}

fn compute(filename: &str, with_diags: bool) -> i32 {
    let strings = crate::util::read_lines(filename);

    // read lines

    let mut lines = Vec::<Line>::new();
    for string in strings { // "0,9 -> 5,9"
        let ps: Vec<String> =
            string
            .split(' ')
            .filter(|s| s.find(',') != None)
            .map(|s| s.to_owned())
            .collect();
        lines.push(Line { start: read_point(ps[0].to_owned()), end:  read_point(ps[1].to_owned()) });
    }

    // board setup

    let mut maxx = 0;
    let mut maxy = 0;
    for line in &lines {
        let max = line.max();
        maxx = cmp::max(maxx, max.0);
        maxy = cmp::max(maxy, max.1);
    }
    maxx += 1;
    maxy += 1;

    let mut b = Board{
        width: maxx,
        data: vec![0; maxx * maxy]
    };

    // draw lines

    for line in lines {
        if with_diags || line.is_line() {
            for p in line.get_points() {
                b.add_point(p);
            }
        }
    }

    b.cnt(1)
}

fn read_point(s: String) -> Point {
    let cs: Vec<usize> = s
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    Point { x: cs[0], y: cs[1] }
}

struct Board {
    width: usize,
    data: Vec<usize>
}

impl Board {

    fn add_point(&mut self, p: Point) {
        let i = self.width * p.y + p.x;
        self.data[i] += 1
    }

    fn cnt(self, t: usize) -> i32 {
        let mut ret = 0;

        for i in self.data.iter() {
            if i > &t {
                ret += 1
            }
        }

        ret
    }

}

struct Point {
    x: usize,
    y: usize
}

struct Line {
    start: Point,
    end: Point
}

impl Line {

    fn is_line(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn max(&self) -> (usize, usize) {
        (cmp::max(self.start.x, self.end.x), cmp::max(self.start.y, self.end.y))
    }

    fn get_points(&self) -> Vec<Point> {
        let mut ret = Vec::new();

        let dx =
            if self.start.x == self.end.x { 0 }
            else if self.start.x < self.end.x { 1 }
            else { -1 };
        let dy =
            if self.start.y == self.end.y { 0 }
            else if self.start.y < self.end.y { 1 }
            else { -1 };

        let mut i = 0;
        loop {
            let x = (self.start.x as i32 + i * dx) as usize;
            let y = (self.start.y as i32 + i * dy) as usize;
            ret.push(Point { x: x, y: y });
            if x == self.end.x && y == self.end.y {
                break;
            }
            i += 1;
        }

        ret
    }

}
