use std::fs;

fn part1(txt: &str) -> i64 {
    let mut pos = 0usize;
    for line in txt.lines() {
        pos = line.chars().collect::<Vec<char>>().windows(4)
            .position(|win|
                win.iter().all(|c| win.iter().filter(|&d| d == c).count() == 1)
            ).unwrap() + 4;
        println!("{:?}",pos);
    }
    pos as i64
}

fn part2(txt: &str) -> i64 {
    let mut pos = 0usize;
    for line in txt.lines() {
        pos = line.chars().collect::<Vec<char>>().windows(14)
            .position(|win|
                win.iter().all(|c| win.iter().filter(|&d| d == c).count() == 1)
            ).unwrap() + 14;
        println!("{:?}",pos);
    }
    pos as i64
}

#[allow(non_snake_case)]
fn main() {
    let dayX = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    //let path = String::from(root) + "/src/" + dayX + "/test1.txt";
    let path = String::from(root) + "/src/" + dayX + "/input.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", dayX);
    println!("Part 1: {}", part1(&txt));
    println!("Part 2: {}", part2(&txt));
}
