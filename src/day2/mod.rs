pub fn find_invalid_ids_lexicographically(range: &str, verbose: bool) -> Vec<u64> {
    let (first, last) = range.trim().split_once("-").unwrap();
    let mut results: Vec<Vec<u64>> = vec![];
    for divisor in 2..=last.len() {
        results.push(get_repeats(range, verbose, first, last, divisor));
    }
    results.into_iter().flatten().collect()
}
pub fn find_invalid_ids_lexicographically_by_two(range: &str, verbose: bool) -> Vec<u64> {
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
    get_repeats(range, verbose, first, last, 2)
}
fn get_repeats(range: &str, verbose: bool, first: &str, last: &str, divisor: usize) -> Vec<u64> {
    let start_dup_range = lexicographical_lowest_bound(first, divisor, verbose);
    let end_dup_range = lexicographical_upper_bound(last, divisor, verbose);
    let end_digits = end_dup_range.parse::<u64>().unwrap();
    let start_digits = start_dup_range.parse::<u64>().unwrap();
    if end_digits < start_digits {
        if verbose {
            println!("{} has no invalid ids in divisor {}", range, divisor)
        }
        return vec![];
    }
    let result: Vec<u64> = (start_digits..=end_digits)
        .map(|i| i.to_string().repeat(divisor).parse::<u64>().unwrap())
        .collect();
    if verbose {
        if !result.is_empty() {
            print!("{} has invalid IDs in divisor {} ", range, divisor);

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
fn lexicographical_lowest_bound(num_str: &str, divisor: usize, verbose: bool) -> String {
    let len = num_str.len();
    let chunk = len / divisor;
    if len < divisor {
        // if there aren't enough chunks, return a safe default
        //   of a single 1
        // if verbose {
        //     print!(
        //         "smallest chunk possible for divisor {} is {}; ",
        //         divisor,
        //         1.to_string().repeat(divisor)
        //     )
        // }
        return 1.to_string();
    }
    if len % divisor != 0 {
        // if there are uneven chunks, return the lowest possible chunk as the first chunk
        let ret = String::from("1") + &"0".repeat(len - (chunk) - 1);
        // if verbose {
        //     print!(
        //         "smallest chunk bigger than {} digits for divisor {} for this len is {}; ",
        //         len,
        //         divisor,
        //         ret.to_string().repeat(divisor)
        //     )
        // }
        ret
    } else {
        let (highest_chunk, _min, max) = get_chunks(num_str, chunk);
        let ret = match highest_chunk == max {
            false => (highest_chunk.parse::<u64>().unwrap_or_default() + 1).to_string(),
            true => highest_chunk.to_string(),
        };
        // if verbose {
        //     print!(
        //         "smallest chunk bigger than {} digits for divisor {} for this len is {}; ",
        //         len,
        //         divisor,
        //         ret.to_string().repeat(divisor)
        //     )
        // }
        ret
    }
}
fn lexicographical_upper_bound(num_str: &str, divisor: usize, verbose: bool) -> String {
    if num_str.len() < divisor {
        0.to_string()
    } else if num_str.len() % divisor == 0 {
        let (highest_chunk, min, _max) = get_chunks(num_str, num_str.len() / divisor);
        get_lowest_upper_bound(divisor, verbose, highest_chunk, min)
    } else {
        "9".repeat(num_str.len() / divisor)
    }
}
fn get_chunks(num_str: &str, chunk_size: usize) -> (String, String, String) {
    let digit_groups: Vec<String> = num_str
        .chars()
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect();
    let highest_chunk = digit_groups[0].to_owned();
    let min = digit_groups.iter().min().unwrap().to_string();
    let max = digit_groups.iter().max().unwrap().to_string();
    (highest_chunk, min, max)
}
fn get_lowest_upper_bound(
    divisor: usize,
    verbose: bool,
    highest_chunk: String,
    min: String,
) -> String {
    let result = match highest_chunk == min {
        true => highest_chunk.to_string(),
        false => (highest_chunk.parse::<u64>().unwrap_or_default() - 1).to_string(),
    };
    if verbose {
        print!("upper range found as {}; ", result.repeat(divisor),);
    }
    result
}
