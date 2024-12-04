#[allow(unused_imports)]
use crate::utils::prelude::*;

pub fn run(input: &String) {
    let mut counts = HashMap::<i32,(i32,i32)>::new();

    for line in input.lines() {
        let mut line_data = line.split_whitespace().map(|num| num.parse::<i32>().unwrap());
        counts.entry(line_data.next().unwrap()).or_insert((0,0)).0 += 1;
        counts.entry(line_data.next().unwrap()).or_insert((0,0)).1 += 1;
    }

    let mut sum = 0i32;

    for (num, (left, right)) in counts.iter() {
        sum += num * left * right;
    }
    
    println!("{}", sum);
}