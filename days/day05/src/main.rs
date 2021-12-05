#![warn(clippy::pedantic)]
use regex::Regex;
use std::{
    cmp::{max, min},
    collections::HashMap,
    env::args,
    hash::Hash,
    io::BufRead,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct VentLine {
    start: Point,
    end: Point,
}

fn parse_input() -> Vec<VentLine> {
    let std_in = std::io::stdin();
    let input_lines = std_in.lock().lines();
    let mut vent_lines: Vec<VentLine> = Vec::new();
    let re = Regex::new(r"^(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)$").unwrap();
    for line in input_lines {
        let vent_line = line.unwrap();
        let captures = re.captures(vent_line.as_str()).unwrap();
        let x1 = captures.name("x1").unwrap().as_str().parse().unwrap();
        let y1 = captures.name("y1").unwrap().as_str().parse().unwrap();
        let x2 = captures.name("x2").unwrap().as_str().parse().unwrap();
        let y2 = captures.name("y2").unwrap().as_str().parse().unwrap();
        vent_lines.push(VentLine {
            start: Point { x: x1, y: y1 },
            end: Point { x: x2, y: y2 },
        });
    }
    vent_lines
}

fn part_one() {
    let input = parse_input();
    let vent_lines = input
        .iter()
        .filter(|l| l.start.x == l.end.x || l.start.y == l.end.y);
    let mut vent_points: HashMap<Point, i32> = HashMap::new();
    for line in vent_lines {
        // This works because we know that either X or Y does not change.
        // We need to iterate from lower values to higher values.
        let (start_x, end_x) = (min(line.start.x, line.end.x), max(line.start.x, line.end.x));
        let (start_y, end_y) = (min(line.start.y, line.end.y), max(line.start.y, line.end.y));
        for x in start_x..=end_x {
            for y in start_y..=end_y {
                let point = Point { x, y };
                let count = vent_points.entry(point).or_insert(0);
                *count += 1;
            }
        }
    }
    let overlap_count = vent_points.values().filter(|x| **x > 1).count();
    println!("There are {} overlaps.", overlap_count);
}

fn part_two() {
    let vent_lines = parse_input();
    let mut vent_points: HashMap<Point, i32> = HashMap::new();
    for line in vent_lines {
        if line.start.x != line.end.x && line.start.y != line.end.y {
            let n_steps = (line.end.x - line.start.x).abs();
            let x_stride: i32 = (line.end.x - line.start.x).signum();
            let y_stride: i32 = (line.end.y - line.start.y).signum();
            for step in 0..=n_steps {
                let point = Point {x: line.start.x + step * x_stride, y: line.start.y + step * y_stride};
                let count = vent_points.entry(point).or_insert(0);
                *count += 1;
            }
        } else {
            // This works because we know that either X or Y does not change.
            // We need to iterate from lower values to higher values.
            let (start_x, end_x) = (min(line.start.x, line.end.x), max(line.start.x, line.end.x));
            let (start_y, end_y) = (min(line.start.y, line.end.y), max(line.start.y, line.end.y));
            for x in start_x..=end_x {
                for y in start_y..=end_y {
                    let point = Point { x, y };
                    let count = vent_points.entry(point).or_insert(0);
                    *count += 1;
                }
            }
        }
    }
    let overlap_count = vent_points.values().filter(|x| **x > 1).count();
    println!("There are {} overlaps.", overlap_count);
}

fn main() {
    match args().last().unwrap().as_str() {
        "one" => part_one(),
        "two" => part_two(),
        _ => (),
    }
}
