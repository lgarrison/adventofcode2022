#![allow(non_snake_case)]

use std::{fs, collections::{HashMap, HashSet}};

#[derive(Clone, Copy)]
enum Dir {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Dir {
    fn next(&self) -> Self {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::E,
            Dir::E => Dir::N,
            _ => panic!("bad dir"),
        }
    }
}

type Pos = [i64;2];

type Elf = Pos;

// can't define impl on type alias
fn neigh(p: &Elf, dir: Dir) -> Elf {
    match dir {
        Dir::N => { [p[0] - 1, p[1]] },
        Dir::NE => { [p[0] - 1, p[1] + 1] },
        Dir::E => { [p[0], p[1] + 1] },
        Dir::SE => { [p[0] + 1, p[1] + 1] },
        Dir::S => { [p[0] + 1, p[1]] },
        Dir::SW => { [p[0] + 1, p[1] - 1] },
        Dir::W => { [p[0], p[1] - 1] },
        Dir::NW => { [p[0] - 1, p[1] - 1] },
    }
}

struct Grove {
    elves: HashSet<Elf>,
    prop_dir: Dir,
}

impl Grove {
    fn from_str(txt: &str) -> Self {
        let mut elves = HashSet::new();
        for (i,line) in txt.lines().enumerate() {
            for (j, char) in line.chars().enumerate() {
                if char == '#' {
                    elves.insert([i as i64, j as i64]);
                }
            }
        }

        Grove {
            elves,
            prop_dir: Dir::N,
        }
    }

    fn step(&mut self) -> bool {
        let mut props: HashMap<Pos,Dir> = HashMap::with_capacity(self.elves.len());
        let mut incoming: HashMap<Pos,usize> = HashMap::with_capacity(self.elves.len());

        let mut ret = false;
        for elf in self.elves.iter() {
            let mut try_prop = self.prop_dir;

            if !(self.elves.contains(&neigh(&elf, Dir::N)) ||
                self.elves.contains(&neigh(&elf, Dir::NE)) ||
                self.elves.contains(&neigh(&elf, Dir::E)) || 
                self.elves.contains(&neigh(&elf, Dir::SE)) ||
                self.elves.contains(&neigh(&elf, Dir::S)) ||
                self.elves.contains(&neigh(&elf, Dir::SW)) ||
                self.elves.contains(&neigh(&elf, Dir::W)) ||
                self.elves.contains(&neigh(&elf, Dir::NW)))
            {
                continue;
            }

            for _ in 0..4 {
                match try_prop {
                    Dir::N => {
                        if !self.elves.contains(&neigh(&elf, Dir::NW))
                            && !self.elves.contains(&neigh(&elf, Dir::N))
                            && !self.elves.contains(&neigh(&elf, Dir::NE))
                        {
                            props.insert(*elf, Dir::N);
                            *incoming.entry(neigh(&elf, Dir::N))
                                .or_insert(0) += 1;
                            break;
                        }
                    }

                    Dir::E => {
                        if !self.elves.contains(&neigh(&elf, Dir::NE))
                            && !self.elves.contains(&neigh(&elf, Dir::E))
                            && !self.elves.contains(&neigh(&elf, Dir::SE))
                        {
                            props.insert(*elf, Dir::E);
                            *incoming.entry(neigh(&elf, Dir::E))
                                .or_insert(0) += 1;
                            break;
                        }
                    }

                    Dir::S => {
                        if !self.elves.contains(&neigh(&elf, Dir::SE))
                            && !self.elves.contains(&neigh(&elf, Dir::S))
                            && !self.elves.contains(&neigh(&elf, Dir::SW))
                        {
                            props.insert(*elf, Dir::S);
                            *incoming.entry(neigh(&elf, Dir::S))
                                .or_insert(0) += 1;
                            break;
                        }
                    }

                    Dir::W => {
                        if !self.elves.contains(&neigh(&elf, Dir::SW))
                            && !self.elves.contains(&neigh(&elf, Dir::W))
                            && !self.elves.contains(&neigh(&elf, Dir::NW))
                        {
                            props.insert(*elf, Dir::W);
                            *incoming.entry(neigh(&elf, Dir::W))
                                .or_insert(0) += 1;
                            break;
                        }
                    }

                    _ => panic!("bad dir"),
                }
                try_prop = try_prop.next();
            }
        }

        for (elf,dir) in props.iter() {
            if *incoming.get(&neigh(elf, *dir)).unwrap() == 1 {
                self.elves.remove(elf);
                self.elves.insert(neigh(elf, *dir));
                ret = true;
            }
        }

        self.prop_dir = self.prop_dir.next();
        ret
    }

    fn bounding_area(&self) -> usize {
        let minx = self.elves.iter().map(|a| a[1]).min().unwrap();
        let maxx = self.elves.iter().map(|a| a[1]).max().unwrap();
        let miny = self.elves.iter().map(|a| a[0]).min().unwrap();
        let maxy = self.elves.iter().map(|a| a[0]).max().unwrap();

        (maxy + 1 - miny) as usize * (maxx + 1 - minx) as usize
    }

    fn empties(&self) -> usize {
        let minx = self.elves.iter().map(|a| a[1]).min().unwrap();
        let maxx = self.elves.iter().map(|a| a[1]).max().unwrap();
        let miny = self.elves.iter().map(|a| a[0]).min().unwrap();
        let maxy = self.elves.iter().map(|a| a[0]).max().unwrap();
        
        self.bounding_area() -
            self.elves.iter().filter(|a| a[1] >= minx && a[1] <= maxx &&
                a[0] >= miny && a[0] <= maxy ).count()
    }

    fn print(&self) {
        let minx = self.elves.iter().map(|a| a[1]).min().unwrap();
        let maxx = self.elves.iter().map(|a| a[1]).max().unwrap();
        let miny = self.elves.iter().map(|a| a[0]).min().unwrap();
        let maxy = self.elves.iter().map(|a| a[0]).max().unwrap();
        let Nx = (maxx + 1 - minx) as usize;
        let Ny = (maxy + 1 - miny) as usize;

        let mut map: Vec<bool> = vec![false;Nx*Ny];
        for elf in self.elves.iter() {
            map[(elf[0] - miny) as usize*Nx + (elf[1] - minx) as usize] = true;
        }

        for i in 0..Ny {
            for j in 0..Nx {
                print!("{}", if map[i*Nx + j] { "#" } else { "." });
            }
            println!();
        }
        println!();
    }
}

fn part1(txt: &str) -> usize {
    let mut grove = Grove::from_str(txt);

    // grove.print();
    // println!("{}", grove.bounding_area());

    for _ in 0..10 {
        grove.step();
        // grove.print();
    }

    grove.empties()
}

fn part2(txt: &str) -> i64 {
    let mut grove = Grove::from_str(txt);

    let mut count = 1;
    while grove.step() {
        count += 1;
    }

    count
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
