pub fn parse_id_ranges(id_ranges_string: &str) -> Vec<(i32, i32)> {
    id_ranges_string.split(",").map(parse_id_range).collect()
}
pub fn parse_id_range(s: &str) -> (i32, i32) {
    let x: Vec<String> = s.trim().split("-").map(|s| s.to_string()).collect();
    let start_string = x.first();
    let start_range: i32 = match start_string {
        Some(s) => s.parse::<i32>().unwrap(),
        None => 0,
    };
    let end_string = x.last();
    let end_range: i32 = match end_string {
        Some(s) => s.parse::<i32>().unwrap(),
        None => 0,
    };
    (start_range, end_range)
}
