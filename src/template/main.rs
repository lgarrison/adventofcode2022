use std::fs;

fn part1(txt: String) -> u64 {
    0
}

fn part2(txt: String) -> u64 {
    0
}

#[allow(non_snake_case)]
fn main() {
    let dayX = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    let path = root.to_owned() + "/src/" + dayX + "/input.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", dayX);
    println!("Part 1: {}", part1(txt));
    //println!("Part 2: %d", part2(txt));
}
