#[allow(unused_imports)]
use crate::utils::prelude::*;

pub fn run(input: &String) {
    let mut sum = 0i32;
    let mut multiply = true;

    let re = Regex::new(r"mul\(\d+,\d+\)|(do|don't)\(\)").unwrap();
    let matches = re.find_iter(input).map(|x| x.as_str());

    for item in matches {
        if item.chars().nth(0).unwrap() == 'm' {
            if multiply {
                let mut nums = item.split(',')
                    .map(|x| 
                        x.chars()
                            .filter(|c| c.is_digit(10))
                            .collect::<String>()
                            .parse::<i32>()
                            .unwrap()
                    );
                sum += nums.next().unwrap() * nums.next().unwrap();
            }
        } else {
            multiply = item.len() < 5;
        }
    }

    println!("{}", sum);
}