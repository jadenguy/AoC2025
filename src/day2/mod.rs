pub fn parse_id_ranges(id_ranges_string: &str) -> Vec<(u64, u64)> {
    id_ranges_string.split(",").map(parse_id_range).collect()
}
pub fn parse_id_range(s: &str) -> (u64, u64) {
    let x: Vec<String> = s.trim().split("-").map(|s| s.to_string()).collect();
    let start_string = x.first();
    let start_range: u64 = match start_string {
        Some(s) => s.parse::<u64>().unwrap(),
        None => 0,
    };
    let end_string = x.last();
    let end_range: u64 = match end_string {
        Some(s) => s.parse::<u64>().unwrap(),
        None => 0,
    };
    if start_range > end_range {
        return (0, 0);
    }
    (start_range, end_range)
}
pub fn find_invalid_ids(range: (u64, u64)) -> Vec<u64> {
    let lower_bound = range.0;
    let upper_bound = range.1;
    let mut lower_order_of_magnitude = lower_bound.ilog10();
    let mut upper_order_of_magnitude = upper_bound.ilog10();
    let results: Vec<u64> = vec![];
    static BASE_10: u64 = 10;
    if lower_order_of_magnitude % 2 == 0 {
        lower_order_of_magnitude += 1;
    }
    if upper_order_of_magnitude % 2 == 0 {}
    upper_order_of_magnitude -= 1;
    let low_pow = BASE_10.pow(lower_order_of_magnitude / 2 + 1);
    let mut lowest_invalid_id_half = lower_bound / low_pow;
    let upper_half_domain = BASE_10.pow(upper_order_of_magnitude / 2 + 1);
    let mut highest_invalid_id_half = upper_bound / upper_half_domain;
    let mut lowest_invalid_id = lowest_invalid_id_half * low_pow + lowest_invalid_id_half;
    if lowest_invalid_id < lower_bound {
        lowest_invalid_id_half += 1;
    }
    lowest_invalid_id = lowest_invalid_id_half * low_pow + lowest_invalid_id_half;

    let mut highest_invalid_id =
        highest_invalid_id_half * upper_half_domain + highest_invalid_id_half;
    if highest_invalid_id > upper_bound {
        highest_invalid_id_half -= 1;
    }
    highest_invalid_id = highest_invalid_id_half * upper_half_domain + highest_invalid_id_half;
    println!("{} domain to {}", low_pow, upper_half_domain);
    println!("{} value to {}", lowest_invalid_id, highest_invalid_id);
    if lowest_invalid_id > highest_invalid_id {
        println!("invalid");
        return results;
    }
    // let domains: u32 = (lower_bound..=upper_bound)
    //     .filter(|b| b % 2 == 1)
    //     .map(|b| b as i32);
    // for domain in domains {
    //     for value in (BASE_10.pow(domain))..(BASE_10.pow(domain + 1)) {
    //         results.push(value);
    //     }
    // }
    return results;
}
pub fn find_invalid_ids_lexicographically(range: &str, verbose: bool) -> Vec<u64> {
    let (first, last) = range.trim().split_once("-").unwrap();
    let x = lexicographical_lower_bound(first, verbose);
    let y = lexicographical_upper_bound(last, verbose);

    let result = (x..=y)
        .map(|i| format!("{}{}", i, i).parse::<u64>().unwrap())
        .collect();
    if verbose {
        print!("{} has invalid IDs ", range);
        for v in &result {
            print!(",{}", v)
        }
        println!();
    }

    result
}

fn lexicographical_lower_bound(num_str: &str, verbose: bool) -> u64 {
    let (high, low) = num_str.split_at(num_str.len() / 2);
    if high.len() < low.len() {
        if verbose {
            println!("default low digits {}", low.len() / 2);
        }
        return (String::from("1") + &"0".repeat(low.len() / 2))
            .parse::<u64>()
            .unwrap();
    }
    let higher = high.parse::<u64>().unwrap();
    let lower = low.parse::<u64>().unwrap();
    if higher>=lower{return higher;}
higher+1
}
fn lexicographical_upper_bound(num_str: &str, verbose: bool) -> u64 {
    let (high, low) = num_str.split_at(num_str.len() / 2);
    if high.len() < low.len() {
        if verbose {
            println!("default high digits {}", low.len() / 2);
        }
        return ("9".repeat(low.len() / 2))
            .parse::<u64>()
            .unwrap_or_default();
    }
    let higher = high.parse::<u64>().unwrap();
    let lower = low.parse::<u64>().unwrap();
if higher<=lower{
return    higher
;}
higher-1
}
