#![allow(non_snake_case)]

use std::{fs, collections::VecDeque};

const M: usize = 8;

struct Monkees<'a> {
    items: [VecDeque<usize>; M],
    tests: [&'a dyn Fn(usize) -> usize; M],
    ops: [&'a dyn Fn(usize) -> usize; M],
    n_inspect: [usize; M],
}

impl Monkees<'_> {
    fn new() -> Self {
        let m = Self {
            items: [
                VecDeque::from([75, 75, 98, 97, 79, 97, 64]),
                VecDeque::from([50, 99, 80, 84, 65, 95]),
                VecDeque::from([96, 74, 68, 96, 56, 71, 75, 53]),
                VecDeque::from([83, 96, 86, 58, 92]),
                VecDeque::from([99]),
                VecDeque::from([60, 54, 83]),
                VecDeque::from([77, 67]),
                VecDeque::from([95, 65, 58, 76]),
                ],
            tests: [
                &|w| if w % 19 == 0 { 2 } else { 7 },
                &|w| if w %  3 == 0 { 4 } else { 5 },
                &|w| if w % 11 == 0 { 7 } else { 3 },
                &|w| if w % 17 == 0 { 6 } else { 1 },
                &|w| if w %  5 == 0 { 0 } else { 5 },
                &|w| if w %  2 == 0 { 2 } else { 0 },
                &|w| if w % 13 == 0 { 4 } else { 1 },
                &|w| if w %  7 == 0 { 3 } else { 6 },
            ],
            ops: [
                &|w| w * 13,
                &|w| w +  2,
                &|w| w +  1,
                &|w| w +  8,
                &|w| w *  w,
                &|w| w +  4,
                &|w| w * 17,
                &|w| w +  5,
            ],
            n_inspect: [0; M],
        };
        m
    }

    fn step(&mut self) {
        for m in 0..M {
            while let Some(mut w) = self.items[m].pop_front() {
                self.n_inspect[m] += 1;
                w = self.ops[m](w);
                w /= 3;
                self.items[ self.tests[m](w) ].push_back(w);
            }
        }
    }

    fn step_p2(&mut self) {
        for m in 0..M {
            while let Some(mut w) = self.items[m].pop_front() {
                self.n_inspect[m] += 1;
                w = self.ops[m](w);
                self.items[ self.tests[m](w) ].push_back(w % (19 * 3 * 11 * 17 * 5 * 2 * 13 * 7));
            }
        }
    }

}

fn part1(_txt: &str) -> i64 {
    let mut monkees = Monkees::new();

    for _ in 0..20 {
        monkees.step();
    }
    
    monkees.n_inspect.sort();
    monkees.n_inspect[M-2..].iter().map(|&i| i as i64).product()
}

fn part2(_txt: &str) -> i64 {
    let mut monkees = Monkees::new();

    for _ in 0..10000 {
        monkees.step_p2();
    }
    
    monkees.n_inspect.sort();
    monkees.n_inspect[M-2..].iter().map(|&i| i as i64).product()
}

fn main() {
    let dayX = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    //let path = String::from(root) + "/src/" + dayX + "/input.txt";
    let path = String::from(root) + "/src/" + dayX + "/test1.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", dayX);
    println!("Part 1: {:?}", part1(&txt));
    println!("Part 2: {:?}", part2(&txt));
}
