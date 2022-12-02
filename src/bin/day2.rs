#[derive(Debug)]
#[derive(Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

fn score(a: &RPS, b: &RPS) -> u64 {
    match (a, b) {
        (RPS::Rock, RPS::Scissors) => 6,
        (RPS::Paper, RPS::Rock) => 6,
        (RPS::Scissors, RPS::Paper) => 6,
        (RPS::Rock, RPS::Paper) => 0,
        (RPS::Paper, RPS::Scissors) => 0,
        (RPS::Scissors, RPS::Rock) => 0,
        _ => 3,
    }
}

fn points(a: &RPS) -> u64 {
    match a {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    }
}

fn to_beat(a: &RPS) -> RPS {
    match a {
        RPS::Rock => RPS::Paper,
        RPS::Paper => RPS::Scissors,
        RPS::Scissors => RPS::Rock,
    }
}

fn to_lose(a: &RPS) -> RPS {
    match a {
        RPS::Rock => RPS::Scissors,
        RPS::Paper => RPS::Rock,
        RPS::Scissors => RPS::Paper,
    }
}

fn to_rps(s: &str) -> RPS {
    match s {
        "A" | "X" => RPS::Rock,
        "B" | "Y" => RPS::Paper,
        "C" | "Z" => RPS::Scissors,
        _ => panic!("invalid input"),
    }
}

fn player_move(s: &str, opponent: RPS) -> RPS {
    match s {
        "X" => to_lose(&opponent),
        "Y" => opponent,
        "Z" => to_beat(&opponent),
        _ => panic!("invalid input"),
    }
}

fn part_1(path: &str) {
    let s = std::fs::read_to_string(path)
        .expect("can't read file");

    let score = s.lines()
        .map(|line| {
            let plays = line.split(" ").map(to_rps).collect::<Vec<RPS>>();
            score(&plays[1], &plays[0]) + points(&plays[1])
        })
        .sum::<u64>();

    println!("result: {score}");
}

fn part_2(path: &str) {
    let s = std::fs::read_to_string(path)
        .expect("can't read file");

    let score = s.lines()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();
            let opponent_move = to_rps(parts[0]);
            let player_move = player_move(parts[1], opponent_move);

            score(&player_move, &opponent_move) + points(&player_move)
        })
        .sum::<u64>();
    
    println!("result: {score}");
}

fn main() {
    let path = "data/day2.txt";
    part_1(path);
    part_2(path);
}
