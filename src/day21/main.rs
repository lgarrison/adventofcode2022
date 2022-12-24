#![allow(non_snake_case)]

use core::panic;
use std::{fs, collections::HashMap};

#[derive(Debug,Clone)]
struct Expression {
    value: Option<i64>,
    expr: Option<Vec<String>>,
}

impl Expression {
    fn from_str(txt: &str) -> Self {
        let split: Vec<String> = txt.split_whitespace()
            .map(|s| s.to_owned()).collect();
        if split.len() == 1{
            return Expression {
                value: Some(split[0].parse::<i64>().unwrap()),
                expr: None,
            };
        } else {
            return Expression {
                value: None,
                expr: Some(split),
            };
        }
    }
}

#[derive(Debug)]
struct Monkees {
    m: HashMap<String, Expression>
}

impl Monkees {
    fn from_str(txt: &str) -> Self {
        let mut m = HashMap::new();
        for line in txt.lines() {
            let split: Vec<&str> = line.split(": ").collect();
            m.insert(split[0].to_owned(), Expression::from_str(split[1]));
        }

        Monkees {
            m: m,
         }
    }

    fn get_value(&self, root: &str) -> i64 {
        let node = self.m.get(root).unwrap();
        if node.value.is_some() {
            return node.value.unwrap();
        } else {
            let expr: &Vec<String> = node.expr.as_ref().unwrap();
            let v1 = self.get_value(&expr[0]);
            let v2 = self.get_value(&expr[2]);

            let res =  match expr[1].as_str() {
                "+" => { v1 + v2 },
                "*" => { v1 * v2 },
                "-" => { v1 - v2 },
                "/" => { v1 / v2 },
                _ => { panic!("unknown op") }
            };
            // self.m.insert(root.to_owned(), Expression {
            //     value: Some(res),
            //     expr: None,
            // });
            res
        }
    }

    fn try_get_value_p2(&self, root: &str) -> Option<i64> {
        if root == "humn" {
            return None;
        }
        let node = self.m.get(root).unwrap();
        if node.value.is_some() {
            return Some(node.value.unwrap());
        } else {
            let expr: &Vec<String> = node.expr.as_ref().unwrap();
            let v1 = self.try_get_value_p2(&expr[0]);
            let v2 = self.try_get_value_p2(&expr[2]);
            if v1.is_none() || v2.is_none() {
                return None;
            }

            let res =  match expr[1].as_str() {
                "+" => { v1.unwrap() + v2.unwrap() },
                "*" => { v1.unwrap() * v2.unwrap() },
                "-" => { v1.unwrap() - v2.unwrap() },
                "/" => { v1.unwrap() / v2.unwrap() },
                _ => { panic!("unknown op") }
            };
            Some(res)
        }
    }

    fn infer_humn(&self, root: &str, target: i64) -> i64 {
        // println!("{} {}", root, target);
        if root == "humn" {
            return target;
        }
        
        let node = self.m.get(root).unwrap();
        if node.value.is_some() {
            return node.value.unwrap();
        } else {
            let expr: &Vec<String> = node.expr.as_ref().unwrap();
            let v1 = self.try_get_value_p2(&expr[0]);
            let v2 = self.try_get_value_p2(&expr[2]);

            if v1.is_some() {
                let next_target =  match expr[1].as_str() {
                    "+" => { target - v1.unwrap() },
                    "*" => { target / v1.unwrap() },
                    "-" => { -target + v1.unwrap() },
                    "/" => { v1.unwrap() / target },
                    _ => { panic!("unknown op") }
                };
                return self.infer_humn(&expr[2], next_target);
            } else {
                let next_target =  match expr[1].as_str() {
                    "+" => { target - v2.unwrap() },
                    "*" => { target / v2.unwrap() },
                    "-" => { target + v2.unwrap() },
                    "/" => { target * v2.unwrap() },
                    _ => { panic!("unknown op") }
                };
                return self.infer_humn(&expr[0], next_target);
            }
        }
    }
}

fn part1(txt: &str) -> i64 {
    let m = Monkees::from_str(txt);
    // println!("{:#?}", m);
    m.get_value("root")
}

fn part2(txt: &str) -> i64 {
    let m = Monkees::from_str(txt);
    let s1 = &m.m["root"].expr.as_ref().unwrap()[0];
    let s2 = &m.m["root"].expr.as_ref().unwrap()[2];
    let v1 = m.try_get_value_p2(s1);
    let v2 = m.try_get_value_p2(s2);
    let target = if v1.is_none() { (s1,v2.unwrap()) } else { (s2,v1.unwrap()) };
    // println!("{:?}", target);
    m.infer_humn(target.0, target.1)
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
