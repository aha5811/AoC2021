
pub fn main() {
    crate::util::test(1, 15, part1("day09_test"));
    crate::util::do_part(1, part1, "day09");
    crate::util::test(2, 1134, part2("day09_test"));
    crate::util::do_part(2, part2, "day09");
}

pub fn part1(filename: &str) -> usize {
    let strings = crate::util::read_lines(filename);
    let b = read_board(strings);

    let mut ret = 0;

    for x in 0..b.width {
        for y in 0..b.height() {
            if b.is_low(x, y, false) {
                ret += b.get_h(x, y) + 1
            }
        }
    }

    ret
}

fn read_board(strings: Vec<String>) -> Board {
    let mut w = 0;
    let mut hs = Vec::<usize>::new();
    for string in strings {
        let mut ns = read_ns(string);
        w = ns.len();
        hs.append(&mut ns);
    }

    Board { width: w, heights: hs,
        curr_fill: 0,
        curr_fill_cnt: 0,
        fill_sizes: Vec::new()
     }
}

fn read_ns(string: String) -> Vec<usize> {
    string.chars().map(|c| (c as i32 - 0x30) as usize).collect()
}

struct Board {
    width: usize,
    heights: Vec<usize>,

    // for part2 speedup
    curr_fill: usize,
    curr_fill_cnt: usize,
    fill_sizes: Vec<usize>
}

impl Board {

    fn height(&self) -> usize {
        self.heights.len() / self.width
    }

    fn get_h(&self, x: usize, y: usize) -> usize {
        self.heights[self.width * y + x]
    }

    fn is_low(&self, x: usize, y: usize, for_part2: bool) -> bool {
        let h = self.get_h(x, y);
        
        if x > 0 { // has left neighbour
            let nh = self.get_h(x - 1, y);
            if nh <= h {
                return false;
            } else if for_part2 && nh <= 9 {
                return true;
            }
        }

        if x < self.width - 1 { // has right neighbour
            let nh = self.get_h(x + 1, y);
            if nh <= h {
                return false;
            } else if for_part2 && nh <= 9 {
                return true;
            }
        }

        if y > 0 { // has top neighbour
            let nh = self.get_h(x, y - 1);
            if nh <= h {
                return false;
            } else if for_part2 && nh <= 9 {
                return true;
            }
        }

        if y < self.height() - 1 { // has bottom neighbour
            let nh = self.get_h(x, y + 1);
            if nh <= h {
                return false;
            } else if for_part2 && nh <= 9 {
                return true;
            }
        }

        true
    }

    fn fill(&mut self,  x: usize, y: usize, fill: usize) {
        
        // we assume (correctly) that no previously used v is used again
        if fill != self.curr_fill { // fill changes
            self.fill_sizes.push(self.curr_fill_cnt);
            self.curr_fill_cnt = 0;
            self.curr_fill = fill;
        }

        self.heights[self.width * y + x] = fill;

        self.curr_fill_cnt += 1;

        if x > 0 { // has left neighbour
            if self.get_h(x - 1, y) < 9 {
                self.fill(x - 1, y, fill);
            }
        }
        if x < self.width - 1 { // has right neighbour
            if self.get_h(x + 1, y) < 9 {
                self.fill(x + 1, y, fill);
            }
        }
        if y > 0 { // has top neighbour
            if self.get_h(x, y - 1) < 9 {
                self.fill(x, y - 1, fill);
            }
        }
        if y < self.height() - 1 { // has bottom neighbour
            if self.get_h(x, y + 1) < 9 {
                self.fill(x, y + 1, fill);
            }
        }
    }

    /*
    // pre part2 speedup
    fn cnt(&self, v: usize) -> i32 {
        let mut ret = 0;
        for h in &self.heights {
            if *h == v {
                ret += 1
            }
        }
        ret
    }
    */
}

pub fn part2(filename: &str) -> usize {
    let strings = crate::util::read_lines(filename);
    let mut b = read_board(strings);

    // fill each basin with different valued 'color'
    // starting with 10 so all colors work as 9-boundaries

    let mut fill_value = 10;

    for x in 0..b.width {
        for y in 0..b.height() {
            if b.is_low(x, y, true) { // any low point is also inside a basin
                b.fill(x, y, fill_value);
                fill_value += 1;
            }
        }
    }

    // count sizes of different fill values
    
    /*
    // slow
    let mut sizes = Vec::new();
    for v in 10..fillv {
        sizes.push(b.cnt(v));
    }
    */

    // faster
    let mut sizes = b.fill_sizes;

    // multiply the three biggest

    sizes.sort();
    sizes.pop().unwrap() * sizes.pop().unwrap() * sizes.pop().unwrap()
}
