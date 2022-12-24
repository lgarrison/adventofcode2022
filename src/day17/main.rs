#![allow(non_snake_case)]

use core::panic;
use std::{fs, cmp::max};

// const HMAX: usize = (11000*5/5 + 1)*13;
const HMAX: usize = 10000000000;
const W: usize = 7;

#[derive(Debug,PartialEq,Clone,Copy)]
enum Dir {
    Left,
    Right,
    Down,
}

#[derive(Debug)]
struct Board {
    board: Vec<bool>,
    jets: Vec<Dir>,
    nextjet: usize,
    next: usize,
    height: usize,
}

trait Moveable {
    fn try_move(&mut self, board: &Board, dir: &Dir) -> bool;
    fn top(&self) -> usize;
    fn mark_board(&self, board: &mut Board);
    fn getx(&self) -> usize;
}

#[derive(Debug)]
struct HLine {
    pos: [usize;2],
}
#[derive(Debug)]
struct Plus {
    pos: [usize;2],
}
#[derive(Debug)]
struct VLine {
    pos: [usize;2],
}
#[derive(Debug)]
struct Ell {
    pos: [usize;2],
}
#[derive(Debug)]
struct Square {
    pos: [usize;2],
}

impl Moveable for HLine {
    fn try_move(&mut self, board: &Board, dir: &Dir) -> bool {
        match dir {
            Dir::Left => {
                if self.pos[0] > 0 &&
                    !board.at(self.pos[0]-1, self.pos[1])
                {
                    self.pos[0] -= 1;
                    // println!("new pos: {:#?}", self.pos);
                    return true;
                }
            },
            Dir::Right => {
                if self.pos[0] + 3 < W-1 &&
                    !board.at(self.pos[0]+4, self.pos[1])
                {
                    self.pos[0] += 1;
                    // println!("new pos: {:#?}", self.pos);
                    return true;
                }
            },
            Dir::Down => {
                if self.pos[1] > 0 &&
                    (self.pos[0]..=self.pos[0]+3).all(|i|
                    !board.at(i, self.pos[1]-1))
                {
                    self.pos[1] -= 1;
                    // println!("new pos: {:#?}", self.pos);
                    return true;
                }
            },
        };
        // println!("failed move {:?}; new pos: {:#?}", dir, self.pos);
        return false;
    }

    fn top(&self) -> usize { self.pos[1] }

    fn mark_board(&self, board: &mut Board) {
        // println!("marking at {:?}", self);
        for i in self.pos[0]..=self.pos[0]+3 {
            *board.at_mut(i, self.pos[1]) = true;
        }
    }

    fn getx(&self) -> usize { self.pos[0] }
}

impl Moveable for Plus {
    fn try_move(&mut self, board: &Board, dir: &Dir) -> bool {
        match dir {
            Dir::Left => {
                if self.pos[0] > 0 &&
                    !board.at(self.pos[0], self.pos[1]) &&
                    !board.at(self.pos[0]-1, self.pos[1]-1) &&
                    !board.at(self.pos[0], self.pos[1]-2)
                {
                    self.pos[0] -= 1;
                    return true;
                }
            },
            Dir::Right => {
                if self.pos[0] + 2 < W-1 &&
                    !board.at(self.pos[0]+2, self.pos[1]) &&
                    !board.at(self.pos[0]+3, self.pos[1]-1) &&
                    !board.at(self.pos[0]+2, self.pos[1]-2)
                {
                    self.pos[0] += 1;
                    return true;
                }
            },
            Dir::Down => {
                if self.pos[1] - 2 > 0 &&
                    !board.at(self.pos[0], self.pos[1]-2) &&
                    !board.at(self.pos[0]+1, self.pos[1]-3) &&
                    !board.at(self.pos[0]+2, self.pos[1]-2)
                {
                    self.pos[1] -= 1;
                    return true;
                }
            },
        };
        return false;
    }

    fn top(&self) -> usize { self.pos[1] }

