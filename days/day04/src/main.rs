#![warn(clippy::pedantic)]

use std::{
    collections::HashSet,
    env::args,
    io::{stdin, BufRead},
};

#[derive(Debug, Clone)]
struct BingoBoard {
    digits: [i32; 5 * 5],
    hits: HashSet<i32>,
}

impl BingoBoard {
    fn new(lines: &str) -> Self {
        let numbers: Vec<i32> = lines
            .split(' ')
            .map(|s| s.trim().parse().unwrap())
            .collect();
        BingoBoard {
            digits: numbers.try_into().unwrap(),
            hits: HashSet::new(),
        }
    }

    fn mark_digit(&mut self, d: i32) -> Option<i32> {
        self.hits.insert(d);
        if let Some(sum) = self.score() {
            return Some(d * sum);
        }
        None
    }

    fn score(&self) -> Option<i32> {
        // Naive implementation, fast enough for AOC
        let mut victory = false;
        for start_idx in 0..5 {
            // Check row for match
            let row_match = (0..5).all(|col_idx| {
                self.hits
                    .contains(&self.digits[BingoBoard::convert_idx(col_idx, start_idx)])
            });
            let col_match = (0..5).all(|row_idx| {
                self.hits
                    .contains(&self.digits[BingoBoard::convert_idx(start_idx, row_idx)])
            });
            if row_match || col_match {
                victory = true;
                break;
            }
        }
        if victory {
            let unmarked_numbers = self.digits.iter().filter(|x| !self.hits.contains(x));
            return Some(unmarked_numbers.sum());
        }
        None
    }

    fn convert_idx(col: usize, row: usize) -> usize {
        row * 5 + col
    }
}

fn parse_input() -> (Vec<i32>, Vec<BingoBoard>) {
    let std_in = stdin();
    let lines = std_in
        .lock()
        .lines()
        .map(|s| format!("{}\n", s.unwrap()))
        .collect::<String>();
    let groups: Vec<&str> = lines.split("\n\n").collect();
    let draws: Vec<i32> = groups[0]
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let boards: Vec<BingoBoard> = groups[1..]
        .iter()
        .map(|s| BingoBoard::new(s.trim().replace('\n', " ").replace("  ", " ").as_str()))
        .collect();
    (draws, boards)
}

fn part_one() {
    let (draws, mut boards) = parse_input().clone();
    let (mut max_score, mut max_idx) = (0, 0);
    let mut winner_found = false;
    for draw in draws {
        for board_idx in 0..boards.len() {
            if let Some(score) = boards[board_idx].mark_digit(draw) {
                winner_found = true;
                if score > max_score {
                    max_score = score;
                    max_idx = board_idx;
                }
            }
        }
        if winner_found {
            break;
        }
    }
    println!("Winner is board {} with score: {}", max_idx, max_score);
}

fn part_two() {
    let (draws, mut boards) = parse_input().clone();
    let (mut last_score, mut last_winner) = (0, 0);
    let candidates: HashSet<usize> = (0..boards.len()).collect();
    let mut winners = HashSet::new();
    for draw in draws {
        let mut iteration_winners = HashSet::new();
        for board_idx in candidates.difference(&winners) {
            if let Some(score) = boards[*board_idx].mark_digit(draw) {
                iteration_winners.insert(*board_idx);
                last_score = score;
                last_winner = *board_idx;
            }
        }
        winners.extend(iteration_winners);
    }
    println!("Last winner is board {} with score: {}", last_winner, last_score);
}

fn main() {
    match args().last().unwrap().as_str() {
        "one" => part_one(),
        "two" => part_two(),
        _ => (),
    }
}
