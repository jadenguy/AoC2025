pub fn largest_joltage(batteries: String, wanted_batteries: usize) -> String {
    let mut ret: Vec<char> = Vec::new();
    let battery_count = batteries.len();
    let mut skip = 0usize;
    // println!("{}; ", batteries);
    for i in 1..=wanted_batteries {
        // print!("  ");
        let cutoff = wanted_batteries - i;
        let range: Vec<char> = batteries
            .chars()
            .skip(skip)
            .take(battery_count - cutoff - skip)
            .collect();
        // println!(
        //     "{}{}{}: ",
        //     "*".repeat(skip),
        //     range.iter().collect::<String>(),
        //     "*".repeat(cutoff)
        // );

        let max = range.iter().max().unwrap();
        skip += range.iter().position(|x| x == max).unwrap() + 1;
        // print!("{} at {}", max, skip - 1);
        ret.push(max.to_owned());
    }
    ret.iter().collect::<String>()
}
