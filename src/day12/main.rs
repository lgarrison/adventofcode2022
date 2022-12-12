#![allow(non_snake_case)]

use std::{fs, collections::BinaryHeap};
use std::cmp::Reverse;

#[derive(Debug)]
struct Grid {
    grid: Vec<i64>,
    N: usize,
    M: usize,
    start: usize,
    end: usize,
}

type Node = Reverse<(usize,usize)>;

impl Grid {
    fn from_txt(txt: &str) -> Self {
        Grid {
            grid: Vec::from_iter(
                txt.lines().flat_map(
                    |l| l.chars().map(
                        |c| if c == 'S' { 'a' as i64 } else {
                            if c == 'E' { 'z' as i64 } else { c as i64 } } - 'a' as i64
                    )
                )
            ),
            N: txt.lines().count(),
            M: txt.lines().next().unwrap().len(),
            start: txt.lines().flat_map(|l| l.chars()).position(|c| c == 'S').unwrap(),
            end: txt.lines().flat_map(|l| l.chars()).position(|c| c == 'E').unwrap(),
        }
    }

    fn mincost(&self, from: usize) -> usize {
        let i = (from / self.M) as i64;
        assert!( i < self.N as i64);
        let j = (from % self.M) as i64;

        let x = (self.end / self.M) as i64;
        assert!( x < self.N as i64);
        let y = (self.end % self.M) as i64;

        ((x - i).abs() + (y - j).abs()) as usize
    }

    fn shortest_path(&self, from: usize, to: usize) -> usize {
        let mut queue = BinaryHeap::from([Reverse((0usize, from))]);
        let mut count = 0;
        let mut bestdist = vec![usize::MAX;self.grid.len()];
        bestdist[0] = 0;

        while let Some(Reverse(v)) = queue.pop() {
            assert!(bestdist[v.1] >= v.0);
            bestdist[v.1] = v.0;
            // println!("{:?}", v);
            if v.1 == to {
                return v.0;
            }
            let trans: Vec<Node> = self.transitions(v, &mut bestdist).iter()
                .map(|&q| Reverse((v.0 + 1, q))).collect();
            for Reverse(t) in trans.iter() {
                assert!(bestdist[t.1] >= t.0);
                bestdist[t.1] = t.0;
            }
            queue.extend(trans);
            // println!("{:?}", queue);
            
            count += 1;
            // if count == 3 { break; }
        }
        panic!("search failed");
    }

    fn transitions(&self, from: (usize,usize), bestdist: &mut Vec<usize>) -> Vec<usize> {
        let i = (from.1 / self.M) as i64;
        assert!( i < self.N as i64);
        let j = (from.1 % self.M) as i64;

        [-1,0,1].iter()
            .filter_map(|dx| if i + dx >= 0 && i + dx < self.N as i64 {
                Some(
                    [-1,0,1].iter().filter_map(
                        |dy| if j + dy >= 0 && j + dy < self.M as i64 && ((*dx == 0) != (*dy == 0))
                            && from.0 + 1 < bestdist[((i+*dx)*self.M as i64 + j+dy) as usize] {
                            Some(((i+*dx)*self.M as i64 + j+dy) as usize)
                    } else {
                        None
                    }
                )
            )
        } else { None }
        ).flatten().filter(|&k| self.grid[k] - self.grid[from.1] <= 1).collect()
    }

    fn shortest_to_a(&self, from: usize) -> usize {
        let mut queue = BinaryHeap::from([Reverse((0usize, from))]);
        let mut count = 0;
        let mut bestdist = vec![usize::MAX;self.grid.len()];
        bestdist[0] = 0;

        while let Some(Reverse(v)) = queue.pop() {
            assert!(bestdist[v.1] >= v.0);
            bestdist[v.1] = v.0;
            // println!("{:?}", v);
            if self.grid[v.1] == 0 {
                return v.0;
            }
            let trans: Vec<Node> = self.transitions_p2(v, &mut bestdist).iter()
                .map(|&q| Reverse((v.0 + 1, q))).collect();
            for Reverse(t) in trans.iter() {
                assert!(bestdist[t.1] >= t.0);
                bestdist[t.1] = t.0;
            }
            queue.extend(trans);
            // println!("{:?}", queue);
            
            count += 1;
            // if count == 3 { break; }
        }
        panic!("search failed");
    }

    fn transitions_p2(&self, from: (usize,usize), bestdist: &mut Vec<usize>) -> Vec<usize> {
        let i = (from.1 / self.M) as i64;
        assert!( i < self.N as i64);
        let j = (from.1 % self.M) as i64;

        [-1,0,1].iter()
            .filter_map(|dx| if i + dx >= 0 && i + dx < self.N as i64 {
                Some(
                    [-1,0,1].iter().filter_map(
                        |dy| if j + dy >= 0 && j + dy < self.M as i64 && ((*dx == 0) != (*dy == 0))
                            && from.0 + 1 < bestdist[((i+*dx)*self.M as i64 + j+dy) as usize] {
                            Some(((i+*dx)*self.M as i64 + j+dy) as usize)
                    } else {
                        None
                    }
                )
            )
        } else { None }
        ).flatten().filter(|&k| self.grid[k] - self.grid[from.1] >= -1).collect()
    }

}

fn part1(txt: &str) -> i64 {
    let grid = Grid::from_txt(txt);
    // println!("{:?}", grid);
    grid.shortest_path(grid.start, grid.end) as i64
}

fn part2(txt: &str) -> i64 {
    let grid = Grid::from_txt(txt);
    // println!("{:?}", grid);
    grid.shortest_to_a(grid.end) as i64
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
