#![allow(non_snake_case)]

use std::fs;
use std::collections::{HashMap, VecDeque};
use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Opcode { ADDX, NOOP }

lazy_static! {
    static ref LATENCY: HashMap<Opcode,usize> = HashMap::from([
        (Opcode::ADDX,2),
        (Opcode::NOOP,1),
    ]);
}

#[derive(Debug)]
struct Instr {
    op: Opcode,
    val: Option<i64>,
}

#[derive(Debug)]
struct CPU {
    X: i64,
    pipeline: VecDeque<Instr>,
    cycle: usize,
    left: usize,
}

impl Instr {
    fn from_str(line: &str) -> Self {
        let mut tokens = line.split_whitespace();
        Self {
            op: match tokens.next().unwrap() {
                "addx" => Opcode::ADDX,
                "noop" => Opcode::NOOP,
                _ => panic!("bad op"),
            },
            val: tokens.next().map(|v| v.parse::<i64>().unwrap()),
        }
    }
}

impl CPU {
    fn new(instr: Vec<Instr>) -> Self {
        Self {
            X: 1,
            pipeline: VecDeque::from(instr),
            cycle: 1,
            left: 0,
        }
    }

    fn stepto(&mut self, until: usize) -> (i64,[bool;40]) {
        let mut x: i64 = 0;
        let mut line = [false;40];
        for c in 0..=until-self.cycle {
            if self.left == 0 {
                self.left = *LATENCY.get(&self.pipeline.front().unwrap().op).unwrap();
            }

            self.left -= 1;
            self.cycle += 1;

            x = self.X;

            if (x - c as i64).abs() < 2 {
                line[c] = true;
            }

            if self.left == 0 {
                let instr = self.pipeline.pop_front().unwrap();
                match instr.op {
                    Opcode::ADDX => { self.X += instr.val.unwrap(); },
                    Opcode::NOOP => { },
                };
            }
        }

        (x,line)
    }
}

fn part1(txt: &str) -> i64 {
    let instr = txt.lines().map(|l| Instr::from_str(l)).collect();
    let mut cpu = CPU::new(instr);
    
    let ret = (20..=220).step_by(40).map(|c| c as i64*cpu.stepto(c).0).sum();

    println!("{:?}", cpu);

    ret
}

fn part2(txt: &str) -> String {
    let instr = txt.lines().map(|l| Instr::from_str(l)).collect();
    let mut cpu = CPU::new(instr);
    
    (40..=240).step_by(40).map(|c| cpu.stepto(c).1
        .map(|v| if v { '#' } else { '.' } )
        .iter().collect()).collect::<Vec<String>>().join("\n")
}

fn main() {
    let dayX = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    let path = String::from(root) + "/src/" + dayX + "/input.txt";
    // let path = String::from(root) + "/src/" + dayX + "/test1.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", dayX);
    println!("Part 1: {:?}", part1(&txt));
    println!("Part 2: \n{}", part2(&txt));
}
