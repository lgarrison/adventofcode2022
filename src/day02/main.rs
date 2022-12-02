use std::{fs, str::Lines};

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn from_string(s: &str) -> RPS {
        match s {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissors,
            _ => panic!("no match!"),
        }
    }

    fn value(self: &RPS) -> i64 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn score_as_outcome(self: &RPS) -> i64 {
        match self {
            RPS::Rock => 0,
            RPS::Paper => 3,
            RPS::Scissors => 6,
        }
    }

    fn response_for_outcome(self: &RPS, outcome: RPS) -> RPS {
        match self {
            RPS::Rock => {
                if outcome == RPS::Paper {
                    RPS::Rock
                } else if outcome == RPS::Scissors {
                    RPS::Paper
                } else {
                    RPS::Scissors
                }
            }
            RPS::Paper => {
                if outcome == RPS::Scissors {
                    RPS::Scissors
                } else if outcome == RPS::Rock {
                    RPS::Rock
                } else {
                    RPS::Paper
                }
            }
            RPS::Scissors => {
                if outcome == RPS::Rock {
                    RPS::Paper
                } else if outcome == RPS::Paper {
                    RPS::Scissors
                } else {
                    RPS::Rock
                }
            }
        }
    }

    // The scoring is for resp (the response)
    fn score(self: &RPS, resp: RPS) -> i64 {
        match self {
            RPS::Rock => {
                if resp == RPS::Paper {
                    6
                } else if resp == RPS::Scissors {
                    0
                } else {
                    3
                }
            }
            RPS::Paper => {
                if resp == RPS::Scissors {
                    6
                } else if resp == RPS::Rock {
                    0
                } else {
                    3
                }
            }
            RPS::Scissors => {
                if resp == RPS::Rock {
                    6
                } else if resp == RPS::Paper {
                    0
                } else {
                    3
                }
            }
        }


    }
}

fn get_guide(lines: Lines) -> (Vec<RPS>, Vec<RPS>) {
    let mut opp = Vec::new();
    let mut guide = Vec::new();

    for line in lines {
        let vals: Vec<&str> = line.split_whitespace().collect();
        assert!(vals.len() == 2);
        opp.push(RPS::from_string(vals[0]));
        guide.push(RPS::from_string(vals[1]));
    }

    (opp, guide)
}

fn part1(txt: &str) -> i64 {
    let (opp, guide) = get_guide(txt.lines());

    let mut score: i64 = 0;
    for it in opp.iter().zip(guide.iter()) {
        let (a,b) = it;
        score += a.score(*b);
        score += b.value();
    }
    //println!("{:?}", opp);
    //println!("{:?}", guide);
    
    score
}

fn part2(txt: &str) -> i64 {
    let (opp, guide) = get_guide(txt.lines());

    let mut score: i64 = 0;
    for it in opp.iter().zip(guide.iter()) {
        let (a,b) = it;
        score += b.score_as_outcome();
        score += a.response_for_outcome(*b).value();
        //score += b.value();
    }
    //println!("{:?}", opp);
    //println!("{:?}", guide);
    
    score
}

#[allow(non_snake_case)]
fn main() {
    let dayX = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    let path = String::from(root) + "/src/" + dayX + "/input.txt";
    //let path = String::from(root) + "/src/" + dayX + "/test1.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", dayX);
    println!("Part 1: {}", part1(&txt));
    println!("Part 2: {}", part2(&txt));
}
