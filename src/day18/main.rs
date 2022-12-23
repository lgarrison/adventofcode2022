#![allow(non_snake_case)]

use std::{fs, collections::VecDeque};

#[derive(Debug,PartialEq,Clone,Copy)]
enum Kind {
    Outside,
    Inside,
    Lava,
}

#[derive(Debug)]
struct Droplet {
    cubes: Vec<Kind>,
    N: usize,
}

impl Droplet {
    fn from_str(txt: &str) -> Self {
        let coords: Vec<[usize;3]> = 
            txt.lines().map(
                |l| l.split(',').map(
                    |c| c.parse::<usize>().unwrap()
                ).collect::<Vec<usize>>().try_into().unwrap()
            ).collect();
        let N: usize = (0..3).map(
            |i| coords.iter().map(
                |xyz| xyz[i]
            ).max().unwrap()
        ).max().unwrap() + 1;

        let mut cubes: Vec<Kind> = vec![Kind::Inside;N*N*N];
        for x in coords {
            cubes[x[0]*N*N + x[1]*N + x[2]] = Kind::Lava;
        }
        
        Droplet {
            cubes: cubes,
            N: N,
         }
    }

    fn at(&self, i: usize, j: usize, k: usize) -> Kind {
        self.cubes[i*self.N*self.N + j*self.N + k]
    }

    fn count_faces(&self) -> usize {
        let N = self.N;
        let mut tot = 0;
        for i in 0..self.N {
            for j in 0..self.N {
                for k in 0..self.N {
                    if self.at(i,j,k) != Kind::Lava { continue; }
                    
                    if i == 0 { tot += 1; }
                    else if self.at(i-1, j, k) != Kind::Lava { tot += 1; }

                    if i == N - 1 { tot += 1; }
                    else if self.at(i+1, j, k) != Kind::Lava { tot += 1; }

                    if j == 0 { tot += 1; }
                    else if self.at(i, j-1, k) != Kind::Lava { tot += 1; }

                    if j == N - 1 { tot += 1; }
                    else if self.at(i, j+1, k) != Kind::Lava { tot += 1; }

                    if k == 0 { tot += 1; }
                    else if self.at(i, j, k-1) != Kind::Lava { tot += 1; }

                    if k == N - 1 { tot += 1; }
                    else if self.at(i, j, k+1) != Kind::Lava { tot += 1; }

                }
            }
        }
        tot
    }


    fn flood_fill(&mut self) {
        let N = self.N;
        let mut queue: VecDeque<[usize;3]> = VecDeque::with_capacity(N*N*N);
        queue.push_back([0;3]);

        while let Some(i) = queue.pop_front() {
            match self.at(i[0], i[1], i[2]) {
                Kind::Inside => {
                    self.cubes[i[0]*N*N + i[1]*N + i[2]] = Kind::Outside;
                },
                Kind::Lava => {
                    continue;
                }
                Kind::Outside => { 
                    continue;
                },
            }

            if i[0] > 0 {
                queue.push_back([i[0]-1, i[1], i[2]]);
            }
            if i[0] < N-1 {
                queue.push_back([i[0]+1, i[1], i[2]]);
            }
            if i[1] > 0 {
                queue.push_back([i[0], i[1]-1, i[2]]);
            }
            if i[1] < N-1 {
                queue.push_back([i[0], i[1]+1, i[2]]);
            }
            if i[2] > 0 {
                queue.push_back([i[0], i[1], i[2]-1]);
            }
            if i[2] < N-1 {
                queue.push_back([i[0], i[1], i[2]+1]);
            }
        }
    }

    fn count_exterior(&mut self) -> usize {
        self.flood_fill();

        let N = self.N;
        let mut tot = 0;
        for i in 0..self.N {
            for j in 0..self.N {
                for k in 0..self.N {
                    if self.at(i,j,k) != Kind::Lava { continue; }
                    
                    if i == 0 { tot += 1; }
                    else if self.at(i-1, j, k) == Kind::Outside { tot += 1; }

                    if i == N - 1 { tot += 1; }
                    else if self.at(i+1, j, k) == Kind::Outside { tot += 1; }

                    if j == 0 { tot += 1; }
                    else if self.at(i, j-1, k) == Kind::Outside { tot += 1; }

                    if j == N - 1 { tot += 1; }
                    else if self.at(i, j+1, k) == Kind::Outside { tot += 1; }

                    if k == 0 { tot += 1; }
                    else if self.at(i, j, k-1) == Kind::Outside { tot += 1; }

                    if k == N - 1 { tot += 1; }
                    else if self.at(i, j, k+1) == Kind::Outside { tot += 1; }

                }
            }
        }
        tot

    }
}

fn part1(txt: &str) -> usize {
    let d = Droplet::from_str(txt);
    // println!("{:#?}", d);
    d.count_faces()
}

fn part2(txt: &str) -> usize {
    let mut d = Droplet::from_str(txt);
    d.count_exterior()
}

fn main() {
    let dayX = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    let path = String::from(root) + "/src/" + dayX + "/input.txt";
    // let path = String::from(root) + "/src/" + dayX + "/test1.txt";
    // let path = String::from(root) + "/src/" + dayX + "/test2.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", dayX);
    println!("Part 1: {:?}", part1(&txt));
    println!("Part 2: {:?}", part2(&txt));
}
