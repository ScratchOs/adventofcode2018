use std::env;
extern crate regex;

enum Part {
    First,
    Second,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify the code to run (e.g. 2b)");
        return;
    }
    if args.len() > 2 {
        println!("Only 1 argument allowed");
        return;
    }
    let arg = &args[1];
    let day: u32;
    let part: Part;
    if arg.len() == 2 {
        let mut iter = arg.chars();
        let one = iter.next().unwrap().to_digit(10);
        match one {
            Some(i) => day = i,
            None => {
                println!("Day specifier not found");
                return;
            }
        }
        match iter.next() {
            Some('a') => part = Part::First,
            Some('b') => part = Part::Second,
            _ => {
                println!("Part specifier not found");
                return;
            }
        }
        run(day, part);
    } else if arg.len() == 3 {
        let mut iter = arg.chars();
        let num_str = iter.next().unwrap().to_string() + &iter.next().unwrap().to_string();
        match num_str.parse::<i32>() {
            Ok(i) => day = i as u32,
            Err(_) => {
                println!("Day specifier not found");
                return;
            }
        }
        match iter.next() {
            Some('a') => part = Part::First,
            Some('b') => part = Part::Second,
            _ => {
                println!("Part specifier not found");
                return;
            }
        }
        run(day, part);
    } else {
        println!("Invalid code specifier");
    }
}
struct ID(u32, Part);

mod day1a;
mod day1b;
mod day2a;
mod day2b;
mod day3a;
mod day3b;
mod day4a;
mod day4b;
mod day5a;
mod day5b;
mod day6a;
mod day6b;
mod day7a;
mod day7b;
mod day8a;
mod day8b;

fn run(day: u32, part: Part) {
    let id = ID(day, part);
    match id {
        ID(1, Part::First) => day1a::day1a(),
        ID(1, Part::Second) => day1b::day1b(),
        ID(2, Part::First) => day2a::day2a(),
        ID(2, Part::Second) => day2b::day2b(),
        ID(3, Part::First) => day3a::day3a(),
        ID(3, Part::Second) => day3b::day3b(),
        ID(4, Part::First) => day4a::day4a(),
        ID(4, Part::Second) => day4b::day4b(),
        ID(5, Part::First) => day5a::day5a(),
        ID(5, Part::Second) => day5b::day5b(),
        ID(6, Part::First) => day6a::day6a(),
        ID(6, Part::Second) => day6b::day6b(),
        ID(7, Part::First) => day7a::day7a(),
        ID(7, Part::Second) => day7b::day7b(),
        ID(8, Part::First) => day8a::day8a(),
        ID(8, Part::Second) => day8b::day8b(),
        ID(_, _) => println!("Could not find day/part"),
    }
}
