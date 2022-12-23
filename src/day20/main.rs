#![allow(non_snake_case)]

use std::{fs, cmp::min, cmp::max};

struct Mixer {
    v: Vec<i64>,
}

impl Mixer {
    fn from_str(txt: &str) -> Self {
        Mixer {
            v: txt.lines().map(|s| s.parse::<i64>().unwrap()).collect(),
        }
    }

    fn score(&self) -> i64 {
        // println!("{} {} {}",
        //     self.v[1000 % self.v.len()],
        //     self.v[2000 % self.v.len()],
        //     self.v[3000 % self.v.len()],
        // );
        self.v[1000 % self.v.len()] + self.v[2000 % self.v.len()] + self.v[3000 % self.v.len()]
    }

    fn mix(&mut self, key: i64, rounds: usize) {
        self.v = self.v.iter().map(|i| i*key).collect();
        // println!("{:#?}", self.v);

        let n = self.v.len() as i64;
        let mut idx: Vec<i64> = (0..n).collect();

        for _ in 0..rounds {
            for i in 0..n as usize {
                let oldidx = idx[i];
                let dist = self.v[i].rem_euclid(n-1);
                let newidx = (idx[i] + dist).rem_euclid(n);
                for j in 0..n as usize {
                    if (idx[j] - oldidx).rem_euclid(n) <= dist {
                        idx[j] = (idx[j] - 1).rem_euclid(n);
                    }
                }
                idx[i] = newidx;
                // println!("{:#?}", idx);
            }
        }

        let zeroidx = idx[self.v.iter().position(|&q| q == 0).unwrap()];

        let mut new = vec![99999;n as usize];
        for (i,val) in self.v.iter().enumerate() {
            let j = (idx[i] - zeroidx).rem_euclid(n) as usize;
            assert!(new[j] == 99999);
            new[j] = *val;
        }
        self.v = new;
        // println!("{:#?}", self.v);
        // println!("{:#?}", zeroidx);
    }
}

fn part1(txt: &str) -> i64 {
    let mut mixer = Mixer::from_str(txt);
    mixer.mix(1, 1);
    mixer.score()
}

fn part2(txt: &str) -> i64 {
    let mut mixer = Mixer::from_str(txt);
    mixer.mix(811589153, 10);
    mixer.score()
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
