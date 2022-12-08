use std::fs;

fn part1(txt: &str) -> usize {
    const MAX_SIZE: usize = 100000;
    let mut res = 0;

    let mut stack: Vec<usize> = Vec::new();

    for line in txt.lines() {
        let mut split = line.split_whitespace();
        match split.next().unwrap() {
            "$" => {
                match split.next().unwrap() {
                    "cd" => {
                        let target = split.next().unwrap();
                        match target {
                            ".." => {
                                let s = stack.pop().unwrap();
                                if s <= MAX_SIZE {
                                    res += s;
                                }
                            },
                            _ => {
                                stack.push(0);
                            },
                        }
                    }
                    _ => (),
                }
            }
            info => {
                match info {
                    "dir" => (),
                    sz => {
                        for d in stack.iter_mut() {
                            *d += sz.parse::<usize>().unwrap();
                        }
                    }
                }
            }
        }
    }

    for s in stack.iter() {
        if *s <= MAX_SIZE {
            res += s;
        }
    }

    println!("{:?}", stack[0]);

    res
}

fn part2(txt: &str) -> usize {
    const USED: usize = 46552309;  // input
    // const USED: usize = 48381165;  // test1
    const MINFREE: usize = USED - 40000000;

    let mut smallest = USED;

    let mut stack: Vec<usize> = Vec::new();

    for line in txt.lines() {
        let mut split = line.split_whitespace();
        match split.next().unwrap() {
            "$" => {
                match split.next().unwrap() {
                    "cd" => {
                        let target = split.next().unwrap();
                        match target {
                            ".." => {
                                let s = stack.pop().unwrap();
                                if s < smallest && s >= MINFREE {
                                    smallest = s;
                                }
                            },
                            _ => {
                                stack.push(0);
                            },
                        }
                    }
                    _ => (),
                }
            }
            info => {
                match info {
                    "dir" => (),
                    sz => {
                        for d in stack.iter_mut() {
                            *d += sz.parse::<usize>().unwrap();
                        }
                    }
                }
            }
        }
    }

    for &s in stack.iter() {
        if s < smallest && s >= MINFREE {
            smallest = s;
        }
    }

    smallest
}

#[allow(non_snake_case)]
fn main() {
    let dayX = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    let path = String::from(root) + "/src/" + dayX + "/input.txt";
    // let path = String::from(root) + "/src/" + dayX + "/test1.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", dayX);
    println!("Part 1: {}", part1(&txt));
    println!("Part 2: {}", part2(&txt));
}
