#![allow(non_snake_case)]

use std::{fs, cmp::Ordering};

#[derive(Debug)]
struct Item {
    list: Option<Vec<Item>>,
    val: Option<i64>,
}

fn list_split(line: &str) -> Item {

    if !line.starts_with("[") {
        return Item {
            list: None,
            val: Some(line.parse::<i64>().unwrap()),
        };
    }

    let mut v: Vec<&str> = Vec::new();
    let mut depth = 0;

    let mut start = 0;
    for (i,c) in line.chars().enumerate() {
        match c {
            '[' => {
                if depth == 0 {
                    start = i+1;
                }
                depth += 1;
            },
            ']' => {
                depth -= 1;
                if depth == 0 {
                    if i > start {
                        v.push(&line[start..i]);
                    }
                }
            },
            ',' => {
                if depth == 1 {
                    if i > start {
                        v.push(&line[start..i]);
                    }
                    start = i+1;
                }
            },
            _ => { },
        }
    }

    let items: Vec<Item> = v.iter().map(|s| list_split(s)).collect();

    Item{ list: Some(items), val: None }
}

fn compare_items(a: &Item, b: &Item) -> Option<bool> {

    if let Some(i) = a.val {
        if let Some(j) = b.val {
            return if i == j { None } else { Some(i < j)};  // two vals
        }
        // a is val, b is vec
        return compare_items(
            &Item {
                list: Some(vec![ Item { list: None, val: Some(i)}]),
                val: None,
            }, b);
    } else if let Some(j) = b.val {
        // a is vec, b is val
        return compare_items(a,
            &Item {
                list: Some(vec![ Item { list: None, val: Some(j)}]),
                val: None,
            }
        );
    }
    else {
        // two vecs
        let alist = a.list.as_ref().unwrap();
        let blist = b.list.as_ref().unwrap();
        for (i,c) in alist.iter().enumerate() {
            if let Some(d) = blist.get(i) {
                if let Some(ret) = compare_items(c, d) {
                    return Some(ret);
                }
            } else {
                return Some(false);  // b ran out
            }
        }
        if blist.len() > alist.len() {
            return Some(true);  // a ran out
        }
        return None;
    }
}


fn check_pair(pair: &[&str]) -> bool {
    let a = list_split(pair[0]);
    let b = list_split(pair[1]);

    // println!("{:?}\n", a);
    // println!("{:?}\n", b);
    
    // println!("{:?}\n", compare_items(&a, &b));
    compare_items(&a, &b).unwrap()
}

fn part1(txt: &str) -> i64 {
    txt.lines().collect::<Vec<&str>>().chunks(3).enumerate()
        .filter(|(_,p)| check_pair(&p[..2]))
        .map(|(i,_)| i + 1).sum::<usize>() as i64
}

fn part2(txt: &str) -> i64 {
    let items = ["[[2]]", "[[6]]"].into_iter().chain(txt.lines())
        .filter(|l| l.len() > 0).map(|l| list_split(l)).collect::<Vec<Item>>();
    let mut iord = (0..items.len()).collect::<Vec<usize>>();
    iord.sort_unstable_by(|&i,&j| if compare_items(&items[i],&items[j]).unwrap() { Ordering::Less } else { Ordering::Greater });
    ((iord.iter().position(|&v| v == 0).unwrap() + 1) *
        (iord.iter().position(|&v| v == 1).unwrap() + 1)) as i64
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
