pub mod day1;
use day1::{Dial, apply_and_count_zeroes_clicks_and_final, parse_instructions, read_lines};
fn main() {
    let lines = read_lines("./data/day1/part1.txt").expect("Failed to read lines from file");
    let instructions = parse_instructions(&lines);
    let dial = Dial { position: 50 };
    let (total_zero_dials, total_clicks, _final_dial) =
        apply_and_count_zeroes_clicks_and_final(instructions, dial);
    println!("Part 1: Your password is {}", total_zero_dials);
    println!("Part 2: Your password is {}", total_clicks);
}
