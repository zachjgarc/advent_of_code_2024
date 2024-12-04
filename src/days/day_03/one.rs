#[allow(unused_imports)]
use crate::utils::prelude::*;

pub fn run(input: &String) {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    println!("{:?}", re.find_iter(input).map(|instance| instance.as_str().split(",").fold(1i32, |a,b| { a * b.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<i32>().unwrap() })).sum::<i32>());
}