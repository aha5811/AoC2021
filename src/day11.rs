
pub fn main() {
    crate::util::test(1, 1656, part1("day11_test"));
    crate::util::do_part(1, part1, "day11");
    crate::util::test(2, 195, part2("day11_test"));
    crate::util::do_part(2, part2, "day11");
}

pub fn part1(filename: &str) -> usize {
    let strings = crate::util::read_lines(filename);
    let mut b = read_board(strings);

    for _ in 0..100 {
        b.step();
    }

    b.flash_cnt
}

// see day09

fn read_board(strings: Vec<String>) -> Board {
    let mut w = 0;
    let mut es = Vec::<usize>::new();
    for string in strings {
        let mut ns = read_ns(string);
        w = ns.len();
        es.append(&mut ns);
    }

    Board { width: w, height: es.len() / w, energys: es, flash_cnt: 0 }
}

fn read_ns(string: String) -> Vec<usize> {
    string.chars().map(|c| (c as i32 - 0x30) as usize).collect()
}

struct Board {
    width: usize,
    height: usize,
    energys: Vec<usize>,
    flash_cnt: usize
}

impl Board {

    fn get_e(&self, x: usize, y: usize) -> usize {
        self.energys[self.xy_to_p((x, y))]
    }

    fn incr(&mut self, x: usize, y: usize) {
        let p = self.xy_to_p((x, y));
        self.energys[p] += 1
    }

    fn step(&mut self) {

        let mut flashers = Vec::<usize>::new();

        // increase all
        for i in 0..self.energys.len() {
            self.energys[i] += 1;
            if self.energys[i] == 10 {
                flashers.push(i)
            }
        }

        while flashers.len() > 0 {
            let p = flashers.pop().unwrap() as usize;
            flashers.append(&mut self.flash(p))
        }

        // set 10 to 0
        for i in 0..self.energys.len() {
            let (x, y) = self.p_to_xy(i);
            if self.get_e(x, y) > 9 {
                self.energys[i] = 0
            }
        }

    }

    fn p_to_xy(&self, p: usize) -> (usize, usize) {
        let x = p % self.width;
        let y = (p - x) / self.width;
        (x, y)
    }

    fn xy_to_p(&self, (x, y): (usize, usize)) -> usize {
        self.width * y + x
    }

    fn flash(&mut self, p: usize) -> Vec<usize> {
        self.flash_cnt += 1;

        let mut ret = Vec::new();

        let mut affected = Vec::new();
        {
            let (x, y) = self.p_to_xy(p);

            let has_left = x > 0;
            let has_right = x < self.width - 1;
            let has_top = y > 0;
            let has_bottom = y < self.height - 1;

            if has_left {
                affected.push((x - 1, y))
            }
            if has_right {
                affected.push((x + 1, y))
            }
            if has_top { 
                affected.push((x, y - 1));
            }
            if has_bottom {
                affected.push((x, y + 1));
            }
            if has_top && has_left {
                affected.push((x - 1, y - 1))
            }
            if has_top && has_right {
                affected.push((x + 1, y - 1))
            }
            if has_bottom && has_left {
                affected.push((x - 1, y + 1))
            }
            if has_bottom && has_right {
                affected.push((x + 1, y + 1))
            }
        }

        for (x, y) in affected {
            self.incr(x, y);
            if self.get_e(x, y) == 10 {
                ret.push(self.xy_to_p((x, y)))
            }
        }

        ret
    }
}

pub fn part2(filename: &str) -> usize {
    let strings = crate::util::read_lines(filename);
    let mut b = read_board(strings);

    let mut step = 1;
    let mut prev_flash_cnt = 0;
    loop {
        b.step();
        let flash_cnt = b.flash_cnt;
        if flash_cnt - prev_flash_cnt == b.width * b.height {
            return step
        }
        prev_flash_cnt = flash_cnt;
        step += 1
    }
}
