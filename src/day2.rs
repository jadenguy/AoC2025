use std::collections::HashSet;

pub fn find_invalid_ids_lexicographically(range: &str, verbose: bool) -> HashSet<u64> {
    let (first, last) = range.trim().split_once("-").unwrap();
    let mut results: HashSet<u64> = HashSet::new();
    if verbose {
        println!(
            "checking {}, {} digit to {} digit",
            range,
            first.len(),
            last.len()
        )
    }
    let mut primes: HashSet<usize> = HashSet::new();
    for divisor in 2..=last.len() {
        if primes.iter().any(|n| divisor % n == 0) {
            if verbose {
                println!("    skipping {} divisor as nonprime", divisor)
            }
        } else {
            primes.insert(divisor);
            let (result, start_dup_range, end_dup_range) = get_repeats(first, last, divisor);
            if verbose {
                if result.is_empty() {
                    println!("  divisor {} yielded no results", divisor)
                } else {
                    print!(
                        "  divisor {} found invalid IDs from {} to {}",
                        divisor,
                        start_dup_range.repeat(divisor),
                        end_dup_range.repeat(divisor)
                    );
                    println!();
                }
            }
            for v in result {
                results.insert(v);
            }
        }
    }
    if verbose {
        println!("")
    }
    results
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
    get_repeats(first, last, 2).0
}
pub fn get_repeats(first: &str, last: &str, divisor: usize) -> (Vec<u64>, String, String) {
    let start_dup_range = lexicographical_lowest_bound(first, divisor);
    let end_dup_range = lexicographical_upper_bound(last, divisor);
    let start_digits = start_dup_range.parse::<u64>().unwrap();
    let end_digits = end_dup_range.parse::<u64>().unwrap();
    if start_digits > end_digits {
        return (vec![], start_dup_range, end_dup_range);
    }
    let result: Vec<u64> = (start_digits..=end_digits)
        .map(|i| i.to_string().repeat(divisor).parse::<u64>().unwrap())
        .collect();

    (result, start_dup_range, end_dup_range)
}
pub fn lexicographical_lowest_bound(num_str: &str, divisor: usize) -> String {
    let len = num_str.len();
    let chunk = len / divisor;
    if len < divisor {
        // if there aren't enough chunks, return a safe default
        //   of a single 1

        return 1.to_string();
    }
    if len % divisor != 0 {
        // if there are uneven chunks, return the lowest possible chunk as the first chunk
        let ret = String::from("1") + &"0".repeat(chunk);

        ret
    } else {
        // let (_chunks, highest_chunk, _second_chunk, _min, max, _asc) =
        //     get_chunks_and_info(num_str, chunk);
        let chunks = get_chunks(num_str, chunk);
        let highest_chunk = chunks[0].to_owned();
        let first_chunk_lowest = chunks_equal_or_desc(&chunks, &highest_chunk);
        let ret = match first_chunk_lowest {
            false => (highest_chunk.parse::<u64>().unwrap_or_default() + 1).to_string(),
            true => highest_chunk.to_string(),
        };

        ret
    }
}

fn chunks_equal_or_desc(chunks: &[String], highest_chunk: &str) -> bool {
    let mut current: String = highest_chunk.to_string();
    for val in chunks.iter().skip(1) {
        if val.to_string() > current {
            return false;
        } else if val.to_string() < current {
            return true;
        } else {
            current = val.to_string();
        }
    }
    true
}
pub fn lexicographical_upper_bound(num_str: &str, divisor: usize) -> String {
    let len = num_str.len();
    if len < divisor {
        0.to_string()
    } else if len % divisor == 0 {
        let chunks = get_chunks(num_str, len / divisor);
        let highest_chunk = chunks[0].to_owned();
        match chunks_equal_or_asc(&chunks, &highest_chunk) {
            true => highest_chunk.to_string(),
            false => (highest_chunk.parse::<u64>().unwrap_or_default() - 1).to_string(),
        }
    } else {
        "9".repeat(len / divisor)
    }
}
fn chunks_equal_or_asc(chunks: &[String], highest_chunk: &String) -> bool {
    let mut current: String = highest_chunk.to_string();
    for val in chunks.iter().skip(1) {
        if val.to_string() > current {
            return true;
        } else if val.to_string() < current {
            return false;
        } else {
            current = val.to_string();
        }
    }
    true
}

fn get_chunks(num_str: &str, chunk_size: usize) -> Vec<String> {
    num_str
        .chars()
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect()
}
