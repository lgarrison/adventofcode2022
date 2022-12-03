use std::fs;

use std::collections::HashSet;

fn split_rucksack(line: &str, sorted: bool) -> (Vec<char>,Vec<char>) {
    let L = line.len();
    let mut c = line.chars();
    let iter = c.by_ref();
    let mut comp1: Vec<char> = iter.take(L/2).collect();
    let mut comp2: Vec<char> = iter.take(L/2).collect();

    if sorted {
        comp1.sort();
        comp2.sort();
    }
    (comp1, comp2)
}

fn set_intersect(v1: &Vec<char>, v2: &Vec<char>) -> Vec<char> {
    let s1: HashSet<char> = HashSet::from_iter(v1.iter().cloned());
    let s2: HashSet<char> = HashSet::from_iter(v2.iter().cloned());

    s1.intersection(&s2).into_iter().map(|i| *i).collect()
}

fn score(c: char) -> i64 {
    if c < 'a' {
        c as i64 - 'A' as i64 + 27
    } else {
        c as i64 - 'a' as i64 + 1
    }
}


fn part1(txt: &str) -> i64 {
    let mut tot = 0;
    for line in txt.lines() {
        let (comp1,comp2) = split_rucksack(line, false);
        let common = set_intersect(&comp1, &comp2);
        // println!("{:?}", comp1);
        // println!("{:?}", comp2);
        // println!("{:?}", common);
        assert!(common.len() == 1);
        tot += score(common[0]);
    }

    tot
}


fn part2(txt: &str) -> i64 {
    let lines: Vec<Vec<char>> = txt.lines().map(|l| l.chars().collect()).collect();
    
    let scores = lines.chunks(3)
        .map(|g| score(set_intersect(&set_intersect(&g[0], &g[1]), &g[2])[0]))
        ;

    scores.sum()
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