    fn mark_board(&self, board: &mut Board) {
        *board.at_mut(self.pos[0] + 1, self.pos[1]) = true;
        *board.at_mut(self.pos[0], self.pos[1] - 1) = true;
        *board.at_mut(self.pos[0] + 1, self.pos[1] - 1) = true;
        *board.at_mut(self.pos[0] + 2, self.pos[1] - 1) = true;
        *board.at_mut(self.pos[0] + 1, self.pos[1] - 2) = true;
    }

    fn getx(&self) -> usize { self.pos[0] }
}

impl Moveable for Ell {
    fn try_move(&mut self, board: &Board, dir: &Dir) -> bool {
        match dir {
            Dir::Left => {
                if self.pos[0] > 0 &&
                    !board.at(self.pos[0]-1, self.pos[1]-2)
                {
                    self.pos[0] -= 1;
                    return true;
                }
            },
            Dir::Right => {
                if self.pos[0] + 2 < W-1 &&
                    !board.at(self.pos[0]+3, self.pos[1]) &&
                    !board.at(self.pos[0]+3, self.pos[1]-1) &&
                    !board.at(self.pos[0]+3, self.pos[1]-2)
                {
                    self.pos[0] += 1;
                    return true;
                }
            },
            Dir::Down => {
                if self.pos[1] - 2 > 0 &&
                    !board.at(self.pos[0], self.pos[1]-3) &&
                    !board.at(self.pos[0]+1, self.pos[1]-3) &&
                    !board.at(self.pos[0]+2, self.pos[1]-3)
                {
                    self.pos[1] -= 1;
                    return true;
                }
            },
        };
        return false;
    }

    fn top(&self) -> usize { self.pos[1] }

    fn mark_board(&self, board: &mut Board) {
        *board.at_mut(self.pos[0] + 2, self.pos[1]) = true;
        *board.at_mut(self.pos[0] + 2, self.pos[1]-1) = true;
        *board.at_mut(self.pos[0] + 2, self.pos[1]-2) = true;
        *board.at_mut(self.pos[0] + 1, self.pos[1]-2) = true;
        *board.at_mut(self.pos[0], self.pos[1]-2) = true;
    }

    fn getx(&self) -> usize { self.pos[0] }
}

impl Moveable for VLine {
    fn try_move(&mut self, board: &Board, dir: &Dir) -> bool {
        match dir {
            Dir::Left => {
                if self.pos[0] > 0 &&
                    (self.pos[1]-3..=self.pos[1]).all(|j|
                        !board.at(self.pos[0]-1, j)
                    )
                {
                    self.pos[0] -= 1;
                    return true;
                }
            },
            Dir::Right => {
                if self.pos[0] < W-1 &&
                    (self.pos[1]-3..=self.pos[1]).all(|j|
                        !board.at(self.pos[0]+1, j)
                    )
                {
                    self.pos[0] += 1;
                    return true;
                }
            },
            Dir::Down => {
                if self.pos[1] - 3 > 0 &&
                    !board.at(self.pos[0], self.pos[1]-4)
                {
                    self.pos[1] -= 1;
                    return true;
                }
            },
        };
        return false;
    }

    fn top(&self) -> usize { self.pos[1] }
    
    fn mark_board(&self, board: &mut Board) {
        for j in self.pos[1]-3..=self.pos[1] {
            *board.at_mut(self.pos[0], j) = true;
        }
    }

    fn getx(&self) -> usize { self.pos[0] }
}

impl Moveable for Square {
    fn try_move(&mut self, board: &Board, dir: &Dir) -> bool {
        match dir {
            Dir::Left => {
                if self.pos[0] > 0 &&
                    !board.at(self.pos[0]-1, self.pos[1]) &&
                    !board.at(self.pos[0]-1, self.pos[1]-1)
                {
                    self.pos[0] -= 1;
                    return true;
                }
            },
            Dir::Right => {
                if self.pos[0] + 1 < W-1 &&
                    !board.at(self.pos[0]+2, self.pos[1]) &&
                    !board.at(self.pos[0]+2, self.pos[1]-1)
                {
                    self.pos[0] += 1;
                    return true;
                }
            },
            Dir::Down => {
                if self.pos[1] - 1 > 0 &&
                    !board.at(self.pos[0], self.pos[1]-2) &&
                    !board.at(self.pos[0]+1, self.pos[1]-2)
                {
                    self.pos[1] -= 1;
                    return true;
                }
            },
        };
        return false;
    }

