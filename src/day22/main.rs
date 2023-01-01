#![allow(non_snake_case)]

use core::panic;
use std::{fs, collections::HashMap};

type Pos = [usize;2];

#[derive(Debug,Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn clockwise(&mut self) {
        *self = match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        };
    }

    fn counterclockwise(&mut self) {
        *self = match self {
            Dir::Up => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Right => Dir::Up,
        };
    }
}

#[derive(Debug)]
enum Move {
    Turn(Dir),
    Step(usize),
}

impl Move {
    fn from_str(txt: &str) -> Self {
        match txt {
            "L" => { Move::Turn(Dir::Left) },
            "R" => { Move::Turn(Dir::Right) },
            v => { Move::Step(v.parse::<usize>().unwrap())},
        }
    }
}

#[derive(Debug)]
struct Board {
    leftwrap: HashMap<Pos,(Pos,Dir)>,
    rightwrap: HashMap<Pos,(Pos,Dir)>,
    upwrap: HashMap<Pos,(Pos,Dir)>,
    downwrap: HashMap<Pos,(Pos,Dir)>,
    pos: Pos,
    dir: Dir,
    instr: Vec<Move>,
}

impl Board {
    fn from_str(txt: &str) -> Self {
        let mut b = Board {
            leftwrap: HashMap::new(),
            rightwrap: HashMap::new(),
            upwrap: HashMap::new(),
            downwrap: HashMap::new(),
            pos: [0,0],
            dir: Dir::Right,
            instr: vec![],
        };

        b.instr = txt.lines().last().unwrap()
            .split_inclusive(['L','R']).map(|s|
                if s.ends_with(['L', 'R']) { s.split_at(s.len()-1) } else { (s,"") }
            )
            .map(|(a,b)| [a,b])
            .flatten()
            .filter(|s| s.len() > 0)
            .map(|v| Move::from_str(v))
            .collect();

        let maplines: Vec<&str> = txt.lines().take_while(|l| l.len() > 0).collect();

        let Nx = maplines.iter()
            .map(|l| l.len()).max().unwrap();
        let Ny = maplines.iter()
            .count();

        // let chars: Vec<char> = Vec::with_capacity(Nx*Ny);
        let chars: Vec<Vec<char>> = maplines.iter()
            .map(|l| l.chars().collect()).collect();

        for (i,line) in maplines.iter().enumerate() {
            let jstart = line.find(|c: char| !c.is_whitespace()).unwrap();
            let jend = line.rfind(|c: char| !c.is_whitespace()).unwrap();
            let jlen = jend + 1 - jstart;

            if i == 0 {
                b.pos = [i,jstart];
            }

            for (j,c) in line.chars().enumerate().skip(jstart) {
                let istart = chars.iter().map(|l| l.get(j).unwrap_or(&' '))
                    .position(|c| !c.is_whitespace()).unwrap();
                let iend   = chars.iter().map(|l| l.get(j).unwrap_or(&' '))
                    .rposition(|c| !c.is_whitespace()).unwrap();
                let ilen = iend + 1 - istart;

                let p: Pos = [i,j];
                match c {
                    '.' => {
                        let jwrap = if j == jstart { jend } else { j - 1 };

                        if chars[i][jwrap] == '#' {
                            b.leftwrap.insert(p, ([i,j], Dir::Left));
                        }
                        else if jwrap >= j {
                            b.leftwrap.insert(p, ([i,jwrap], Dir::Left));
                        }

                        let jwrap = if j == jend { jstart } else { j + 1 };

                        if chars[i][jwrap] == '#' {
                            b.rightwrap.insert(p, ([i,j], Dir::Right));
                        }
                        else if jwrap <= j {
                            b.rightwrap.insert(p, ([i,jwrap], Dir::Right));
                        }

                        // 

                        let iwrap = if i == istart { iend } else { i - 1 };

                        if chars[iwrap][j] == '#' {
                            b.upwrap.insert(p, ([i,j], Dir::Up));
                        }
                        else if iwrap >= i {
                            b.upwrap.insert(p, ([iwrap,j], Dir::Up));
                        }

                        let iwrap = if i == iend { istart } else { i + 1 };

                        if chars[iwrap][j] == '#' {
                            b.downwrap.insert(p, ([i,j], Dir::Down));
                        }
                        else if iwrap <= i {
                            b.downwrap.insert(p, ([iwrap,j], Dir::Down));
                        }
                    }

                    '#' => { },
                    _ => { panic!("unknown char"); }
                }
            }
        }

        b
    }

