#![warn(clippy::pedantic)]
use std::io::{self, BufRead};

fn read_input() -> Vec<i32> {
    let mut depths: Vec<i32> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(data) => if let Ok(depth) = data.parse::<i32>() {
                depths.push(depth);
            },
            Err(error) => println!("Error: {}", error),
        }
    }
    depths
}

fn count_increases(it: &[i32]) -> i32 {
    let mut increases = 0;
    for idx in 1..it.len() {
        if it[idx] > it[idx - 1] {
            increases += 1;
        }
    }
    increases
}

fn part_one() {
    let depths = read_input();
    let increases = count_increases(&depths);
    println!("There were {} increases", increases);
}

fn part_two() {
    let depths = read_input();
    let sums: Vec<i32> = depths
        .windows(3)
        .map(|window| window.iter().sum::<i32>())
        .collect();
    let increases = count_increases(&sums);
    println!("There were {} increases", increases);
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("one") => part_one(),
        Some("two") => part_two(),
        _ => println!("Usage: cargo run one|two"),
    }
}
