pub fn day02_1(filename: &str) -> i32 {
    let cmnds = to_cmds(crate::util::read_lines(filename));
    let mut h = 0;
    let mut v = 0;
    for cmd in cmnds {
        match cmd.dir {
            Dir::F => { h = h + cmd.n },
            Dir::B => { h = h - cmd.n },
            Dir::U => { v = v - cmd.n },
            Dir::D => { v = v + cmd.n }
        }
    }
    h * v
}

enum Dir { F, B, U, D }

impl std::str::FromStr for Dir {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Dir::F),
            "backward" => Ok(Dir::B),
            "up" => Ok(Dir::U),
            "down" => Ok(Dir::D),
            _ => Err(format!("'{}' invalid Dir", s))
        }
    }
}

struct Cmd { dir: Dir, n: i32 }

fn to_cmds(strings: Vec<String>) -> Vec<Cmd> {
    let mut ret = Vec::new();
    for s in strings {
        let cmd: Vec<&str> = s.split(' ').collect();
        ret.push(Cmd {
            dir: cmd[0].parse().unwrap(),
            n: cmd[1].parse().unwrap()
        });
    }
    ret
}

pub fn day02_2(filename: &str) -> i32 {
    let cmnds = to_cmds(crate::util::read_lines(filename));
    let mut h = 0;
    let mut v = 0;
    let mut dspeed = 0; // aim
    for cmd in cmnds {
        match cmd.dir {
            Dir::F => { h = h + cmd.n; v = v + cmd.n * dspeed; },
            Dir::B => { h = h - cmd.n; v = v + cmd.n * dspeed; },
            Dir::U => { dspeed = dspeed - cmd.n },
            Dir::D => { dspeed = dspeed + cmd.n }
        }
    }
    h * v
}
