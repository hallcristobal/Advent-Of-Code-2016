extern crate regex;
use regex::Regex;
use std::collections::HashSet;

const INPUT: &str = "R3, R1, R4, L4, R3, R1, R1, L3, L5, L5, L3, R1, R4, L2, L1, R3, L3, R2, R1, R1, L5, L2, L1, R2, L4, R1, L2, L4, R2, R2, L2, L4, L3, R1, R4, R3, L1, R1, L5, R4, L2, R185, L2, R4, R49, L3, L4, R5, R1, R1, L1, L1, R2, L1, L4, R4, R5, R4, L3, L5, R1, R71, L1, R1, R186, L5, L2, R5, R4, R1, L5, L2, R3, R2, R5, R5, R4, R1, R4, R2, L1, R4, L1, L4, L5, L4, R4, R5, R1, L2, L4, L1, L5, L3, L5, R2, L5, R4, L4, R3, R3, R1, R4, L1, L2, R2, L1, R4, R2, R2, R5, R2, R5, L1, R1, L4, R5, R4, R2, R4, L5, R3, R2, R5, R3, L3, L5, L4, L3, L2, L2, R3, R2, L1, L1, L5, R1, L3, R3, R4, R5, L3, L5, R1, L3, L5, L5, L2, R1, L3, L1, L3, R4, L1, R3, L2, L2, R3, R3, R4, R4, R1, L4, R1, L5";

fn main() {
    let mut dir = 0;
    let mut ns = 0;
    let mut ew = 0;
    let mut cache = HashSet::new();

    let r = Regex::new(r"([LR])([0-9]+)").unwrap();
    'outer: for cap in r.captures_iter(INPUT) {
        match &cap[1] {
            "L" => dir = turn(dir, -1),
            "R" => dir = turn(dir, 1),
            _ => panic!("Unknown Direction"),
        }

        let dist: isize = cap[2].parse().unwrap();
        let (c_ns, c_ew) = dir_to_vec(dir);
        for _ in 0..dist {
            if !cache.insert((ns, ew)) {
                break 'outer;
            }
            ns += c_ns;
            ew += c_ew;
        }
    }

    println!(
        "Direction: {} Distance: {}\nDirection: {} Distance: {}\nTotal Blocks: {}",
        if ew > 0 { "E" } else { "W" },
        ew.abs(),
        if ns > 0 { "N" } else { "S" },
        ns.abs(),
        ew.abs() + ns.abs()
    );
}

fn dir_to_vec(dir: i8) -> (isize, isize) {
    match dir {
        0 => (1, 0),
        1 => (0, 1),
        2 => (-1, 0),
        3 => (0, -1),
        _ => panic!("Unknown Direction"),
    }
}

fn turn(dir: i8, num: i8) -> i8 {
    let dir = dir + num;
    if dir < 0 {
        4 + dir
    } else if dir > 3 {
        dir - 4
    } else {
        dir
    }
}
