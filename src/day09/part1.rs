#![allow(non_snake_case)]

use std::fs;

const N: usize = 1000;

fn part1(txt: &str) -> i64 {
    let mut grid: Vec<bool> = vec![false;N*N];

    let (mut Hx, mut Hy) = (N/2,N/2);
    let (mut Tx, mut Ty) = (N/2,N/2);
    grid[Tx*N + Ty] = true;
    for line in txt.lines() {
        let mut parts = line.split_whitespace();
        let dir = parts.next().unwrap();
        let num = parts.next().unwrap().parse::<usize>().unwrap();

        match dir {
            "U" => {
                Hy += num;
            }
            "D" => {
                Hy -= num;
            }
            "L" => {
                Hx -= num;
            }
            "R" => {
                Hx += num;
            }
            _ => {
                panic!("unknown dir!");
            }
        }

        if (Tx != Hx && (Ty as isize - Hy as isize).abs() > 1isize) ||
           (Ty != Hy && (Tx as isize - Hx as isize).abs() > 1isize) {
            Tx = if Tx < Hx { Tx+1 } else { Tx-1 };
            Ty = if Ty < Hy { Ty+1 } else { Ty-1 };
            grid[Tx*N + Ty] = true;
            assert!(Tx == Hx || Ty == Hy);
        }

        if (Tx as isize - Hx as isize).abs() > 1isize {
            let newTx = if Tx < Hx { Hx-1 } else { Hx+1 };
            for i in (Tx..=newTx).chain(newTx..=Tx) {
                grid[i*N + Ty] = true;
            }
            Tx = newTx;
        }

        if (Ty as isize - Hy as isize).abs() > 1isize {
            let newTy = if Ty < Hy { Hy-1 } else { Hy+1 };
            for j in (Ty..=newTy).chain(newTy..=Ty) {
                grid[Tx*N + j] = true;
            }
            Ty = newTy;
        }

        // println!("tail: ({:?},{:?}); head: ({:?},{:?})", Tx, Ty, Hx, Hy);

        // for j in (0..N).rev() {
        //     for i in 0..N {
        //         print!("{}", if grid[i*N + j] { "#" } else { "." } );
        //     }
        //     println!();
        // }

        // println!("---------------");
    }
    
    grid.iter().map(|&b| b as i64).sum()
}

fn part2(_txt: &str) -> i64 {
    0
}

#[allow(non_snake_case)]
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
