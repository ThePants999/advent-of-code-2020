mod utils;
mod day1;

use std::env;

type DayFunction = fn(Vec<String>) -> (u64, u64);
static DAY_FUNCTIONS: [DayFunction; 1] = [day1::day1];

fn main() {
    let args: Vec<String> = env::args().collect();
    let min_day: usize;
    let max_day: usize;
    if args.len() < 2 {
        min_day = 1;
        max_day = DAY_FUNCTIONS.len();
    } else {
        min_day = args[1].parse::<usize>().expect("Please provide the day number as an integer.");
        max_day = min_day;
    }

    for day in min_day..=max_day {
        println!("Day {}", day);
        let input_lines = utils::load_inputs(day);
        let start_time = std::time::Instant::now();
        let (part1, part2) = DAY_FUNCTIONS[day - 1](input_lines);    
        let elapsed = start_time.elapsed().as_micros();
        println!("Part 1: {}\nPart 2: {}", part1, part2);
        println!("{}.{}ms", elapsed / 1000, elapsed % 1000);
        println!("----------");
    }
}
