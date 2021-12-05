#![warn(clippy::pedantic)]
use std::{env::args, io::stdin, io::BufRead};

#[derive(Default, Clone, Debug)]
struct Accumulator {
    count: i32,
    freqs: Vec<i32>,
}

fn bits_to_int(bits: &[i32]) -> i32 {
    let mut acc: i32 = 0;
    for (i, bit) in bits.iter().rev().enumerate() {
        acc += bit << i;
    }
    acc
}

impl Accumulator {
    fn gamma_bits(&self) -> Vec<i32> {
        let threshold = self.count / 2;
        let mut bits = Vec::new();
        for freq in &self.freqs {
            bits.push((freq > &threshold) as i32);
        }
        bits
    }

    fn gamma(&self) -> i32 {
        let bits = self.gamma_bits();
        bits_to_int(&bits)
    }

    fn epsilon(&self) -> i32 {
        // Extract only the relevant bits after flipping
        let gamma_bits = self.gamma_bits();
        let epsilon_bits: Vec<i32> = gamma_bits.iter().map(|x| !x & 0x1).collect();
        bits_to_int(&epsilon_bits)
    }
}

fn part_one() {
    let stdin = stdin();
    let lines = stdin.lock().lines();
    let mut acc = Accumulator::default();
    for line in lines.flatten() {
        acc.count += 1;
        let line_len = line.len();
        while acc.freqs.len() < line_len {
            acc.freqs.push(0);
        }
        for (i, char) in line.chars().enumerate() {
            if char == '1' {
                acc.freqs[i] += 1;
            }
        }
    }
    let gamma = acc.gamma();
    let epsilon = acc.epsilon();
    println!(
        "Processed {} lines. Gamma: {} Epsilon: {} Product: {}",
        acc.count,
        gamma,
        epsilon,
        gamma * epsilon,
    );
}

fn most_common_digit(digits: &[char]) -> char {
    let n_ones = digits
        .iter()
        .fold(0, |acc, c| if *c == '1' { acc + 1 } else { acc });
    let n_zeroes = digits.len() - n_ones;
    if n_ones >= n_zeroes {
        '1'
    } else {
        '0'
    }
}

fn find_most_common_digit(strings: Vec<String>, bit_idx: usize) -> char {
    let digits: Vec<char> = strings
                .iter()
                .map(|x| x.chars().nth(bit_idx).unwrap())
                .collect();
    most_common_digit(&digits)
}

fn part_two() {
    let std_in = stdin();
    let lines: Vec<String> = std_in
        .lock()
        .lines()
        .map(std::result::Result::unwrap)
        .collect();
    let mut o2_candidates = lines.clone();
    let mut co2_candidates = lines.clone();
    for bit_idx in 0..lines[0].len() {
        if o2_candidates.len() == 1 && co2_candidates.len() == 1 {
            break;
        }
        let f_filter = |x: &&String, c: char| x.chars().nth(bit_idx).unwrap() == c;
        if o2_candidates.len() > 1 {
            // this clone is probably unnecessary, but I don't have time to figure it out
            let o2_char = find_most_common_digit(o2_candidates.clone(), bit_idx);
            o2_candidates = o2_candidates
                .iter()
                .filter(|x| f_filter(x, o2_char))
                .cloned()
                .collect();
        }
        if co2_candidates.len() > 1 {
            let o2_char = find_most_common_digit(co2_candidates.clone(), bit_idx);
            let co2_char = if o2_char == '0' { '1' } else { '0' };
            co2_candidates = co2_candidates
                .iter()
                .filter(|x| f_filter(x, co2_char))
                .cloned()
                .collect();
        }
    }
    assert!(o2_candidates.len() == 1 && co2_candidates.len() == 1);
    let co2_rating = i32::from_str_radix(co2_candidates[0].as_str(), 2).unwrap();
    let o2_rating = i32::from_str_radix(o2_candidates[0].as_str(), 2).unwrap();
    let product = co2_rating * o2_rating;
    println!(
        "O2 rating: {} CO2 rating: {} Product: {}",
        o2_rating, co2_rating, product
    );
}

fn main() {
    match args().last().unwrap().as_str() {
        "one" => part_one(),
        "two" => part_two(),
        _ => (),
    }
}