    fn top(&self) -> usize { self.pos[1] }

    fn mark_board(&self, board: &mut Board) {
        *board.at_mut(self.pos[0], self.pos[1]) = true;
        *board.at_mut(self.pos[0]+1, self.pos[1]) = true;
        *board.at_mut(self.pos[0], self.pos[1]-1) = true;
        *board.at_mut(self.pos[0]+1, self.pos[1]-1) = true;
    }

    fn getx(&self) -> usize { self.pos[0] }
}

impl Board {
    fn from_str(txt: &str) -> Self {
        Board {
            board: vec![false; W*HMAX],
            jets: txt.lines().next().unwrap().chars()
                .map(|c|
                    match c { '<' => { Dir::Left }, '>' => { Dir::Right } _=> { panic!("bad char") }
                }).collect()
            ,
            nextjet: 0,
            next: 0,
            height: 0,
        }
    }

    fn drop_next(&mut self) -> usize {
        let mut piece: Box<dyn Moveable> = match self.next {
            0 => Box::new(HLine { pos: [2, self.height + 3] }),
            1 => Box::new(Plus { pos: [2, self.height + 5] }),
            2 => Box::new(Ell { pos: [2, self.height + 5] }),
            3 => Box::new(VLine { pos: [2, self.height + 6] }),
            4 => Box::new(Square { pos: [2, self.height + 4] }),
            _ => panic!("unknown next"),
        };
        self.next = (self.next + 1) % 5;

        loop {
            // println!("using jet {}, next {}", self.nextjet, self.next);
            let dir = &self.jets[self.nextjet];
            self.nextjet = (self.nextjet + 1) % self.jets.len();  // iterators in structs are hard...
            piece.try_move(&self, dir);
            if !piece.try_move(&self, &Dir::Down){
                break;
            }
        }

        piece.mark_board(self);
        self.height = max(self.height, piece.top() + 1);
        // println!("{}", self.height);
        piece.getx()
    }

    fn at(&self, i: usize, j: usize) -> bool {
        self.board[i*HMAX + j]
    }

    fn at_mut(&mut self, i: usize, j: usize) -> &mut bool {
        &mut self.board[i*HMAX + j]
    }

    fn draw(&self) {
        for j in (0..self.height).rev() {
            print!("|");
            for i in 0..W {
                print!("{}", if self.at(i,j) { "#" } else { "." });
            }
            println!("|");
        }
        println!("---------");
    }
}

fn part1(txt: &str) -> usize {
    let mut b = Board::from_str(txt);
    
    for _ in 0..2022 {
        b.drop_next();
    }
    // b.draw();
    b.height
}

fn part2(txt: &str) -> usize {
    let mut b = Board::from_str(txt);
    
    let burnin = 100000;
    for _ in 0..burnin-1 {
        b.drop_next();
    }
    let xstart = b.drop_next();
    let hstart = b.height;
    let jstart = b.nextjet;
    println!("xstart {}", xstart);

    let mut count = 0;
    loop {
        for _ in 0..4 {
            b.drop_next();
        }
        let x = b.drop_next();
        count += 5;

        if b.nextjet == jstart && // b.next == 0
            x == xstart 
            // [2,3,4,5].into_iter().any(|i| b.at(i, b.height-1))
        {
        // if [0,1,2].into_iter().all(|i| b.at(i, b.height-1)) {
            // assert!(b.next == 0);
            break;
        }
    }
    // b.draw();
    let hcycle = b.height - hstart;
    println!("cycle size {} with height {} starting at {}", count, hcycle, burnin);

    let left = (1000000000000 - burnin) % count;
    println!("left: {}, count: {}", left, count);
    for _ in 0..left {
        b.drop_next();
        // b.draw();
    }
    
    hcycle * ((1000000000000 - burnin) / count) + (b.height - hcycle)
}

fn main() {
    let dayX = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    let path = String::from(root) + "/src/" + dayX + "/input.txt";
    // let path = String::from(root) + "/src/" + dayX + "/test1.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", dayX);
    println!("Part 1: {:?}", part1(&txt));
    println!("Part 2: {:?}", part2(&txt));
}
