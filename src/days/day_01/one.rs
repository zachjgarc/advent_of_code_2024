#[allow(unused_imports)]
use crate::utils::prelude::*;

pub fn run(input: &String) {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        let mut line_data = line.split_whitespace().map(|num| num.parse::<i32>().unwrap());
        left_list.push(line_data.next().unwrap());
        right_list.push(line_data.next().unwrap());
    }

    left_list.sort();
    right_list.sort();

    let sum: i32 = left_list.iter().zip(right_list.iter())
        .map(|(left, right)| (left - right).abs()).sum();

    println!("{}", sum);
}