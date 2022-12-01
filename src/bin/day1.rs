use itertools::Itertools; // https://docs.rs/itertools/latest/itertools/s

fn part_2(path: &str, top_n: usize) {
    let result: u64 = std::fs::read_to_string(path)
        .expect("can't read file")
        .split("\n\n")
        .map(|r| {
            r.lines().filter_map(|s| s.parse::<u64>().ok()).sum::<u64>()
        })
        .sorted_unstable()
        .rev()
        .take(top_n)
        .sum();
    println!("result: {result}");
}

fn main() {
    let path = "data/day1.txt";
    part_2(path, 1);
    part_2(path, 3);
}
