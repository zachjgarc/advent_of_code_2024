#[allow(unused_imports)]
use crate::utils::prelude::*;

pub fn run(input: &str) {
    println!("{}",
        input
        .lines()
        .filter(|line| {
            let num_windows: Vec<(i32,i32)> = line
                .split_whitespace()
                .map(|str| str.parse::<i32>().unwrap())
                .tuple_windows::<(_,_)>()
                .collect();

            num_windows
                .iter()
                .map(|(x,y)| (x - y).abs())
                .all(|num| num < 4)
            &&
            (
                num_windows
                    .iter()
                    .all(|(x,y)| x > y)
                ||
                num_windows
                    .iter()
                    .all(|(x,y)| x < y)
            )
        }).count()
    );
}