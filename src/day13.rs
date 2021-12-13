
pub fn main() {
    crate::util::test(1, 17, part1("day13_test"));
    crate::util::do_part(1, part1, "day13");
    crate::util::test(2, 16, part2("day13_test"));
    crate::util::do_part(2, part2, "day13");
}

use std::cmp;

pub fn part1(filename: &str) -> usize {
    compute(filename, false)
}

pub fn part2(filename: &str) -> usize {
    compute(filename, true)
}

fn compute(filename: &str, for_part2: bool) -> usize {
    let strings = crate::util::read_lines(filename);
    
    let mut points = Vec::<(usize, usize)>::new();
    let mut folds = Vec::<(bool, usize)>::new();

    for string in strings {
        if string.len() > 0 {
            match &string[..1] {
                "f" => folds.push(read_fold(&string)),
                _   => points.push(read_point(&string))
            }
        }
    }

    let mut maxx = 0;
    let mut maxy = 0;
    for (x, y) in &points {
        maxx = cmp::max(*x, maxx);
        maxy = cmp::max(*y, maxy);
    }
    let width = maxx + 1;
    let height = maxy + 1;

    let mut b = Board { width: width, height: height, dots: vec![false; width * height] };

    for p in points {
        b.dot(p)
    }

    for (x_or_y, n) in folds {
        b = b.fold(x_or_y, n);
        if !for_part2 {
            break;
        }
    }

    if for_part2 {
        b.print()
    }

    b.dots()
}

struct Board {
    width: usize,
    height: usize,
    dots: Vec<bool>
}

impl Board {

    fn dot(&mut self, xy: (usize, usize)) {
        let p = self.xy_to_p(xy);
        self.dots[p] = true;
    }

    pub fn xy_to_p(&self, (x, y): (usize, usize)) -> usize {
        self.width * y + x
    }

    fn fold(&self, x_or_y: bool, fpos: usize) -> Board {
        
        // next board dim & dots
        let width = if x_or_y { fpos } else { self.width };
        let height = if x_or_y { self.height } else { fpos };
        let mut retboard = Board {
            width: width, height: height,
            dots: vec![false; width * height] };

        // fold
        for a in 0 .. fpos {
            for b in 0 .. if x_or_y { self.height } else { self.width } {
                let xy1 = if x_or_y { (a, b) } else { (b, a) };
                let a_mirror = fpos + (fpos - a);
                let xy2 = if x_or_y { (a_mirror, b) } else { (b, a_mirror) };
                let p1 = self.xy_to_p(xy1);
                let p2 = self.xy_to_p(xy2);
                let bp = retboard.xy_to_p(xy1);
                retboard.dots[bp] = self.dots[p1] || self.dots[p2];
            }
        }

        retboard
    }

    fn dots(&self) -> usize {
        self.dots.iter().map(|b| { if *b { 1 } else { 0 } }).sum()
    }

    fn print(&self) {
        for y in 0..self.height {
            let mut line = "".to_owned();
            for x in 0..self.width {
                let p = self.xy_to_p((x, y));
                line.push_str(if self.dots[p] { "#" } else { "." });
            }
            println!("{}", line);
        }
    }

}

fn read_fold(s: &String) -> (bool, usize) {
    let sp: Vec<String> = s.split("=").map(|ss| ss.to_owned()).collect();
    let n = sp[1].parse::<usize>().unwrap();
    let char = sp[0].chars().last().unwrap();
    (char == 'x', n)
}

fn read_point(s: &String) -> (usize, usize) {
    let xy = crate::util::read_ns(s.to_owned());
    (xy[0] as usize, xy[1]  as usize)
}
