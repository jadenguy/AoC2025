pub mod day1;
use day1::{Dial, apply_and_count_zeroes_clicks_and_final, parse_instructions, read_lines};
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().count() == 1 {
        run_all();
        for n in 3..=12 {
            println!(
                "fn run_day{}() {{    println!(\"Running day {} logic\");}",
                n
            )
        }
    } else {
        for day in args.iter().skip(1) {
            let var_name = day.as_str();
            match var_name {
                "day1" => run_day1(),
                "day2" => run_day2(),
                "day3" => run_day3(),
                _ => println!("Unknown day: {}", day),
            }
        }
    }
}

fn run_all() {
    run_day1();
    run_day2();
    run_day3();
}
fn run_day1() {
    let lines = read_lines("./data/day1/part1.txt").expect("Failed to read lines from file");
    let instructions = parse_instructions(&lines);
    let dial = Dial { position: 50 };
    let (total_zero_dials, total_clicks, _final_dial) =
        apply_and_count_zeroes_clicks_and_final(instructions, dial);
    println!("Part 1: Your password is {}", total_zero_dials);
    println!("Part 2: Your password is {}", total_clicks);
}
