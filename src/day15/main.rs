#![allow(non_snake_case)]

use std::{fs, ops::{RangeInclusive}, collections::HashMap};

use regex::{Regex, Captures};

#[derive(Debug)]
struct Cave {
    exclusions: HashMap<i64,Vec<RangeInclusive<i64>>>,
    ybeacons: HashMap<i64,usize>,
}

impl Cave {
    fn new(txt: &str) -> Self {
        let re = Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();
        
        let mut exclusions: HashMap<i64,Vec<RangeInclusive<i64>>> = HashMap::new();
        let mut beacons: Vec<[i64;2]> = vec![];
        let mut ybeacons: HashMap<i64,usize> = HashMap::new();
        for line in txt.lines() {
            for cap in re.captures_iter(line).collect::<Vec<Captures>>().chunks(2) {
                let sensor = [
                    cap[0][1].parse::<i64>().unwrap(),
                    cap[0][2].parse::<i64>().unwrap(),
                ];
                let beacon = [
                    cap[1][1].parse::<i64>().unwrap(),
                    cap[1][2].parse::<i64>().unwrap(),
                ];
                if !beacons.contains(&beacon) {
                    beacons.push(beacon);
                    *ybeacons.entry(beacon[1]).or_insert(0) += 1;
                }

                let mhdist = (sensor[0] - beacon[0]).abs() + (sensor[1] - beacon[1]).abs();
                // println!("sensor: {:?}, beacon: {:?}, mhdist: {:?}", sensor, beacon, mhdist);
                
                for dy in -mhdist..=mhdist {
                    exclusions.entry(sensor[1] + dy)
                        .or_insert(Vec::new())
                        .push((sensor[0] - mhdist + dy.abs())..=(sensor[0] + mhdist - dy.abs()));
                }
            }
        }

        // println!("{:?}", exclusions.get(&10));

        let simp: HashMap<i64,Vec<RangeInclusive<i64>>> = HashMap::from_iter(
            exclusions.iter()
            .map(|(&k,v)| (k,simplify_ranges(v)))
        );

        // println!("{:?}", simp.get(&10));
        
        Cave {
            exclusions: simp,
            ybeacons: ybeacons,
        }
    }
}

fn simplify_ranges(ranges: &Vec<RangeInclusive<i64>>) -> Vec<RangeInclusive<i64>> {
    let mut res: Vec<RangeInclusive<i64>> = vec![];

    let mut sorted: Vec<&RangeInclusive<i64>> = ranges.into_iter().collect();
    sorted.sort_by_key(|r| r.start());

    let mut i = 0;
    while i < sorted.len() {
        let left = sorted[i].start();
        let mut right = sorted[i].end();
        for j in (i+1)..sorted.len() {
            if sorted[j].start() <= right {
                i = j;
                if sorted[j].end() > right {
                    right = sorted[j].end();
                }
            }
        }
    
        res.push(*left..=*right);
        i += 1;
    }

    res
}

fn part1(txt: &str, y: i64) -> i64 {
    // For each sensor, build a list of row-wise exclusion ranges
    let cave = Cave::new(txt);
    // println!("{:?}", cave);
    // println!("{:?}", cave.exclusions.get(&10).unwrap());
    cave.exclusions.get(&y).unwrap().iter()
        .map(|r| r.end() + 1 - r.start()).sum::<i64>()
        - *cave.ybeacons.get(&y).unwrap() as i64
}

fn part2(txt: &str) -> i64 {
    let cave = Cave::new(txt);
    for y in 0..=4000000 {
        let v = cave.exclusions.get(&y).unwrap();
        for r in v {
            if r.contains(&0) && !r.contains(&4000000) {
                return (r.end() + 1)*4000000 + y;
            }
        }
    }
    // println!("{:?}", res);
    panic!("not found!")
}

fn main() {
    let dayX = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    let path = String::from(root) + "/src/" + dayX + "/input.txt";
    // let path = String::from(root) + "/src/" + dayX + "/test1.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", dayX);
    // println!("Part 1: {:?}", part1(&txt, 10));
    // println!("Part 1: {:?}", part1(&txt, 2000000));
    println!("Part 2: {:?}", part2(&txt));
}
