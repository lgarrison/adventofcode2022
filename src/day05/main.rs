use std::fs;

const N: usize = 9;

#[derive(Debug)]
#[derive(Default)]
struct Stacks([Vec<char>;N]);

#[derive(Debug)]
struct Instr(i64,i64,i64);

impl Stacks {
    fn init(lines: Vec<&str>) -> Self {
        let mut new: Stacks = Default::default();

        for &line in lines.iter().rev().skip(1) {
            for i in 0..N {
                let j = 1 + 4*i;
                if let Some(c) = line.chars().nth(j) {
                    if c != ' ' {
                        new.0[i].push(c);   // TODO how to make nicer?
                    }
                }
            }
        }

        new
    }

    fn mv(&mut self, n: usize, from: usize, to: usize, rev: bool) {
        let mut popped = self.0[from].split_off(self.0[from].len()-n);
        if rev {
            popped.reverse();
        }
        self.0[to].append(&mut popped);
    }

    fn top(&self) -> String {
        self.0.iter()
        .map(|s| *s.last().unwrap())
        .collect()
    }

}

fn stacks_from_txt(txt: &str) -> (Stacks, Vec<Instr>) {
    let mut parts = txt.split("\n\n");
    let st = Stacks::init(parts.next().unwrap().lines().collect());
    
    let instr_vec: Vec<Vec<i64>> = parts.next().unwrap()
        .lines().map(|l| l.split(' ').enumerate()
            .filter(|(i,_)| [1, 3, 5].contains(i))
            .map(|(_,s)| s.parse::<i64>().unwrap()).collect()
        ).collect();

    let instr: Vec<Instr> = instr_vec.iter()
        .map(|v| Instr(v[0], v[1]-1, v[2]-1)).collect();

    (st, instr)
}

fn part1(txt: &str) -> String {
    let (mut stacks,instr) = stacks_from_txt(txt);

    for i in instr {
        stacks.mv(i.0 as usize, i.1 as usize, i.2 as usize, true);
    }
    
    stacks.top()
}

fn part2(txt: &str) -> String {
    let (mut stacks,instr) = stacks_from_txt(txt);

    for i in instr {
        stacks.mv(i.0 as usize, i.1 as usize, i.2 as usize, false);
    }
    
    stacks.top()
}

#[allow(non_snake_case)]
fn main() {
    let dayX = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    let path = String::from(root) + "/src/" + dayX + "/input.txt";
    //let path = String::from(root) + "/src/" + dayX + "/test1.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {:?}", dayX);
    println!("Part 1: {:?}", part1(&txt));
    println!("Part 2: {:?}", part2(&txt));
}
