use std::fs;

#[derive(Debug)]
struct Node {
    name: String,
    is_dir: bool,
    size: usize,
    // parent: Option<&'a Node<'a>>,
    children: Vec<Node>,
}

fn txt_to_fs(txt: &str) -> Node {
    let mut tree = Node{name: "/".to_string(),
                        is_dir: true,
                        size: 0,
                        // parent: None,
                        children: Vec::new(),
                    };
    let mut cwd = &mut tree;
    let mut cwdstack: Vec<&mut Node> = Vec::new();
    let mut listing = false;
    for line in txt.lines().skip(1) {
        let mut split = line.split_whitespace();
        match split.next().unwrap() {
            "$" => {
                listing = false;
                match split.next().unwrap() {
                    "cd" => {
                        let target = split.next().unwrap();
                        match target {
                            ".." => {
                                cwd = cwdstack.pop().unwrap();
                            },
                            _ => {
                                cwd.children.push(
                                    Node{name: target.to_string(),
                                        is_dir: true,
                                        size: 0,
                                        // parent: Some(&cwd),
                                        children: Vec::new(),
                                    });
                                    cwdstack.push(cwd);
                                    cwd = cwd.children.last_mut().unwrap();
                            },
                        }
                    }
                    "ls" => listing = true,
                    _ => panic!("unknown command!"),
                }
            }
            info => {
                assert!(listing);
                match info {
                    "dir" => (),
                    sz => {
                        cwd.size += sz.parse::<usize>().unwrap();
                    }
                }
            }
        }
    }

    tree
}


fn make_sizes_cumulative(root: &mut Node) -> usize {
    root.size = root.children.iter_mut().map(
        |c| c.size + make_sizes_cumulative(c)
    ).sum();
    root.size
}

fn p1_size(root: &Node, max: usize) -> usize {
    (if root.size <= max { root.size } else { 0usize }) +
        root.children.iter().map(
            |c| p1_size(c, max))
        .sum::<usize>()
}


fn part1(txt: &str) -> i64 {
    let mut tree = txt_to_fs(txt);
    println!("{:?}",tree);
    make_sizes_cumulative(&mut tree);
    p1_size(&tree, 10000usize) as i64
}

fn part2(_txt: &str) -> i64 {
    0
}

#[allow(non_snake_case)]
fn main() {
    let dayX = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    //let path = String::from(root) + "/src/" + dayX + "/input.txt";
    let path = String::from(root) + "/src/" + dayX + "/test1.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", dayX);
    println!("Part 1: {}", part1(&txt));
    println!("Part 2: {}", part2(&txt));
}
