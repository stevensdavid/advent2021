#![warn(clippy::pedantic)]
use std::env::args;

fn parse_input() -> Vec<i32> {
    let std_in = std::io::stdin();
    let mut input = String::new();
    std_in.read_line(&mut input).expect("Failed to read input.");
    input.split(",").map(|x| x.parse().unwrap()).collect()
}

fn cheapest_cost(input: Vec<i32>, fuel_cost: fn(i32, i32) -> i32) -> i32 {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();
    (min..max)
        .map(|place| input.iter().map(|x| fuel_cost(*x, place)).sum())
        .min()
        .unwrap()
}

fn part_one() {
    let input = parse_input();
    let fuel_cost = |a: i32, b: i32| (a - b).abs();
    println!("The minimum cost is {}", cheapest_cost(input, fuel_cost));
}

fn part_two() {
    let input = parse_input();
    let fuel_cost = |a: i32, b: i32| {
        let n_steps = (a-b).abs();
        n_steps * (n_steps + 1) / 2
    };
    println!("The minimum cost is {}", cheapest_cost(input, fuel_cost));
}

fn main() {
    match args().last().unwrap().as_str() {
        "one" => part_one(),
        "two" => part_two(),
        _ => (),
    }
}
