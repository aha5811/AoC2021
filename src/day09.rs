
pub fn main() {
    crate::util::test(1, 15, part1("day09_test"));
    crate::util::do_part(1, part1, "day09");
    crate::util::test(2, 1134, part2("day09_test"));
    crate::util::do_part(2, part2, "day09");
}

pub fn part1(filename: &str) -> i32 {
    let strings = crate::util::read_lines(filename);
    let b = read_board(strings);

    let mut ret = 0;

    for x in 0..b.width {
        for y in 0..b.height() {
            if b.is_low(x, y) {
                ret += b.get_h(x, y) + 1
            }
        }
    }

    ret as i32
}

fn read_board(strings: Vec<String>) -> Board {
    let mut w = 0;
    let mut hs = Vec::<usize>::new();
    for string in strings {
        let ns = read_ns(string);
        w = ns.len();
        for n in ns {
            hs.push(n as usize)
        }
    }

    Board { width: w, heights: hs }
}

fn read_ns(string: String) -> Vec<usize> {
    string.chars().map(|c| c as i32 - 0x30).map(|i| i as usize).collect()
}

struct Board {
    width: usize,
    heights: Vec<usize>
}

impl Board {

    fn height(&self) -> usize {
        (self.heights.len() / self.width) as usize
    }

    fn get_h(&self, x: usize, y: usize) -> usize {
        self.heights[self.width * y + x]
    }

    fn is_low(&self, x: usize, y: usize) -> bool {
        let h = self.get_h(x, y);
        if x > 0 { // has left neighbour
            if self.get_h(x - 1, y) <= h {
                return false;
            }
        }
        if x < self.width - 1 { // has right neighbour
            if self.get_h(x + 1, y) <= h {
                return false;
            }
        }
        if y > 0 { // has top neighbout
            if self.get_h(x, y - 1) <= h {
                return false;
            }
        }
        if y < self.height() - 1 { // has bottom neighbour
            if self.get_h(x, y + 1) <= h {
                return false;
            }
        }

        true
    }

    fn fill(&mut self,  x: usize, y: usize, v: usize) {
        self.heights[self.width * y + x] = v;

        if x > 0 { // has left neighbour
            if self.get_h(x - 1, y) < 9 {
                self.fill(x - 1, y, v);
            }
        }
        if x < self.width - 1 { // has right neighbour
            if self.get_h(x + 1, y) < 9 {
                self.fill(x + 1, y, v);
            }
        }
        if y > 0 { // has top neighbout
            if self.get_h(x, y - 1) < 9 {
                self.fill(x, y - 1, v);
            }
        }
        if y < self.height() - 1 { // has bottom neighbour
            if self.get_h(x, y + 1) < 9 {
                self.fill(x, y + 1, v);
            }
        }
    }

    fn cnt(&self, v: usize) -> i32 {
        let mut ret = 0;
        for h in &self.heights {
            if *h == v {
                ret += 1
            }
        }
        ret
    }

}

pub fn part2(filename: &str) -> i32 {
    let strings = crate::util::read_lines(filename);
    let mut b = read_board(strings);

    // fill each basin with different value

    let mut fillv = 10;

    for x in 0..b.width {
        for y in 0..b.height() {
            if b.is_low(x, y) {
                b.fill(x, y, fillv);
                fillv += 1;
            }
        }
    }

    // count sizes of different fill values

    let mut sizes = Vec::new();
    for v in 10..fillv {
        sizes.push(b.cnt(v));
    }

    // multiply the three biggest

    sizes.sort();
    sizes[sizes.len() - 3] * sizes[sizes.len() - 2] * sizes[sizes.len() - 1]
}

