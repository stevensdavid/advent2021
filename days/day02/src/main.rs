#![warn(clippy::pedantic)]
use std::str::FromStr;
use std::io::BufRead;

#[derive(Debug)]
enum Direction {
    Forward,
    Up,
    Down
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    distance: i32
}

impl FromStr for Command {   
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let direction = match split.next().unwrap() {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => return Err(())
        };
        let distance: i32 = split.next().unwrap().parse().unwrap();
        Ok(Command { direction, distance })
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
            Command {direction: Direction::Forward, distance} => position.0 += distance,
            Command {direction: Direction::Up, distance} => position.1 -= distance,
            Command {direction: Direction::Down, distance} => position.1 += distance,
        }
    }
    let product = position.0 * position.1;
    println!("Final position: ({0}, {1}) Product is: {2}", position.0, position.1, product);
}

fn part_two() {
    let commands = parse_input();
    let mut position = (0, 0, 0);
    for command in commands {
        match command {
            Command {direction: Direction::Forward, distance} => {
                position.0 += distance;
                position.1 += distance * position.2;
            },
            Command {direction: Direction::Up, distance} => {
                position.2 -= distance;
            },
            Command {direction: Direction::Down, distance} => {
                position.2 += distance;
            },
        }
    }
    let product = position.0 * position.1;
    println!("Final position: ({0}, {1}) Product is: {2}", position.0, position.1, product);
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("one") => part_one(),
        Some("two") => part_two(),
        _ => println!("Usage: cargo run one|two"),
    }
}
