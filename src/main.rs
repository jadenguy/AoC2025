pub mod common;

use common::read_lines;
fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();
    if args.iter().count() == 1 {
        run_all();
    } else {
        for day in args.iter().skip(1) {
            let var_name = day.as_str();
            match var_name {
                "day01" => run_day01(),
                "day02" => run_day02(),
                "day03" => run_day03(),
                "day04" => run_day04(),
                "day05" => run_day05(),
                "day06" => run_day06(),
                "day07" => run_day07(),
                "day08" => run_day08(),
                "day09" => run_day09(),
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
    run_day01();
    run_day02();
    run_day03();
    run_day04();
    run_day05();
    run_day06();
    run_day07();
    run_day08();
    run_day09();
    run_day10();
    run_day11();
    run_day12();
}
fn run_day01() {
    use aoc2025::day01::*;
    let lines = read_lines("./data/day01/part1.txt").expect("Failed to read lines from file");
    let instructions = parse_instructions(&lines);

    let (total_zero_dials, total_clicks, _final_dial) =
        apply_and_count_zeroes_clicks_and_final(instructions, 50, false);
    println!("Day 1 Part 1: Your password is {}", total_zero_dials);
    println!("Day 2 Part 2: Your password is {}", total_clicks);
}

fn run_day02() {
    use aoc2025::day02::*;
    let lines = read_lines("./data/day02/part1.txt").expect("Failed to read lines from file");
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

fn run_day03() {
    use aoc2025::day03::*;
    let lines = read_lines("./data/day03/part1.txt").expect("Failed to read lines from file");
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
fn run_day04() {
    use aoc2025::day04::*;
    {
        let rows = read_lines("./data/day04/part1.txt").expect("Failed to read lines from file");
        let board = convert_lines_to_board(rows);
        let isolated_rows = find_isolated_rolls(board);
        println!("Day 4 Part 1: Isolated rolls count {}", isolated_rows);
    }
    {
        let rows = read_lines("./data/day04/part1.txt").expect("Failed to read lines from file");
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

fn run_day05() {
    use aoc2025::day05::*;
    let db_file = read_lines("./data/day05/part1.txt").expect("Failed to read lines from file");
    let db = parse_db(db_file);
    let ingredient_count = count_fresh_ingredients(&db);
    println!("Day 5 Part 1: Fresh ingredient count {}", ingredient_count);
    let id_count = total_fresh_ids(&db);
    println!("Day 5 Part 2: All fresh IDs possible {}", id_count);
}
fn run_day06() {
    use aoc2025::day06::*;

    let worksheet = read_lines("./data/day06/part1.txt").expect("Failed to read lines from file");

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
fn run_day07() {
    use aoc2025::day07::*;
    let manifold_initial_state =
        read_lines("./data/day07/part1.txt").expect("Failed to read lines from file");
    let manifold =
        parse_manifold_strings(manifold_initial_state.iter().map(|s| s.as_str()).collect());
    let proc = process_manifold(&manifold);
    println!("Day 7 Part 1: Manifold beam splits {}", proc.0);
    println!("Day 7 Part 2: Manifold beam paths {}", proc.1);
}
fn run_day08() {
    use aoc2025::day08::*;
    let junction_box_strings =
        read_lines("./data/day08/part1.txt").expect("Failed to read lines from file");
    let jbox: Vec<&str> = junction_box_strings.iter().map(|l| l.as_str()).collect();
    let junction_boxes = parse_junction_boxes(jbox);

    let junction_networks = connect_junction_boxes_n_times(&junction_boxes, 1000);
    let mut sizes: Vec<usize> = junction_networks.iter().map(|x| x.len()).collect();
    sizes.sort_by_key(|&k| -1 * (k as i64));
    let product_of_three_longest: usize = sizes.iter().take(3).product();
    println!(
        "Day 8 Part 1: largest networks product  {}",
        product_of_three_longest
    );
    let last_connection = connect_junction_boxes_to_exhaustion(&junction_boxes);
    println!(
        "Day 8 Part 2: last connection x coord product  {}",
        last_connection.0.x * last_connection.1.x
    );
}
fn run_day09() {
    use aoc2025::day09::*;
    let tile_string = read_lines("./data/day09/part1.txt").expect("Failed to read lines from file");
    let tiles = parse_tiles(tile_string.iter().map(|x| x.as_str()).collect());

    let (_a, _b, area) =
        furthest_tiles(&tiles).expect("returned none but should have returned red pair tiles");
    println!("Day 9 Part 1: Largest rectangle area {}", area);
    let (_a, _b, area) = furthest_red_green_tiles(&tiles)
        .expect("returned none but should have returned red pair tiles over red/green");
    println!("Day 9 Part 2: Largest red/green rectangle area {}", area);
}
fn run_day10() {
    use aoc2025::day10::*;
    let machine_inst_list =
        read_lines("./data/day10/part1.txt").expect("Failed to read lines from file");
    let machines: Vec<MachineState> = machine_inst_list
        .iter()
        // .skip(1)
        .map(|d| MachineState::from_instructions(&parse_machine_instructions(d)))
        // .take(1)
        .collect();
    // act
    let presses: usize = machines
        .iter()
        .map(|m| find_min_presses_for_indicators(m))
        .sum();
    println!("Day 10 Part 1: Min Presses {}", presses);
    // let presses: Joltage = machines
    //     .iter()
    //     .map(|m| find_minimal_presses_for_joltage(m))
    //     .sum();
    // println!("Day 10 Part 2: Min Presses {}", presses);
    println!(
        "Day 10 Part 2: Min Presses {} via goodlp, need to find another non-system dependant way",
        14677
    );
}
fn run_day11() {
    use aoc2025::day11::reactor::Reactor;
    use aoc2025::day11::*;
    let lines = read_lines("./data/day11/part1.txt").expect("Failed to read lines from file");
    let reactor = Reactor::from_string(lines);
    println!(
        "Day 11 Part 1: Paths from YOU to OUT {}",
        reactor.path_iter("you", "out").count()
    );

    println!(
        "Day 11 Part 2: Paths from SVR to OUT passing DAC and FFT {}",
        count_paths_containing_nodes("svr", "out", reactor, vec!["dac", "fft"])
    );
}
fn run_day12() {
    let lines = read_lines("./data/day12/part1.txt").expect("Failed to read lines from file");
    let requirement = aoc2025::day12::Requirement::from_strings(lines);
    println!(
        "Day 12 Part 1: These rows will always work {}",
        requirement.always_possible_areas().len()
    );
}
