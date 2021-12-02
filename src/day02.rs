pub fn day02_1(filename: &str) -> i32 {
    let cmnds = to_cmds(crate::util::read_lines(filename));
    let mut h = 0;
    let mut v = 0;
    for cmd in cmnds {
        match cmd.d {
            Dir::F => { h = h + cmd.n },
            Dir::B => { h = h - cmd.n },
            Dir::U => { v = v - cmd.n },
            Dir::D => { v = v + cmd.n },
            _ => {} // won't happen
        }
    }
    h * v
}

enum Dir { F, B, U, D, ERR }
struct Cmd { d: Dir, n: i32 }

fn to_cmds(strings: Vec<String>) -> Vec<Cmd> {
    let mut ret = Vec::new();
    for s in strings {
        let cmd: Vec<&str> = s.split(' ').collect();
        let d: Dir = match cmd[0] {
            "forward" => Dir::F,
            "backward" => Dir::B,
            "up" => Dir::U,
            "down" => Dir::D,
            _ => Dir::ERR
        };
        let n: i32 = cmd[1].parse().unwrap();
        ret.push(Cmd { d: d, n: n });
    }
    ret
}

pub fn day02_2(filename: &str) -> i32 {
    let cmnds = to_cmds(crate::util::read_lines(filename));
    let mut h = 0;
    let mut v = 0;
    let mut dspeed = 0; // aim
    for cmd in cmnds {
        match cmd.d {
            Dir::F => { h = h + cmd.n; v = v + cmd.n * dspeed; },
            Dir::B => { h = h - cmd.n; v = v + cmd.n * dspeed; },
            Dir::U => { dspeed = dspeed - cmd.n },
            Dir::D => { dspeed = dspeed + cmd.n },
            _ => {} // won't happen
        }
    }
    h * v
}