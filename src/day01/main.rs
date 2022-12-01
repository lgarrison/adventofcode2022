use std::{fs, str::Lines};

fn get_calories(lines: Lines) -> Vec<i64> {
    let mut this: i64 = -1;
    let mut vec = Vec::new();

    for line in lines {
        if let Ok(int) = line.parse::<i64>() {
            assert!(int != -1);
            if this == -1 {
                this = 0;
            }
            this += int
        } else {
            vec.push(this);
            this = -1;
        }
    }

    if this != -1 {
        vec.push(this);
    }

    vec
}

fn part1(txt: &str) -> i64 {
    let vec = get_calories(txt.lines());
    *vec.iter().max().unwrap()
}

fn part2(txt: &str) -> i64 {
    let mut calories = get_calories(txt.lines());
    calories.sort();
    calories.iter().rev().take(3).sum()

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
