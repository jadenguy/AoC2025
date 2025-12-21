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
    let divisor = 2;
    let start_dup_range = lexicographical_lower_bound(first, divisor, verbose);
    let end_dup_range = lexicographical_upper_bound(last, divisor, verbose);
    let end_digits = end_dup_range.digits.parse::<u64>().unwrap();
    let start_digits = start_dup_range.digits.parse::<u64>().unwrap();
    if end_digits < start_digits {
        if verbose {
            println!("{} has no invalid ids", range)
        }
        return vec![];
    }

    let result: Vec<u64> = (start_digits..=end_digits)
        .map(|i| {
            i.to_string()
                .repeat(start_dup_range.count)
                .parse::<u64>()
                .unwrap()
        })
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

fn lexicographical_lower_bound(num_str: &str, divisor: usize, verbose: bool) -> DigitsAndCount {
    let result: String;
    let len = num_str.len();
    let chunk_size = len - (len / divisor);
    if len < chunk_size {
        result = u64::MAX.to_string();
    } else if num_str.len() % divisor == 0 {
        let digit_groups: Vec<String> = num_str
            .chars()
            .collect::<Vec<_>>()
            .chunks(chunk_size)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect();
        let highest_chunk = &digit_groups[0];
        let max = digit_groups.iter().max().unwrap();
        result = match highest_chunk == max {
            false => (highest_chunk.parse::<u64>().unwrap_or_default() + 1).to_string(),
            true => highest_chunk.to_string(),
        };
        if verbose {
            print!("lower range found as {}{}; ", result, result,);
        }
    } else {
        result = String::from("1") + &"0".repeat(chunk_size - 1);
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
    digits: String,
    count: usize,
}
fn lexicographical_upper_bound(num_str: &str, divisor: usize, verbose: bool) -> DigitsAndCount {
    let len = num_str.len();
    let chunk_size = len - (len / divisor);
    let result: String;
    if len < chunk_size {
        result = 0.to_string();
    } else if len % chunk_size == 0 {
        let digit_groups: Vec<String> = num_str
            .chars()
            .collect::<Vec<_>>()
            .chunks(chunk_size)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect();
        let highest_chunk = &digit_groups[0];
        let min = digit_groups.iter().min().unwrap();
        result = match highest_chunk == min {
            true => highest_chunk.to_string(),
            false => (highest_chunk.parse::<u64>().unwrap_or_default() - 1).to_string(),
        };
        if verbose {
            print!("upper range found as {}{}; ", result, result,);
        }
    } else {
        result = "9".repeat(chunk_size - 1);
        if verbose {
            print!(
                "range has too many digits at {}, running {}{}; ",
                len, result, result,
            );
        }
    }
    DigitsAndCount {
        digits: result,
        count: divisor,
    }
}
