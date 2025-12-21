pub fn find_invalid_ids_lexicographically(range: &str, verbose: bool) -> Vec<u64> {
    if verbose {
        print!("{} check; ", range);
    }
    let (first, last) = range.trim().split_once("-").unwrap();
    if (first.len() == last.len()) && (first.len() % 2 == 1) {
        if verbose {
            println!("{} is in an odd range and cannot have results.", range)
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
    let (high, low) = num_str.split_at(num_str.len() / divisor);
    if num_str.len() % divisor == 1 {
        let n = high.len();
        let var_name = "0";
        let repeat = var_name.repeat(n);
        let result = (String::from("1") + &repeat).parse::<u64>().unwrap();
        if verbose {
            print!(
                "range has too few starting digits at {}, running {}{}; ",
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
    let lower = low.parse::<u64>().unwrap();
    let higher = high.parse::<u64>().unwrap();
    let result = match lower <= higher {
        true => higher,
        false => higher + 1,
    };
    if verbose {
        print!("lower range found as {}{}; ", result, result,);
    }

    return DigitsAndCount {
        digits: result,
        count: divisor,
    };
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
    let (high, low) = num_str.split_at(num_str.len() / divisor);
    if num_str.len() % divisor == 1 {
        let result = ("9".repeat(low.len() - 1))
            .parse::<u64>()
            .unwrap_or_default();
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
