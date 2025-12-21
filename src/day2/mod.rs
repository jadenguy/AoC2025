pub fn find_invalid_ids_lexicographically(range: &str, verbose: bool) -> Vec<u64> {
    if verbose {
        print!("{} check; ", range);
    }
    let (first, last) = range.trim().split_once("-").unwrap();
    if (first.len() == last.len()) && (first.len() % 2 == 1) {
        if verbose {
            println!(
                "{} is in an odd range of {} and cannot have results.",
                range,
                last.len()
            )
        }
        return vec![];
    }
    let start_dup_range = lexicographical_lower_bound_by_two(first, verbose);
    let end_dup_range = lexicographical_upper_bound_by_two(last, verbose);
    if end_dup_range < start_dup_range {
        if verbose {
            println!("{} has no invalid ids", range)
        }
        return vec![];
    }
    let result: Vec<u64> = (start_dup_range..=end_dup_range)
        .map(|i| format!("{}{}", i, i).parse::<u64>().unwrap())
        .collect();
    if verbose {
        if !result.is_empty() {
            print!("{} has invalid IDs ", range);

            print!(
                "{}",
                result
                    .iter()
                    .map(|y| y.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
        }
        println!();
    };
    result
}

fn lexicographical_lower_bound_by_two(num_str: &str, verbose: bool) -> u64 {
    lexicographical_lower_bound_by_divisor(num_str, 2, verbose).digits
}
fn lexicographical_lower_bound_by_divisor(
    num_str: &str,
    divisor: usize,
    verbose: bool,
) -> DigitsAndCount {
    let result;
    let len = num_str.len();
    let chunk_size = len - (len / divisor);
    if len < chunk_size {
        result = 0;
    } else if num_str.len() % divisor != 1 {
        let digit_groups: Vec<String> = num_str
            .chars()
            .collect::<Vec<_>>()
            .chunks(chunk_size)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect();
        let var_name = &digit_groups[0];
        let max = digit_groups.iter().max().unwrap();

        if var_name == max {
            result = var_name.parse::<u64>().unwrap_or_default();
        } else {
            result = var_name.parse::<u64>().unwrap_or_default() + 1;
        }
        if verbose {
            print!("lower range found as {}{}; ", result, result,);
        }
    } else {
        result = (String::from("1") + &"0".repeat(chunk_size - 1))
            .parse::<u64>()
            .unwrap();
        if verbose {
            print!(
                "range has too few starting digits at {}, running {}{}; ",
                len, result, result,
            );
        }
    }
    DigitsAndCount {
        digits: result,
        count: divisor,
    }
}
struct DigitsAndCount {
    digits: u64,
    count: usize,
}
fn lexicographical_upper_bound_by_two(num_str: &str, verbose: bool) -> u64 {
    lexicographical_upper_bound_by_divisor(num_str, 2, verbose).digits
}
fn lexicographical_upper_bound_by_divisor(
    num_str: &str,
    divisor: usize,
    verbose: bool,
) -> DigitsAndCount {
    if num_str.len() % divisor == 1 {
        let result = ("9".repeat(divisor - 1)).parse::<u64>().unwrap_or_default();
        if verbose {
            print!(
                "range has too many digits at {}, running {}{}; ",
                num_str.len(),
                result,
                result,
            );
        }
        return DigitsAndCount {
            digits: result,
            count: divisor,
        };
    }
    let (high, low) = num_str.split_at(num_str.len() / divisor);
    let lower = low.parse::<u64>().unwrap();
    let higher = high.parse::<u64>().unwrap();
    let result = match lower >= higher {
        true => higher,
        false => higher - 1,
    };
    if verbose {
        print!("upper range found as {}{}; ", result, result,);
    }

    DigitsAndCount {
        digits: result,
        count: divisor,
    }
}
