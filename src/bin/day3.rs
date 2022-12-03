use std::collections::HashSet;
use itertools::Itertools; // https://docs.rs/itertools/latest/itertools/s

fn priority(x: char) -> u64 {
    // return 1-26 for a-z and 27-52 for A-Z
    let num: u64 = x.into(); 
    if num <= 90 {
        num - 38
    } else {
        num - 96
    }
}

fn rucksack_priority(sack: &str) -> u64 {
    let half = sack.len() / 2;
    let mut total: u64 = 0;

    //insert character into a set
    let mut set = HashSet::new();
    let mut counted = HashSet::new();

    for (i, x) in sack.chars().enumerate() {
        if i < half {
            set.insert(x);
        } else {
            if set.contains(&x) && !counted.contains(&x) {
                //println!("  {} value {}", x, priority(x));
                total += priority(x);
                counted.insert(x);
            }
        }
    }
    //println!("{sack} total {total}");
    total
}

fn part_1(path: &str) {
    let s = std::fs::read_to_string(path)
        .expect("can't read file");

    let result: u64 = s.lines()
        .map(rucksack_priority)
        .sum();

    println!("result: {result}");
}

fn common_item_in_group(group: Vec<&str>) -> u64 {
    let sets:  Vec<HashSet<char>> = group.into_iter()
        .map(|line| 
            line.chars().collect::<HashSet<_>>()
        )
        .collect();

    let intersection = sets.iter()
        .skip(1)
        .fold(sets[0].clone(), |acc, set| {
            acc.intersection(&set).cloned().collect()
        });

    let element = intersection.iter().next().unwrap();
    priority(*element)
}

fn part_2(path: &str) {
    let s = std::fs::read_to_string(path).expect("can't read file");

    let result: u64 = s.lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            common_item_in_group(chunk.collect_vec())
        })
        .sum();

    println!("result: {result}");
}

fn main() {
    let path = "data/day3.txt";
    part_1(path);
    part_2(path);
}
