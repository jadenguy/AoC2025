pub mod common;
use aoc2025::{
    day4::find_isolated_rolls_with_output, day6::convert_worksheet_to_problems_cephalopod,
};
use common::read_lines;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().count() == 1 {
        run_all();
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
                _ => {
                    println!("Unknown day: {}", day);
                    run_all();
                }
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
            find_invalid_ids_lexicographically(l, false)
                .iter()
                .sum::<u64>()
        })
        .sum::<u64>();
    println!("Day 2 Part 2: Invalids IDs sum to {}", invalid_ids);
}

fn run_day3() {
    use aoc2025::day3::largest_joltage;
    let lines = read_lines("./data/day3/part1.txt").expect("Failed to read lines from file");
    let invalid_ids = lines
        .iter()
        .map(|l| largest_joltage(l.to_owned(), 2).parse::<u64>().unwrap())
        .sum::<u64>();
    println!("Day 3 Part 1: Jolt total {}", invalid_ids);
    let invalid_ids = lines
        .iter()
        .map(|l| largest_joltage(l.to_owned(), 12).parse::<u64>().unwrap())
        .sum::<u64>();
    println!("Day 3 Part 2: Jolt total {}", invalid_ids);
}
fn run_day4() {
    use aoc2025::day4::{
        convert_lines_to_board,
        find_isolated_rolls,
        // print_board,
    };
    {
        let rows = read_lines("./data/day4/part1.txt").expect("Failed to read lines from file");
        let board = convert_lines_to_board(rows);
        let isolated_rows = find_isolated_rolls(board);
        println!("Day 4 Part 1: Isolated rolls count {}", isolated_rows);
    }
    {
        let rows = read_lines("./data/day4/part1.txt").expect("Failed to read lines from file");
        let mut board = convert_lines_to_board(rows);
        let mut all_found = 0usize;
        let mut found = 1;
        // let (max_row, max_col) = <(i32, i32)>::from(board.keys().max().unwrap());
        while found > 0 {
            let result = find_isolated_rolls_with_output(board);
            // todo!("return the updated items and only look NEAR those positions");
            found = result.0;
            board = result.1;

            all_found += found;
            // print_board(&board, max_row, max_col);
        }
        println!("Day 4 Part 2: Isolated rolls count {}", all_found);
    }
}

fn run_day5() {
    use aoc2025::day5::{count_fresh_ingredients, parse_db, total_fresh_ids};
    let db_file = read_lines("./data/day5/part1.txt").expect("Failed to read lines from file");
    let db = parse_db(db_file);
    let ingredient_count = count_fresh_ingredients(&db);
    println!("Day 5 Part 1: Fresh ingredient count {}", ingredient_count);
    let id_count = total_fresh_ids(&db);
    println!("Day 5 Part 2: All fresh IDs possible {}", id_count);
}
fn run_day6() {
    use aoc2025::day6::{convert_worksheet_to_problems, generate_ast_from_problem};

    let worksheet = read_lines("./data/day6/part1.txt").expect("Failed to read lines from file");

    let problems = convert_worksheet_to_problems(&worksheet);
    let actual_sum_of_eval: i64 = problems
        .iter()
        .map(|problem| {
            generate_ast_from_problem(problem.to_owned())
                .unwrap()
                .evaluate()
        })
        .sum();
    println!(
        "Day 6 Part 1: Worksheet sum of problems {}",
        actual_sum_of_eval
    );

    let problems = convert_worksheet_to_problems_cephalopod(&worksheet);
    let actual_sum_of_eval: i64 = problems
        .iter()
        .map(|problem| {
            // println!("{}", problem.join("\n"));
            // println!();
            let ast = generate_ast_from_problem(problem.to_owned());
            if let Some(e) = ast {
                // print!("{}", e.to_string());
                let ret = e.evaluate();
                // println!("={}", ret);
                ret
            } else {
                0
            }
        })
        .sum();
    println!(
        "Day 6 Part 2: Worksheet sum of problems cephalopod style {}",
        actual_sum_of_eval
    );
}
fn run_day7() {
    use aoc2025::day7::{parse_manifold_strings, process_manifold};
    let manifold_initial_state =
        read_lines("./data/day7/part1.txt").expect("Failed to read lines from file");
    let manifold =
        parse_manifold_strings(manifold_initial_state.iter().map(|s| s.as_str()).collect());
    let proc = process_manifold(&manifold);
    println!("Day 7 Part 1: Manifold beam splits {}", proc.0);
    println!("Day 7 Part 2: Manifold beam paths {}", proc.1);
}
fn run_day8() {
    use aoc2025::day8::{connect_junction_boxes, parse_junction_boxes};
    let junction_box_strings =
        read_lines("./data/day8/part1.txt").expect("Failed to read lines from file");
    let jbox: Vec<&str> = junction_box_strings.iter().map(|l| l.as_str()).collect();
    let junction_boxes = parse_junction_boxes(jbox);

    let junction_networks = connect_junction_boxes(junction_boxes, 1000);
    let mut sizes: Vec<usize> = junction_networks.iter().map(|x| x.len()).collect();
    sizes.sort_by_key(|&k| -1 * (k as i64));
    let product_of_three_longest: usize = sizes.iter().take(3).product();
    println!(
        "Day 8 Part 1: largest networks product {} *{} * {} ={}",
        sizes[0], sizes[1], sizes[2], product_of_three_longest
    );
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
