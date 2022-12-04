use std::fs;
use std::ops::Range;

trait CompareRange {
    fn contains_range(&self, other: &Range<i64>) -> bool;
    fn overlaps_range(&self, other: &Range<i64>) -> bool;
}

impl CompareRange for Range<i64> {
    // does `self` contain `other`?
    fn contains_range(&self, other: &Range<i64>) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps_range(&self, other: &Range<i64>) -> bool {
        (self.start <= other.start && other.start <= self.end) ||
        (self.start <= other.end && other.end <= self.end)
    }
}

fn part1(txt: &str) -> i64 {
    let mut count = 0;
    for line in txt.lines() {
        let halves: Vec<Vec<i64>> = line.split(',').map(|s| s.split('-').map(|d| d.parse::<i64>().unwrap())
                .collect()).collect();
        assert!(halves.len() == 2);
        let ranges: Vec<Range<i64>> = halves.iter()
            .map(|h| h[0]..h[1]).collect();
        if ranges[0].contains_range(&ranges[1]) ||
            ranges[1].contains_range(&ranges[0]) {
                count += 1;
            }
    }
    count
}

fn part2(txt: &str) -> i64 {
    let mut count = 0;
    for line in txt.lines() {
        let halves: Vec<Vec<i64>> = line.split(',').map(|s| s.split('-').map(|d| d.parse::<i64>().unwrap())
                .collect()).collect();
        assert!(halves.len() == 2);
        let ranges: Vec<Range<i64>> = halves.iter()
            .map(|h| h[0]..h[1]).collect();
        if ranges[0].overlaps_range(&ranges[1]) ||
            ranges[1].overlaps_range(&ranges[0]) {
                count += 1;
            }
    }
    count
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
