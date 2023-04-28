use std::{borrow::Borrow, str::Chars};

const PRIORITY: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub struct Backbag {
    container1: Chars,
    container2: Chars,
}

#[aoc_generator(day3)]
pub fn generate_day3(input: &String) -> Vec<Backbag> {
    input
        .lines()
        .map(|l| {
            let containers = l.split_at(l.len() / 2);
            Backbag {
                container1: containers.0.to_string().chars(),
                container2: containers.1.to_string().chars(),
            }
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: Vec<Backbag>) -> u32 {
    0
}
