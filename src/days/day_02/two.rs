#[allow(unused_imports)]
use crate::utils::prelude::*;

pub fn run(input: &str) {
    println!("{}",
        input
        .lines()
        .filter(|line| {
            let numbers: Vec<i32> = line.split_whitespace().map(|str| str.parse::<i32>().unwrap()).collect();

            let num_windows: Vec<Vec<(i32,i32)>> =
                (0..(line.len()))
                    .map(|i|
                        numbers.iter().enumerate().filter_map(|(j,num)| if i == j { None } else { Some(num.clone()) })
                            .tuple_windows::<(_,_)>()
                            .collect()
                   ).collect();
                

            num_windows
                .iter()
                .any(|permutation|
                    permutation.iter().map(|(x,y)| (x - y).abs()).all(|num| num < 4)
                    &&
                    (
                        permutation.iter().all(|(x,y)| x > y)
                        ||
                        permutation.iter().all(|(x,y)| x < y)
                    )
                )
        }).count()
    );
}