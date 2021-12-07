
pub fn part1(filename: &str) -> i32 {
    let mut strings = crate::util::read_lines(filename);
    
    let ns = crate::util::read_ns(strings.remove(0));

    let mut boards = _read_boards(strings);

    for n in ns {
        for b in boards.iter_mut() {
            b.call(n);
            let res = b.res();
            if res != 0 {
                return res * n;
            }
        }
    }

    0
}

fn _read_boards(strings: Vec<String>) -> Vec<Board> {
    let mut ret = Vec::<Board>::new();
    {
        let mut bns = Vec::<i32>::new();
        for line in strings {
            if line.len() > 0 {
                let mut lns: Vec<i32> =
                    line
                        .split(' ')
                        .filter(|s| !s.is_empty())
                        .map(|n| n.parse::<i32>().unwrap())
                        .collect();
                bns.append(&mut lns);
            } else {
                if _read_board(&mut bns, &mut ret) {
                    bns = Vec::<i32>::new();
                }
            }
        }
        _read_board(&mut bns, &mut ret);
    }

    ret
}

fn _read_board(bns: &Vec<i32>, boards: &mut Vec<Board>) -> bool {
    if (&bns).len() > 0 {
        let size = (bns.len() as f64).sqrt() as usize;
        boards.push(
            Board {
                n: boards.len() as i32,
                size: size,
                numbers: bns.clone(),
                hits: vec![false; size * size]
            });

        return true
    }

    false
}

struct Board {
    n: i32,
    size: usize,
    numbers: Vec<i32>,
    hits: Vec<bool>
}

impl Board {

    fn call(&mut self, n: i32) {
        for i in 0 .. self.size * self.size {
            if self.numbers[i] == n {
                self.hits[i] = true
            }
        }
    }

    fn res(&self) -> i32 {
        if self._win() {
            let mut ret = 0;
            for i in 0 .. self.size * self.size {
                if !self.hits[i] {
                    ret += self.numbers[i]
                }
            }
            ret
        } else {
            0
        }
    }

    fn _hit(&self, w: usize, h: usize) -> bool {
        self.hits[h * self.size + w]
    }

    fn _win(&self) -> bool {
        for i in 0..self.size {
            if self._win_line(i, true) || self._win_line(i, false) {
                return true
            }
        }
        /*
        if self._win_diag(true) || self._win_diag(false) {
            return true
        }
        */
        false
    }

    fn _win_line(&self, x: usize, dir: bool) -> bool {
        for i in 0..self.size {
            let hit = if dir { self._hit(i, x) } else { self._hit(x, i) };
            if !hit {
                return false
            }
        }
        true
    }

    /*
    fn _win_diag(&self, dir: bool) -> bool {
        for h in 0..self.size {
            let v = if dir { self.size - h - 1 } else { h };
            if !self._h(h, v) {
                return false
            }
        }
        true
    }
    */
}


pub fn part2(filename: &str) -> i32 {
    let mut strings = crate::util::read_lines(filename);
    
    let ns = crate::util::read_ns(strings.remove(0));

    let mut boards = _read_boards(strings);

    let mut boards_done = Vec::<i32>::new();

    let bcnt = boards.len();

    for n in ns {
        for b in boards.iter_mut() {
            if boards_done.contains(&b.n) {
                continue;
            }
            b.call(n);
            let res = b.res();
            if res != 0 {
                boards_done.push(b.n);
                if boards_done.len() == bcnt {
                    return res * n;
                }
            }
        }
    }

    0
}
