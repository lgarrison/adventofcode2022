use std::fs;

fn make_grid(txt: &str) -> (Vec<i8>, usize) {
    let N = txt.lines().next().unwrap().len();
    let mut grid: Vec<i8> = Vec::with_capacity(N*N);

    for line in txt.lines() {
        grid.extend(line.chars()
            .map(|c| c.to_digit(10).unwrap() as i8)
        );
    }

    (grid,N)
}

fn part1(txt: &str) -> i64 {
    let (grid,N) = make_grid(txt);

    let mut vis: Vec<bool> = vec![false;grid.len()];

    for i in 0..N {
        let mut maxheight = -1;
        for j in 0..N {
            if grid[i*N + j] > maxheight {
                vis[i*N + j] = true;
                maxheight = grid[i*N + j];
            }
        }
    }

    for i in 0..N {
        let mut maxheight = -1;
        for j in (0..N).rev() {
            if grid[i*N + j] > maxheight {
                vis[i*N + j] = true;
                maxheight = grid[i*N + j];
            }
        }
    }

    for i in 0..N {
        let mut maxheight = -1;
        for j in 0..N {
            if grid[j*N + i] > maxheight {
                vis[j*N + i] = true;
                maxheight = grid[j*N + i];
            }
        }
    }

    for i in 0..N {
        let mut maxheight = -1;
        for j in (0..N).rev() {
            if grid[j*N + i] > maxheight {
                vis[j*N + i] = true;
                maxheight = grid[j*N + i];
            }
        }
    }

    // println!("{:?}", grid);
    // println!("{:?}", vis);
    
    vis.iter().map(|&b| b as i64).sum()
}

fn part2(txt: &str) -> i64 {
    let (grid,N) = make_grid(txt);

    let mut best = 0;
    for i in 1..N-1 {
        for j in 1..N-1 {
            let h = grid[i*N + j];
            let mut score: i64 = 1;

            let mut s = (1..=j).take_while(|k| grid[i*N + j - k] < h).count();
            if s < j {
                s += 1;
            }
            score *=  s as i64;
            
            s = (1..N-j).take_while(|k| grid[i*N + j + k] < h).count();
            if s < N-j-1 {
                s += 1;
            }
            score *=  s as i64;

            s = (1..=i).take_while(|k| grid[(i-k)*N + j] < h).count();
            if s < i {
                s += 1;
            }
            score *=  s as i64;

            s = (1..N-i).take_while(|k| grid[(i+k)*N + j] < h).count();
            if s < N-i-1 {
                s += 1;
            }
            score *=  s as i64;


            if score > best {
                best = score;
            }
        }
    }

    best
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
