#![warn(clippy::pedantic)]
use std::str::FromStr;
use std::io::BufRead;

#[derive(Debug)]
enum Direction {
    FORWARD,
    UP,
    DOWN
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    distance: i32
}

impl FromStr for Command {   
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");
        let direction = match split.next().unwrap() {
            "forward" => Direction::FORWARD,
            "up" => Direction::UP,
            "down" => Direction::DOWN,
            _ => return Err(())
        };
        let distance: i32 = split.next().unwrap().parse().unwrap();
        Ok(Command { direction: direction, distance: distance })
    }
}

fn parse_input() -> Vec<Command> {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    lines.map(|s| s.unwrap().parse::<Command>().unwrap()).collect()
}

fn part_one() {
    let commands = parse_input();
    let mut position = (0,0);
    for command in commands {
        match command {
            Command {direction: Direction::FORWARD, distance} => position.0 += distance,
            Command {direction: Direction::UP, distance} => position.1 -= distance,
            Command {direction: Direction::DOWN, distance} => position.1 += distance,
        }
    }
    let product = position.0 * position.1;
    println!("Final position: ({0}, {1}) Product is: {2}", position.0, position.1, product);
}

fn part_two() {

}

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("one") => part_one(),
        Some("two") => part_two(),
        _ => println!("Usage: cargo run one|two"),
    }
}
