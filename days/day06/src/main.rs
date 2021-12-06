#![warn(clippy::pedantic)]
use std::env::args;

struct LanternFishes {
    counts: [i64; 9],
}

impl LanternFishes {
    fn new(initial_state: Vec<usize>) -> Self {
        let mut counts = [0; 9];
        for x in initial_state {
            counts[x] += 1;
        }
        LanternFishes {counts}
    }

    fn step(&mut self) {
        let mut next_counts = [0; 9];
        for time in (1..=8).rev() {
            next_counts[time - 1] = self.counts[time];
        }
        // Reset the fish that just reproduced
        next_counts[6] += self.counts[0];
        // Add the new fish
        next_counts[8] += self.counts[0];
        self.counts = next_counts;
    }

    fn len(&self) -> i64 {
        self.counts.iter().sum()
    }
}

fn parse_input() -> LanternFishes {
    let std_in = std::io::stdin();
    let mut input = String::new();
    std_in.read_line(&mut input).expect("Parsing failed");
    let initial_state = input.split(',').map(|x|  x.parse().unwrap() ).collect();
    LanternFishes::new(initial_state)
}

fn simulate(n_days: i32) {
    let mut state = parse_input();
    for _day in 1..=n_days {
        state.step();
    }
    println!("Ended with {} fish", state.len());
}

fn part_one() {
    let n_days = 80;
    simulate(n_days);
}

fn part_two() {
    let n_days = 256;
    simulate(n_days);
}


fn main() {
    match args().last().unwrap().as_str() {
        "one" => part_one(),
        "two" => part_two(),
        _ => (),
    }
}