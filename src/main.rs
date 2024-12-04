use clap::{Arg, ArgAction, Command};
use std::fs;
use std::{io::{self, Read, Write}, time::Instant};
use std::path::Path;

mod days;
mod utils;

fn main() {
    let matches = Command::new("Advent of Code 2024")
    .version("1.0")
    .author("Your Name <youremail@example.com>")
    .about("Runs Advent of Code solutions")
    .arg(
        Arg::new("day")
            .help("Day number to run")
            .required(false)
            .index(1),
    )
    .arg(
        Arg::new("test")
            .short('t')
            .long("test")
            .help("Use test input")
            .action(ArgAction::SetTrue),
    )
    .subcommand(
        Command::new("add")
            .about("Adds a new day")
            .arg(
                Arg::new("day")
                    .help("Day number to add")
                    .required(true)
                    .index(1),
            ),
    )
    .get_matches();

    if let Some(add_matches) = matches.subcommand_matches("add") {
        let day_str = add_matches
            .get_one::<String>("day")
            .expect("Day number is required");

        let day_num: u32 = match day_str.parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid day number: {}", day_str);
                std::process::exit(1);
            }
        };

        add_new_day(day_num);
    } else if let Some(day_str) = matches.get_one::<String>("day") {
        let day_num: u32 = match day_str.parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid day number: {}", day_str);
                std::process::exit(1);
            }
        };

        let use_test = *matches.get_one::<bool>("test").unwrap_or(&false);
        run_day(day_num, use_test);
    } else {
        eprintln!("Please provide a command. Use `cargo run -- --help` for more information.");
        std::process::exit(1);
    }
}

fn run_day(day: u32, test: bool) {
    let day_str: String = get_day_str(day);

    let input_dir = if test { "test inputs" } else { "inputs" };
    let input_file = format!("src/{}/day_{}.txt", input_dir, day_str);
    let _input = match fs::read_to_string(&input_file) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Failed to read input file: {}", input_file);
            std::process::exit(1);
        }
    };

    let mut start_time;
    let runtime_one;
    let runtime_two;
    match day_str.as_str() {
		"01" => {
            start_time = Instant::now();
            days::day_01::one::run(&_input);
            runtime_one = start_time.elapsed();

            start_time = Instant::now();
            days::day_01::two::run(&_input);
            runtime_two = start_time.elapsed();
        },
		"02" => {
            start_time = Instant::now();
            days::day_02::one::run(&_input);
            runtime_one = start_time.elapsed();

            start_time = Instant::now();
            days::day_02::two::run(&_input);
            runtime_two = start_time.elapsed();
        },
		"03" => {
            start_time = Instant::now();
            days::day_03::one::run(&_input);
            runtime_one = start_time.elapsed();

            start_time = Instant::now();
            days::day_03::two::run(&_input);
            runtime_two = start_time.elapsed();
        },
		_ => {
            if day > 25 {
                eprintln!("Day {} does not exist", day_str);
            } else {
                eprintln!("Day {} not yet implemented", day_str);
            }
            std::process::exit(1);
        }
    }

    println!("Part one: {:?}\nPart two: {:?}", runtime_one, runtime_two);
}

fn add_new_day(day: u32) {
    let day_str: String = get_day_str(day);

    // check for file existence
    let day_dir = format!("src/days/day_{}", day_str);
    if Path::new(&day_dir).exists() {
        eprintln!("Day {} already exists.", day_str);
        std::process::exit(1);
    }

    // crate day parent directory
    if let Err(e) = fs::create_dir(&day_dir) {
        eprintln!("Failed to create directory {}: {}", day_dir, e);
        std::process::exit(1);
    }

    println!("Created {}", day_dir);

    // write placeholder to one.rs and two.rs
    let one_path = format!("{}/one.rs", day_dir);
    fs::write(&one_path, 
        "#[allow(unused_imports)]
        use crate::utils::prelude::*;

        pub fn run(input: &String) {}"
        )
        .expect("Failed to write one.rs");
    
    let two_path = format!("{}/two.rs", day_dir);
    fs::write(&two_path, 
        "#[allow(unused_imports)]
        use crate::utils::prelude::*;

        pub fn run(input: &String) {}"
        )
        .expect("Failed to write two.rs");

    let mod_path = format!("{}/mod.rs", day_dir);
    fs::write(&mod_path, "pub mod one;\npub mod two;").expect("Failed to write mod.rs");

    println!("Created {}/one.rs, {}/two.rs, and {}/mod.rs", day_dir, day_dir, day_dir);

    // update mod.rs
    let mod_path = "src/days/mod.rs";
    let mod_entry = format!("pub mod day_{};\n", day_str);
    fs::OpenOptions::new()
        .append(true)
        .open(mod_path)
        .expect("Failed to open mod.rs")
        .write_all(mod_entry.as_bytes())
        .expect("Failed to write to mod.rs");

    println!("Updated {}", mod_path);

    // prompt for input files
    let inputs_dir = "src/inputs";
    let test_inputs_dir = "src/test inputs";

    let main_input_path = format!("{}/day_{}.txt", inputs_dir, day_str);
    let test_input_path = format!("{}/day_{}.txt", test_inputs_dir, day_str);

    println!("Please enter the content for test input (day_{}.txt):", day_str);
    let test_input = read_multiline_input();
    fs::write(&test_input_path, test_input).expect("Failed to write test input file");
    println!("\nCreated {}", test_input_path);

    println!("Please enter the content for the main input (day_{}.txt)", day_str);
    let main_input = read_multiline_input();
    fs::write(&main_input_path, main_input).expect("Failed to write main input file");
    println!("\nCreated {}", main_input_path);

    // Update main.rs to handle the new day
    let main_rs = "src/main.rs";
    let add_to_run_day = format!(
        "\"{}\" => {{
            start_time = Instant::now();
            days::day_{}::one::run(&_input);
            runtime_one = start_time.elapsed();

            start_time = Instant::now();
            days::day_{}::two::run(&_input);
            runtime_two = start_time.elapsed();
        }},\n\t\t",
        day_str, day_str, day_str
    );

    // Insert the match arm before the default case
    let mut contents = fs::read_to_string(main_rs).expect("Failed to read main.rs");
    let insert_position = contents
        .find("_ => {")
        .expect("Failed to find match arm in main.rs");
    contents.insert_str(insert_position, &add_to_run_day);
    fs::write(main_rs, contents).expect("Failed to update main.rs");

    println!("Updated main.rs to handle Day {}", day);
}

fn read_multiline_input() -> String {
    println!("Press Ctrl+Z (Windows) followed by Enter to finish:");
    let mut input = String::new();
    match io::stdin().read_to_string(&mut input) {
        Ok(_) => input,
        Err(_) => {
            eprintln!("Failed to read input.");
            std::process::exit(1);
        }
    }
}

fn get_day_str(day: u32) -> String {
    if day < 10 {
        return format!("0{}", day);
    } else {
        return day.to_string();
    }
}