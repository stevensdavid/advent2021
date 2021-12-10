use std::{env::args, io::BufRead, };

fn part_one() {
    let std_in = std::io::stdin();
    let lines = std_in.lock().lines();
    let segment_counts = [2, 3, 4, 7];
    let count: usize = lines
        .map(|l| {
            l.unwrap()
                .split(" | ")
                .nth(1)
                .unwrap()
                .split(" ")
                .filter(|x| segment_counts.contains(&x.len()))
                .count()
        })
        .sum();
    println!("There are {} such numbers", count);
}

enum LineSegment {
    A,
    B,
    C,
    D,
    E,
    F,
    G
}

fn number(segments: Vec<LineSegment>) -> Result<i32, ()> {
    // The line segment mapping is:
    //     0:      1:      2:      3:      4:
    //     aaaa    ....    aaaa    aaaa    ....
    //    b    c  .    c  .    c  .    c  b    c
    //    b    c  .    c  .    c  .    c  b    c
    //     ....    ....    dddd    dddd    dddd
    //    e    f  .    f  e    .  .    f  .    f
    //    e    f  .    f  e    .  .    f  .    f
    //     gggg    ....    gggg    gggg    ....
    
    //      5:      6:      7:      8:      9:
    //     aaaa    aaaa    aaaa    aaaa    aaaa
    //    b    .  b    .  .    c  b    c  b    c
    //    b    .  b    .  .    c  b    c  b    c
    //     dddd    dddd    ....    dddd    dddd
    //    .    f  e    f  .    f  e    f  .    f
    //    .    f  e    f  .    f  e    f  .    f
    //     gggg    gggg    ....    gggg    gggg
    match segments.sort()[..] {
        
        _ => return Err(()),
    }
}

fn part_two() {
    todo!()
}

fn main() {
    match args().nth(1).unwrap().as_str() {
        "one" => part_one(),
        "two" => part_two(),
        _ => (),
    }
}