    fn from_str_p2(txt: &str, for_test: bool) -> Self {
        let mut b = Board {
            leftwrap: HashMap::new(),
            rightwrap: HashMap::new(),
            upwrap: HashMap::new(),
            downwrap: HashMap::new(),
            pos: [0,0],
            dir: Dir::Right,
            instr: vec![],
        };

        b.instr = txt.lines().last().unwrap()
            .split_inclusive(['L','R']).map(|s|
                if s.ends_with(['L', 'R']) { s.split_at(s.len()-1) } else { (s,"") }
            )
            .map(|(a,b)| [a,b])
            .flatten()
            .filter(|s| s.len() > 0)
            .map(|v| Move::from_str(v))
            .collect();

        let maplines: Vec<&str> = txt.lines().take_while(|l| l.len() > 0).collect();

        let Nx = maplines.iter()
            .map(|l| l.len()).max().unwrap();
        let Ny = maplines.iter()
            .count();

        // let chars: Vec<char> = Vec::with_capacity(Nx*Ny);
        let chars: Vec<Vec<char>> = maplines.iter()
            .map(|l| l.chars().collect()).collect();

        let P = ((chars.iter().map(|l|
            l.iter().filter(|c| !c.is_whitespace()).count())
            .sum::<usize>() / 6) as f64).sqrt() as usize;
        println!("panel size {}", P);

        for (i,line) in maplines.iter().enumerate() {
            let jstart = line.find(|c: char| !c.is_whitespace()).unwrap();
            let jend = line.rfind(|c: char| !c.is_whitespace()).unwrap();
            let jlen = jend + 1 - jstart;

            let di = i % P;

            if i == 0 {
                b.pos = [i,jstart];
            }

            for (j,c) in line.chars().enumerate().skip(jstart) {
                let istart = chars.iter().map(|l| l.get(j).unwrap_or(&' '))
                    .position(|c| !c.is_whitespace()).unwrap();
                let iend   = chars.iter().map(|l| l.get(j).unwrap_or(&' '))
                    .rposition(|c| !c.is_whitespace()).unwrap();
                let ilen = iend + 1 - istart;

                let dj = j % P;

                let p: Pos = [i,j];
                match c {
                    '.' => {
                        if !for_test {
                            // left edges
                            let wrap = if j == jstart {
                                if i < P {
                                    ((3*P - 1 - di, 0), Dir::Right)
                                } else if i < 2*P {
                                    ((2*P, di), Dir::Down)
                                } else if i < 3*P{
                                    ((P - 1 - di, P), Dir::Right)
                                } else {
                                    ((0, P + di), Dir::Down)
                                }
                            } else {
                                ((i,j - 1), Dir::Left)
                            };

                            if chars[wrap.0.0][wrap.0.1] == '#' {
                                b.leftwrap.insert(p, ([i,j], Dir::Left));
                            }
                            else {
                                b.leftwrap.insert(p, ([wrap.0.0,wrap.0.1], wrap.1));
                            }

                            // right edges
                            let wrap = if j == jend {
                                if i < P {
                                    ((3*P - 1 - di, 2*P - 1), Dir::Left)
                                } else if i < 2*P {
                                    ((P - 1, 2*P + di), Dir::Up)
                                } else if i < 3*P {
                                    ((P - 1 - di, 3*P - 1), Dir::Left)
                                } else {
                                    ((3*P - 1, P + di), Dir::Up)
                                }
                            } else {
                                ((i,j + 1), Dir::Right)
                            };

                            if chars[wrap.0.0][wrap.0.1] == '#' {
                                b.rightwrap.insert(p, ([i,j], Dir::Right));
                            }
                            else {
                                b.rightwrap.insert(p, ([wrap.0.0,wrap.0.1], wrap.1));
                            }

                            // up wrap
                            let wrap = if i == istart {
                                if j < P {
                                    ((P + dj, P), Dir::Right)
                                } else if j < 2*P {
                                    ((3*P + dj, 0), Dir::Right)
                                } else {
                                    ((4*P - 1, dj), Dir::Up)
                                }
                            } else {
                                ((i - 1, j), Dir::Up)
                            };

                            if chars[wrap.0.0][wrap.0.1] == '#' {
                                b.upwrap.insert(p, ([i,j], Dir::Up));
                            }
                            else {
                                b.upwrap.insert(p, ([wrap.0.0,wrap.0.1], wrap.1));
                            }

                            // down wrap
                            let wrap = if i == iend {
                                if j < P {
                                    ((0, 2*P + dj), Dir::Down)
                                } else if j < 2*P {
                                    ((3*P + dj, P - 1), Dir::Left)
                                } else {
                                    ((P + dj, 2*P - 1), Dir::Left)
                                }
                            } else {
                                ((i + 1, j), Dir::Down)
                            };

                            if chars[wrap.0.0][wrap.0.1] == '#' {
                                b.downwrap.insert(p, ([i,j], Dir::Down));
                            }
                            else {
                                b.downwrap.insert(p, ([wrap.0.0,wrap.0.1], wrap.1));
                            }

                        } else {
                            // left edges
                            let wrap = if j == jstart {
                                if i < P {
                                    ((P, P + i), Dir::Down)
                                } else if i < 2*P {
                                    ((3*P - 1, 3*P + i - P), Dir::Up)
                                } else {
                                    ((2*P - 1, 2*P - 1 - (i - 2*P)), Dir::Up)
                                }
                            } else {
                                ((i,j - 1), Dir::Left)
                            };

                            if chars[wrap.0.0][wrap.0.1] == '#' {
                                b.leftwrap.insert(p, ([i,j], Dir::Left));
                            }
                            else {
                                b.leftwrap.insert(p, ([wrap.0.0,wrap.0.1], wrap.1));
                            }

                            // right edges
                            let wrap = if j == jend {
                                if i < P {
                                    ((3*P - 1 - i, 4*P - 1), Dir::Left)
                                } else if i < 2*P {
                                    ((2*P, 4*P - 1 - (i - P)), Dir::Down)
                                } else {
                                    ((P - 1 - (i - 2*P), 3*P - 1), Dir::Left)
                                }
                            } else {
                                ((i,j + 1), Dir::Right)
                            };

                            if chars[wrap.0.0][wrap.0.1] == '#' {
                                b.rightwrap.insert(p, ([i,j], Dir::Right));
                            }
                            else {
                                b.rightwrap.insert(p, ([wrap.0.0,wrap.0.1], wrap.1));
                            }

                            // up wrap
                            let wrap = if i == istart {
                                if j < P {
                                    ((0, 3*P - 1 - j), Dir::Down)
                                } else if j < 2*P {
                                    ((j - P, 2*P), Dir::Right)
                                } else if j < 3*P {
                                    ((P, P - 1 - (j - 2*P)), Dir::Down)
                                } else {
                                    ((2*P - 1 - (j - 3*P), 3*P - 1), Dir::Left)
                                }
                            } else {
                                ((i - 1, j), Dir::Up)
                            };

                            if chars[wrap.0.0][wrap.0.1] == '#' {
                                b.upwrap.insert(p, ([i,j], Dir::Up));
                            }
                            else {
                                b.upwrap.insert(p, ([wrap.0.0,wrap.0.1], wrap.1));
                            }

                            // down wrap
                            let wrap = if i == iend {
                                if j < P {
                                    ((3*P - 1, 3*P - 1 - j), Dir::Up)
                                } else if j < 2*P {
                                    ((3*P - 1 - (j - P), 2*P), Dir::Right)
                                } else if j < 3*P {
                                    ((2*P - 1, P - 1 - (j - 2*P)), Dir::Up)
                                } else {
                                    ((2*P - 1 - (j - 3*P), 0), Dir::Right)
                                }
                            } else {
                                ((i + 1, j), Dir::Down)
                            };

                            if chars[wrap.0.0][wrap.0.1] == '#' {
                                b.downwrap.insert(p, ([i,j], Dir::Down));
                            }
                            else {
                                b.downwrap.insert(p, ([wrap.0.0,wrap.0.1], wrap.1));
                            }
                        }
                    } 
                    '#' => { },
                    _ => { panic!("unknown char"); }
                }
            }
        }

        b
    }

