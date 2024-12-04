#[allow(unused_imports)]
use crate::utils::prelude::*;

pub fn run(input: &String) {
    let data: Vec<Vec<_>> = input.lines()
        .map(|line| line.chars().collect()).collect();

    let surroundings: Vec<(i32,i32)> = vec![(1,0),(1,-1),(0,-1),(-1,-1),(-1,0),(-1,1),(0,1),(1,1)];
    let order: Vec<char> = vec!['M','A','S'];

    let is_valid_sequence = |line_index: usize, char_index: usize, diff: &(i32, i32)| -> bool {
        order.iter().enumerate().all(|(i, next_char)| {
            let new_line_index = line_index as i32 + (i + 1) as i32 * diff.0;
            let new_char_index = char_index as i32 + (i + 1) as i32 * diff.1;
            data.get(new_line_index as usize)
                .and_then(|line| line.get(new_char_index as usize))
                == Some(next_char)
        })
    };

    let mut total = 0;

    for (i, line) in data.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if char.eq(&'X') {
                for diff in surroundings.iter() {
                    if is_valid_sequence(i, j, diff) {
                        total += 1;
                    }
                }
            }
        }
    }

    println!("{}", total);
}