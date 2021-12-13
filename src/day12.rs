
pub fn main() {
    crate::util::test(1, 226, part1("day12_test"));
    crate::util::do_part(1, part1, "day12");
    crate::util::test(2, 36, part2("day12_test_s"));
    crate::util::test(2, 103, part2("day12_test_m"));
    crate::util::test(2, 3509, part2("day12_test"));
    crate::util::do_part(2, part2, "day12");
}

pub fn part1(filename: &str) -> usize {
    compute(filename, false).len()
}

pub fn part2(filename: &str) -> usize {
    let paths = compute(filename, true);

    let ret = paths.len();

    if ret == 0 { // was used for debugging when test returned wrong result
        for p in paths {
            println!("{}", p);
        }
    }

    ret
}

fn compute(filename: &str, for_part2: bool) -> Vec<Path> {
    let strings = crate::util::read_lines(filename);
    let cs = read_cs(strings);

    let mut paths = Vec::new();
    
    let start = Path { cave: "start".to_owned(), small_caves_visited: Vec::new() };
    cs_go(&mut paths, &cs, start, for_part2);

    paths
}

fn cs_go(paths: &mut Vec<Path>, cs: &CaveSystem, mut path: Path, for_part2: bool) {
    
    let cave = path.cave.clone();

    if is_small(&cave) {
        path.small_caves_visited.push(cave.clone());
    }

    if &cave == "end" {
        paths.push(path);
    } else {

        let mut all_next_caves = Vec::new();
        for t in &cs.tunnels {
            if t.cave1 == cave {
                all_next_caves.push(t.cave2.to_owned())
            };
            if t.cave2 == cave {
                all_next_caves.push(t.cave1.to_owned())
            }
        }
        
        let mut next_caves = Vec::new();
        for nc in all_next_caves {
            let mut add = false;
            if !is_small(&nc) { // big caves can be visited without restriction
                add = true
            } else if path.small_caves_visited.iter().find(|&v| *v == nc) == None { // small cave never visited before
                add = true
            } else if for_part2 {
                if nc == "start" { // start never twice
                    continue
                }
                let ncs: usize =
                    path.small_caves_visited.iter()
                    .filter(|&c| c == &nc)
                    .map(|_| 1).sum();
                if ncs == 2 { // this small cave has been visited twice
                    continue
                }

                // get all other small visited
                let mut others: Vec<String> =
                    path.small_caves_visited.iter()
                    .filter(|&c| c != &nc)
                    .map(|s| s.to_owned()).collect();
                let olen1 = others.len();

                // remove duplicates
                others.sort();
                others.dedup();
                let olen2 = others.len();

                if olen2 == olen1 { // if there were no other small caves visited twice 
                    add = true
                }
            }
            if add {
                next_caves.push(nc)
            }
        }

        for n in next_caves {
            cs_go(paths, &cs,
                Path { cave: n,
                    small_caves_visited: path.small_caves_visited.clone() }, for_part2);
        }
    }
}

fn read_cs(strings: Vec<String>) -> CaveSystem {
    let mut tunnels = Vec::new();
    for string in strings {
        tunnels.push(read_tunnel(string))
    }
    CaveSystem { tunnels: tunnels }
}

fn read_tunnel(string: String) -> Tunnel {
    // start-A
    let s: Vec<String> = string.split('-').map(|s| s.to_owned()).collect();
    Tunnel { cave1: s[0].clone(), cave2: s[1].clone() }
}

fn is_small(string: &String) -> bool {
    string.to_uppercase() != *string
}

struct CaveSystem {
    tunnels: Vec<Tunnel>
}

struct Tunnel {
    cave1: String,
    cave2: String
}

struct Path {
    cave: String,
    small_caves_visited: Vec<String>
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = "[".to_owned();
        for v in self.small_caves_visited.iter() {
            let vs = format!(" {}", v);
            s.push_str(vs.as_str());
        }
        s.push_str(" ]");
        write!(f, "{}", s)
    }
}