    fn score(&self) -> usize {
        1000*(self.pos[0] + 1) + 4*(self.pos[1] + 1) + 
            match self.dir {
                Dir::Up => 3,
                Dir::Down => 1,
                Dir::Left => 2,
                Dir::Right => 0,
            }
    }

    fn execute(&mut self) {
        for instr in self.instr.iter() {
            match instr {
                Move::Turn(dir) => {
                    match dir {
                        Dir::Right => self.dir.clockwise(),
                        Dir::Left => self.dir.counterclockwise(),
                        _ => panic!("bad turn"),
                    }
                },
                Move::Step(step) => {
                    for i in 0..*step {
                        let posdir = match self.dir {
                            Dir::Up => {
                                *self.upwrap.get(&self.pos)
                                    .unwrap_or(&([self.pos[0]-1,self.pos[1]],self.dir))
                            },
                            Dir::Right => {
                                *self.rightwrap.get(&self.pos)
                                    .unwrap_or(&([self.pos[0],self.pos[1]+1],self.dir))
                            },
                            Dir::Down => {
                                *self.downwrap.get(&self.pos)
                                    .unwrap_or(&([self.pos[0]+1,self.pos[1]],self.dir))
                            },
                            Dir::Left => {
                                *self.leftwrap.get(&self.pos)
                                    .unwrap_or(&([self.pos[0],self.pos[1]-1],self.dir))
                                    // .unwrap()
                            },
                        };
                        self.pos = posdir.0;
                        self.dir = posdir.1;
                    }
                },
            }
        }
    }
}

fn part1(txt: &str) -> usize {
    let mut board = Board::from_str(txt);
    // println!("{:?}", board);
    board.execute();
    
    board.score()
}

fn part2(txt: &str, for_test: bool) -> usize {
    let mut board = Board::from_str_p2(txt, for_test);
    // println!("{:?}", board);
    board.execute();
    
    board.score()
}

fn main() {
    let dayX = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    let path = String::from(root) + "/src/" + dayX + "/input.txt";
    // let path = String::from(root) + "/src/" + dayX + "/test1.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", dayX);
    // println!("Part 1: {:?}", part1(&txt));
    println!("Part 2: {:?}", part2(&txt, false));
    // println!("Part 2: {:?}", part2(&txt, true));
}
