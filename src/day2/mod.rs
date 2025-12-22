use std::collections::HashSet;

pub fn find_invalid_ids_lexicographically(range: &str, verbose: bool) -> Vec<u64> {
    let (first, last) = range.trim().split_once("-").unwrap();
    let mut results: HashSet<u64> = HashSet::new();
    if verbose {
        println!("checking {}; ", range)
    }

    for divisor in 2..=last.len() {
        if verbose {
            // print!("divisor {}; ", divisor)
        }
        let value = get_repeats(range, false, first, last, divisor);
        if verbose {
            // if value.len() == 0 {
            //     print!(" no values")
            // }
            print!(
                "{} ",
                value
                    .iter()
                    .map(|y| y.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
        }
        for v in value {
            results.insert(v);
        }
    }
    if verbose {
        println!("")
    }
    results.iter().map(|x| x.to_owned()).collect()
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
    let start_dup_range = lexicographical_lowest_bound(first, divisor);
    let end_dup_range = lexicographical_upper_bound(last, divisor);
    let end_digits = end_dup_range.parse::<u64>().unwrap();
    let start_digits = start_dup_range.parse::<u64>().unwrap();
    if end_digits < start_digits {
        if verbose {
            println!(
                "{} has no invalid ids",
                range,
                // start_digits.to_string().repeat(divisor),
                // end_digits.to_string().repeat(divisor)
            )
        }
        return vec![];
    }
    let result: Vec<u64> = (start_digits..=end_digits)
        .map(|i| i.to_string().repeat(divisor).parse::<u64>().unwrap())
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
fn lexicographical_lowest_bound(num_str: &str, divisor: usize) -> String {
    let len = num_str.len();
    let chunk = len / divisor;
    if len < divisor {
        // if there aren't enough chunks, return a safe default
        //   of a single 1

        return 1.to_string();
    }
    if len % divisor != 0 {
        // if there are uneven chunks, return the lowest possible chunk as the first chunk
        let ret = String::from("1") + &"0".repeat(len - (chunk) - 1);

        ret
    } else {
        let (highest_chunk, _min, max) = get_chunks(num_str, chunk);
        let ret = match highest_chunk == max {
            false => (highest_chunk.parse::<u64>().unwrap_or_default() + 1).to_string(),
            true => highest_chunk.to_string(),
        };

        ret
    }
}
fn lexicographical_upper_bound(num_str: &str, divisor: usize) -> String {
    if num_str.len() < divisor {
        0.to_string()
    } else if num_str.len() % divisor == 0 {
        let (highest_chunk, min, _max) = get_chunks(num_str, num_str.len() / divisor);
        match highest_chunk == min {
            true => highest_chunk.to_string(),
            false => (highest_chunk.parse::<u64>().unwrap_or_default() - 1).to_string(),
        }
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
