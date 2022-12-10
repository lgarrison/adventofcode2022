#![allow(non_snake_case)]

use std::fs;

#[derive(Debug)]
struct Grid {
    grid: Vec<bool>,
    N: usize,
    knots: Vec<[i64;2]>
}

impl Grid {
    fn new(N: usize, n_knots: usize) -> Self {
        let mut g = Self {
            grid: vec![false;N*N],
            N: N,
            knots: vec![[(N/2) as i64;2];n_knots],
        };

        g.grid[(N/2)*N + N/2] = true;

        g
    }

    fn step(&mut self, dx: i64, dy: i64) {
        self.knots[0][0] += dx;
        self.knots[0][1] += dy;

        for i in 1..self.knots.len() {
            let head = self.knots[i-1];
            let tail = &mut self.knots[i];

            if (head[0] - tail[0]).abs() > 1 ||
               (head[1] - tail[1]).abs() > 1   {
                tail[0] += (head[0] - tail[0]).signum();
                tail[1] += (head[1] - tail[1]).signum();
            }

            let tail = self.knots.last().unwrap();
            self.grid[tail[0] as usize*self.N + tail[1] as usize] = true;
        }
    }

    fn count(&self) -> usize {
        self.grid.iter().map(|&b| b as usize).sum()
    }

    fn print(&self) {
        for j in (0..self.N).rev() {
            for i in 0..self.N {
                print!("{}", if self.grid[i*self.N + j] { "#" } else { "." } );
            }
            println!();
        }
    }
}

fn part1(txt: &str) -> i64 {
    let mut grid = Grid::new(1000, 2);

    for line in txt.lines() {
        let mut parts = line.split_whitespace();
        let dir = parts.next().unwrap();
        let num = parts.next().unwrap().parse::<usize>().unwrap() as i64;

        match dir {
            "U" => {
                for _ in 0..num {
                    grid.step(0, 1);
                }
            }
            "D" => {
                for _ in 0..num {
                    grid.step(0, -1);
                }
            }
            "L" => {
                for _ in 0..num {
                    grid.step(-1, 0);
                }
            }
            "R" => {
                for _ in 0..num {
                    grid.step(1, 0);
                }
            }
            _ => {
                panic!("unknown dir!");
            }
        };

        // println!("tail: ({:?},{:?}); head: ({:?},{:?})", Tx, Ty, Hx, Hy);

        // grid.print();

        // println!("---------------");
    }
    
    grid.count() as i64
}

fn part2(txt: &str) -> i64 {
    let mut grid = Grid::new(1000, 10);

    for line in txt.lines() {
        let mut parts = line.split_whitespace();
        let dir = parts.next().unwrap();
        let num = parts.next().unwrap().parse::<usize>().unwrap() as i64;

        match dir {
            "U" => {
                for _ in 0..num {
                    grid.step(0, 1);
                }
            }
            "D" => {
                for _ in 0..num {
                    grid.step(0, -1);
                }
            }
            "L" => {
                for _ in 0..num {
                    grid.step(-1, 0);
                }
            }
            "R" => {
                for _ in 0..num {
                    grid.step(1, 0);
                }
            }
            _ => {
                panic!("unknown dir!");
            }
        };
    }
    
    grid.count() as i64
}

#[allow(non_snake_case)]
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
