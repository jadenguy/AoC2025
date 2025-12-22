pub mod common;
use common::read_lines;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().count() == 1 {
        run_all();
        // run_day2();
    } else {
        for day in args.iter().skip(1) {
            let var_name = day.as_str();
            match var_name {
                "day1" => run_day1(),
                "day2" => run_day2(),
                "day3" => run_day3(),
                "day4" => run_day4(),
                "day5" => run_day5(),
                "day6" => run_day6(),
                "day7" => run_day7(),
                "day8" => run_day8(),
                "day9" => run_day9(),
                "day10" => run_day10(),
                "day11" => run_day11(),
                "day12" => run_day12(),
                _ => println!("Unknown day: {}", day),
            }
        }
    }
}

fn run_all() {
    run_day1();
    run_day2();
    run_day3();
    run_day4();
    run_day5();
    run_day6();
    run_day7();
    run_day8();
    run_day9();
    run_day10();
    run_day11();
    run_day12();
}
fn run_day1() {
    use aoc2025::day1::{apply_and_count_zeroes_clicks_and_final, parse_instructions};
    let lines = read_lines("./data/day1/part1.txt").expect("Failed to read lines from file");
    let instructions = parse_instructions(&lines);

    let (total_zero_dials, total_clicks, _final_dial) =
        apply_and_count_zeroes_clicks_and_final(instructions, 50, false);
    println!("Day 1 Part 1: Your password is {}", total_zero_dials);
    println!("Day 2 Part 2: Your password is {}", total_clicks);
}

fn run_day2() {
    use aoc2025::day2::{
        find_invalid_ids_lexicographically, find_invalid_ids_lexicographically_by_two,
    };
    let lines = read_lines("./data/day2/part1.txt").expect("Failed to read lines from file");
    let invalid_ids = lines
        .first()
        .unwrap()
        .split(',')
        .map(|l| {
            find_invalid_ids_lexicographically_by_two(l, false)
                .iter()
                .sum::<u64>()
        })
        .sum::<u64>();
    println!("Day 2 Part 1: Invalids IDs sum to {}", invalid_ids);
    let invalid_ids = lines
        .first()
        .unwrap()
        .split(',')
        .map(|l| {
            find_invalid_ids_lexicographically(l, true)
                .iter()
                .sum::<u64>()
        })
        .sum::<u64>();
    println!("Day 2 Part 2: Invalids IDs sum to {}", invalid_ids);
}

fn run_day3() {
    println!("Running day 3 logic");
}
fn run_day4() {
    println!("Running day 4 logic");
}
fn run_day5() {
    println!("Running day 5 logic");
}
fn run_day6() {
    println!("Running day 6 logic");
}
fn run_day7() {
    println!("Running day 7 logic");
}
fn run_day8() {
    println!("Running day 8 logic");
}
fn run_day9() {
    println!("Running day 9 logic");
}
fn run_day10() {
    println!("Running day 10 logic");
}
fn run_day11() {
    println!("Running day 11 logic");
}
fn run_day12() {
    println!("Running day 12 logic");
}
