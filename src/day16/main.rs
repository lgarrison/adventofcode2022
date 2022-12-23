#![allow(non_snake_case)]

use std::{fs, collections::{HashMap, VecDeque}, ptr::read_unaligned};

use regex::Regex;

#[derive(Debug)]
struct Valve {
    rate: i64,
    tunnels: Vec<usize>,
}

struct ValveStr {
    rate: i64,
    tunnels: Vec<String>,
}

#[derive(Debug)]
struct Network { 
    valves: Vec<Valve>,
    pair_dist: Vec<Vec<i64>>,
    AA: usize,
}

type Bitmask = u64;

pub trait BitmaskTrait {
    fn isset(&self, i: usize) -> bool;
    fn withset(&self, i: usize) -> Self;
}

impl BitmaskTrait for Bitmask {
    fn isset(&self, i: usize) -> bool {
        (self & (1 << i)) != 0
    }

    fn withset(&self, i: usize) -> Self {
        self | (1 << i)
    }
}

// need list of unopened and "reward" for opening next
// reward is (mins_remain - mins_to_valve)*rate
// rewards for others can *only* stay the same or go down
// so for each choice, we can sum up all the others.
// if choice A plus the sum of its others is less than
// another choice, prune choice A

// can also peek: for each choice, peek at the *next* rewards
// list for taking that choice. sum up the rewards; that, plus
// the reward of taking the choice, is the upper limit.
// if this upper limit is lower than the best immediate choice's
// value, prune first choice


impl Network {
    fn from_str(txt: &str) -> Self {
        let re_rate = Regex::new(r"flow rate=(\d+)").unwrap();
        let re_AA = Regex::new(r"[A-Z]{2}").unwrap();

        // let mut valves: Vec<Valve> = vec![];
        let mut valve_ids: HashMap<String,usize> = HashMap::new();
        let mut valves: Vec<ValveStr> = vec![];
        for line in txt.lines() {
            let mut AA_cap = re_AA.captures_iter(line);
            let AA = AA_cap.next().unwrap()[0].to_owned();
            let _id = *valve_ids.entry(AA)
                .or_insert(valves.len());

            valves.push( ValveStr {
                rate: re_rate.captures_iter(line).next().unwrap()[1]
                    .parse::<i64>().unwrap(),
                tunnels: AA_cap.map(|c| c[0].to_owned()).collect(),
            });
        }

        let intvalves: Vec<Valve> = valves.iter().map(|v| Valve {
            rate: v.rate,
            tunnels: v.tunnels.iter()
                .map(|t| *valve_ids.get(t).unwrap()).collect(),
        }).collect();
        let nvalve = intvalves.len();

        let mut network = Network {
            valves: intvalves,
            pair_dist: vec![vec![60i64; nvalve]; nvalve],
            AA: valve_ids["AA"],
        };

        // println!("{:#?}", network);

        network.fill_pair_dist();

        network
    }

    fn max_reward(&self, from: usize, mins_left: i64,
        opened: Bitmask, bestsofar: i64) -> i64 {
        if mins_left <= 0 {
            return 0;
        }

        let rewards = self.rewards(from, mins_left, opened);
        if rewards.len() == 0 {
            return 0;
        }
        let max_immediate = rewards.iter().map(|r| r.1).max().unwrap();

        let upper_lim: Vec<i64> = rewards.iter().map(|r| r.1 +
            self.rewards(r.0, mins_left - self.pair_dist[from][r.0] - 1,
            opened.withset(r.0)).iter()
            .map(|q| q.1).sum::<i64>()).collect();

        // println!("rewards: {}", rewards.len());
        // let mut try_rewards: Vec<(usize,i64)> = rewards.into_iter().enumerate().filter(
        //     |(i,_r)| max_immediate <= upper_lim[*i]
        //         && upper_lim[*i] > bestsofar
        //         && upper_lim[*i] > 0
        // ).map(|(_i,r)| r).collect();
        // // let mut try_rewards = rewards;
        // // println!("try: {}", try_rewards.len());

        // try_rewards.sort_by_key(|(_i,r)| *r);
        // try_rewards.reverse();
        let try_rewards = rewards.clone();

        let mut max = 0;
        for (j,r) in try_rewards {
            if max == 0 { max = r; }
            let this = r + 
                self.max_reward(j,
                    mins_left - self.pair_dist[from][j] - 1,
                    opened.withset(j),
                    max - r,
                );
            max = if this > max { this } else { max };
        }
        max
    }

    fn max_reward_p2(&self, from: usize, mins_left: i64, opened: Bitmask) -> i64 {
        let rewards1 = self.rewards(from, mins_left, opened);

        // if rewards1.len() > 0 {
            let mut max = 0;
            for &(i,r1) in &rewards1 {
                let this = r1 +
                    self.max_reward_p2(i,
                        mins_left - self.pair_dist[from][i] - 1,
                        opened.withset(i),
                    );
                max = if this > max { this } else { max };
            }
            let this = self.max_reward(self.AA, 26, opened, 0);
            max = if this > max { this } else { max };

            return max;
        // }
        // else {
        //     // we're at the end of the first DFS
        //     return self.max_reward(self.AA, 26, opened, 0);
        // }
    }

    fn rewards(&self, from: usize, mins_left: i64, opened: Bitmask) -> Vec<(usize,i64)> {

        let mut ret: Vec<(usize,i64)> = Vec::with_capacity(self.valves.len());
        for i in (0..self.valves.len()).filter(|&i|
            !opened.isset(i)
            && self.pair_dist[from][i] + 1 < mins_left
            && self.valves[i].rate > 0)
        {
            ret.push((i,
                (mins_left - self.pair_dist[from][i] - 1)*self.valves[i].rate)
            );
        }

        ret 
    }

    fn fill_pair_dist(&mut self) {
        for i in 0..self.valves.len() {
            if i != self.AA && self.valves[i].rate == 0 { continue; }
            self.pair_dist[i][i] = 0;
            for j in (i+1)..self.valves.len() {
                if j != self.AA && self.valves[j].rate == 0 { continue; }
                self.pair_dist[i][j] = self.min_dist(i,j);
                // assert!(self.min_dist(j,i) == self.pair_dist[i][j]);
                self.pair_dist[j][i] = self.pair_dist[i][j];
            }
        }
    }

    fn min_dist(&self, i: usize, j: usize) -> i64 {
        let mut tocheck: VecDeque<(usize,i64)> = VecDeque::with_capacity(self.valves.len());
        tocheck.push_back((i,0));

        while let Some((k,cost)) = tocheck.pop_front() {
            if k == j {
                return cost;
            }
            tocheck.extend(
                self.valves[k].tunnels.iter()
                    .map(|&t| (t,cost+1))
            );
        }

        panic!("no path found!")
    }

}

fn part1(txt: &str) -> i64 {
    // guesses:
    // 2154
    let network = Network::from_str(txt);
    // println!("{:#?}", network);
    println!("build done");
    network.max_reward(network.AA, 30,
        0, 0)
}

fn part2(txt: &str) -> i64 {
    let network = Network::from_str(txt);
    println!("build done");
    network.max_reward_p2(network.AA,
        26, 0,)
}

fn main() {
    let dayX = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    // let path = String::from(root) + "/src/" + dayX + "/input.txt";
    let path = String::from(root) + "/src/" + dayX + "/test1.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", dayX);
    println!("Part 1: {:?}", part1(&txt));
    println!("Part 2: {:?}", part2(&txt));
}
