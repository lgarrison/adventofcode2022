#![allow(non_snake_case)]

use std::{fs, cmp::max};

#[derive(Debug,Clone,Copy)]
struct Factory {
    costs: [[usize;4];4],
    robits: [usize;4],
    ores: [usize;4],
    mins_remain: usize,
}

impl Factory {
    fn from_blueprint(txt: &str, mins_remain: usize) -> Self {
        let mut split = txt.split_whitespace();
        let costs = [
            [split.nth(6).unwrap().parse::<usize>().unwrap(), 0, 0, 0],
            [split.nth(5).unwrap().parse::<usize>().unwrap(), 0, 0, 0],
            [split.nth(5).unwrap().parse::<usize>().unwrap(), 
            split.nth(2).unwrap().parse::<usize>().unwrap(), 0, 0],
            [split.nth(5).unwrap().parse::<usize>().unwrap(),
            0, split.nth(2).unwrap().parse::<usize>().unwrap(), 0],
        ];

        Factory {
            costs: costs,
            robits: [1,0,0,0],
            ores: [0;4],
            mins_remain: mins_remain,
        }
    }

    fn collect(&mut self, nstep: usize) {
        for i in 0..self.robits.len() {
            self.ores[i] += self.robits[i]*nstep;
        }

    }

    fn build(&mut self, robit: usize) {
        for i in 0..3 {
            assert!(self.ores[i] >= self.costs[robit][i]);
            self.ores[i] -= self.costs[robit][i];
        }
        self.robits[robit] += 1;
    }
    
    fn max_geodes(&self, besttotal: usize) -> usize {
        if self.mins_remain == 0 {
            return self.ores[3];
        }

        let mut best = 0;
        let moves = self.moves();
        // println!("current: {:?}", self);
        // println!("moves: {:?}", moves);

        for (target,time) in moves {
            let mut next = self.clone();
            next.collect(time + 1);
            next.mins_remain -= time + 1;
            next.build(target);

            if next.upper_limit() <= max(besttotal,best) {
                continue;
            }
            best = max(best, next.max_geodes(max(besttotal,best)));
        }
        return best;
    }

    fn upper_limit(&self) -> usize {
        self.ores[3] + self.mins_remain * self.robits[3] +
            ((self.mins_remain - 1) * self.mins_remain)/2
    }

    fn moves(&self) -> Vec<(usize,usize)> {  // -> (target,time)
        // doing nothing is an option, but only up to the highest expense
        // N.B. for now we'll build at most 1

        if self.costs[3].iter().zip(self.ores).all(|(a,b)| *a <= b) {
            return vec![(3, 0)];
        }

        let mut moves = vec![];
        for (target,cost) in self.costs.iter().enumerate() {
            let mut maxtime: Option<usize> = None;
            for (j, &k) in cost.iter().enumerate() {
                if k > 0 && self.robits[j] == 0 {
                    maxtime = None;
                    break;
                }
                maxtime = Some(
                    max(maxtime.unwrap_or(0),
                        if self.ores[j] >= k { 0 } else { (k - self.ores[j] - 1)/self.robits[j] + 1} )
                );
            }
            if let Some(mt) = maxtime {
                if mt < self.mins_remain {
                    moves.push((target,mt));
                }
            }
        }

        moves

        // let mut moves: Vec<Option<usize>> =
        //     self.costs.iter().enumerate().filter_map(|(i,&c)|
        //         if c.iter().zip(self.ores).all(|(a,b)| *a <= b)
        //         { Some(Some(i)) } else if i == 3 { Some(None) } else { None }
        //     ).collect();

        // if *moves.last().unwrap() == Some(3usize) {
        //     moves = vec![Some(3)];  // always build geode robot if possible
        // }

        // moves
    }
}

fn part1(txt: &str) -> usize {
    txt.lines().enumerate().map(|(id,l)|
        (id,Factory::from_blueprint(l, 24)))
        // .inspect(|(_,f)| println!("{:#?}", f))
        .map(|(id,f)| f.max_geodes(0)*(id+1))
    .sum::<usize>()
}

fn part2(txt: &str) -> usize {
    txt.lines().take(3).map(|l|
        Factory::from_blueprint(l, 32))
        // .inspect(|(_,f)| println!("{:#?}", f))
        .map(|f| f.max_geodes(0))
    // .inspect(|v| println!("{}", v))
    .product::<usize>()
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
