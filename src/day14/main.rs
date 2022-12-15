#![allow(non_snake_case)]

use std::fs;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Block { Air, Sand, Rock, Hole }

struct Cave {
    grid: Vec<Block>,
    xstart: usize,
    Nx: usize,
    Ny: usize,
    xhole: usize
}

impl Cave {
    fn from_str(txt: &str, floor: bool) -> Self {
        let paths: Vec<Vec<[usize;2]>> = txt.lines().map(|l| l.split(" -> ")
            .map(|c| c.split(",")
                .map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>().try_into().unwrap())
            .collect()
        ).collect();

        let maxy = paths.iter().flatten().map(|c| c[1]).max().unwrap() + 2;
        
        let (minx, maxx) = if floor {
            (0, paths.iter().flatten().map(|c| c[0]).max().unwrap() + 2 + maxy)
        } else {
            (paths.iter().flatten().map(|c| c[0]).min().unwrap() - 1,
                paths.iter().flatten().map(|c| c[0]).max().unwrap() + 2)
        };
        
        let Nx = maxx - minx;

        let mut cave = Cave {
            grid: vec![Block::Air; maxy*Nx],
            xstart: minx,
            Nx: Nx,
            Ny: maxy,
            xhole: 500 - minx,
        };

        cave.grid[cave.xhole] = Block::Hole;

        for path in paths {
            for points in path.windows(2)  {
                cave.draw(&points[0], &points[1]);
            }
        }

        cave
    }

    fn draw(&mut self, p1: &[usize;2], p2: &[usize;2]) {
        for x in (p1[0]..=p2[0]).chain(p2[0]..=p1[0]) {
            self.grid[(x - self.xstart) + p1[1]*self.Nx] = Block::Rock;
        }
        for y in (p1[1]..=p2[1]).chain(p2[1]..=p1[1]) {
            self.grid[(p1[0] - self.xstart) + y*self.Nx] = Block::Rock;
        }
    }

    fn fill_until_spill(&mut self) -> usize {
        let mut count = 0;
        while self.drop() {
            count += 1;
        }
        count
    }

    fn fill_until_full(&mut self) -> usize {
        let mut count = 0;
        while self.grid[self.xhole] != Block::Sand {
            self.drop();
            count += 1;
        }
        count
    }

    fn drop(&mut self) -> bool {
        let mut grain = [self.xhole, 0];

        loop {
            if grain[1] + 1 == self.Ny {
                self.grid[grain[1]*self.Nx + grain[0]] = Block::Sand;
                return false;
            }
            if self.grid[(grain[1] + 1)*self.Nx + grain[0]] == Block::Air {
                grain[1] += 1;
                continue;
            }
            if self.grid[(grain[1] + 1)*self.Nx + grain[0] - 1] == Block::Air {
                grain[1] += 1;
                grain[0] -= 1;
                continue;
            }
            if self.grid[(grain[1] + 1)*self.Nx + grain[0] + 1] == Block::Air {
                grain[1] += 1;
                grain[0] += 1;
                continue;
            }
            
            break;
        }
        self.grid[grain[1]*self.Nx + grain[0]] = Block::Sand;
        true
    }

}

impl std::fmt::Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..self.Ny {
            for x in 0..self.Nx {
                write!(f, "{}", self.grid[y*self.Nx + x]).expect("write failed");
            }
            write!(f, "\n").expect("write failed");
        }
        write!(f, "xstart: {}, Nx: {}, Ny: {}, xhole: {}",
            self.xstart, self.Nx, self.Ny, self.xhole,
        )
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                Block::Air => ".",
                Block::Rock => "#",
                Block::Sand => "o",
                Block::Hole => "+",
            }
        )
    }
}

fn part1(txt: &str) -> usize {
    let mut cave = Cave::from_str(txt, false);
    
    cave.fill_until_spill()
}

fn part2(txt: &str) -> usize {
    let mut cave = Cave::from_str(txt, true);

    // println!("{}", cave);
    
    cave.fill_until_full()
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
