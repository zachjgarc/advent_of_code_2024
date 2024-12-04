#[allow(unused_imports)]
use crate::utils::prelude::*;

pub fn run(input: &String) {
    let data: Vec<Vec<_>> = input.lines()
    .map(|line| line.chars().collect()).collect();

    let cross_sections: Vec<(i32,i32)> = vec![(1,1),(-1,-1),(1,-1),(-1,1)];

    let is_valid_sequence = |line_index: usize, char_index: usize| -> bool {
        let mut cs = cross_sections.iter().map(|diff|
            data
                .get((line_index as i32 + diff.0) as usize)
                .and_then(|new_line| new_line.get((char_index as i32 + diff.1) as usize))
        ).filter(|c| c.eq(&Some(&'M')) || c.eq(&Some(&'S')));
        cs.clone().count() == 4 && (cs.next() != cs.next() && cs.next() != cs.next())
    };


    let mut total = 0;


    for (i, line) in data.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if char.eq(&'A') {
                if is_valid_sequence(i, j) {
                    total += 1;
                }
            }
        }
    }


    println!("{}", total);
}