
pub fn main() {
    crate::util::test(1, 17, part1("day13_test"));
    crate::util::do_part(1, part1, "day13");
    crate::util::test(2, 16, part2("day13_test"));
    crate::util::do_part(2, part2, "day13");
}

const OUTPATH: &str = "output/day13/";

pub fn part1(filename: &str) -> usize {
    compute(filename, false)
}

pub fn part2(filename: &str) -> usize {
    compute(filename, true)
}

use std::cmp;

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
    let mut b = new_board(maxx + 1, maxy + 1);

    for p in points {
        b.dot(p)
    }

    let mut i = 1;
    for (x_else_y, fold_at) in folds {
        b = b.fold(x_else_y, fold_at);
        if !for_part2 { // part1 -> only one fold
            break;
        } else {
            output(&b, format!("{0}step{1}.bmp", OUTPATH, i));
        }
        i += 1;
    }

    if for_part2 { // part2 -> output
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

    fn is_dot(&self, xy: (usize, usize)) -> bool {
        self.dots[self.xy_to_p(xy)]
    }

    fn xy_to_p(&self, (x, y): (usize, usize)) -> usize {
        self.width * y + x
    }

    fn fold(&self, x_else_y: bool, fold_at: usize) -> Board {
        
        // next board dim & dots
        let mut ret =
            new_board(
                if x_else_y { fold_at } else { self.width },
                if x_else_y { self.height } else { fold_at }
            );

        // fold
        for a in 0 .. fold_at {
            for b in 0 .. if x_else_y { self.height } else { self.width } {
                let xy = if x_else_y { (a, b) } else { (b, a) };
                let a_mirror = fold_at + (fold_at - a);
                let xy_mirror = if x_else_y { (a_mirror, b) } else { (b, a_mirror) };
                if self.is_dot(xy) || self.is_dot(xy_mirror) {
                    ret.dot(xy);
                }
            }
        }

        ret
    }

    fn dots(&self) -> usize {
        self.dots.iter().map(|&b| { if b { 1 } else { 0 } }).sum()
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", if self.is_dot((x, y)) { "#" } else { "." });
            }
            println!();
        }
    }

}

fn new_board(width: usize, height: usize) -> Board {
    Board { width: width, height: height, dots: vec![false; width * height] }
}

fn read_fold(s: &String) -> (bool, usize) {
    let sp: Vec<String> = s.split("=").map(|ss| ss.to_owned()).collect();
    let n = sp[1].parse::<usize>().unwrap();
    let char = sp[0].chars().last().unwrap();
    (char == 'x', n)
}

fn read_point(s: &String) -> (usize, usize) {
    let xy = crate::util::read_ns(s.to_owned());
    (xy[0] as usize, xy[1] as usize)
}

use bmp::{Image, Pixel};

fn output(b: &Board, fname: String) {
    let f = 1; // magnifying factor

    let mut img = Image::new((b.width * f) as u32, (b.height * f) as u32);
    for x in 0..b.width {
        for y in 0..b.height {
            let e = if b.is_dot((x, y)) { 0 } else { 255 };
            for dx in 0..f {
                for dy in 0..f {
                    img.set_pixel((x * f + dx) as u32, (y * f + dy) as u32, Pixel::new(e, e, e));
                }
            }
        }
    }
    let _ = img.save(fname);
}
