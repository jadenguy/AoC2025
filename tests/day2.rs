#[cfg(test)]
mod tests {
    use aoc2025::day2::{parse_id_range, parse_id_ranges};
    #[test]
    fn parse_range_11_22() {
        // arrange
        let id_ranges = "11-22";
        // act
        let parsed_range = parse_id_range(id_ranges);
        // assert
        assert_eq!(parsed_range, (11, 22))
    }

    #[test]
    fn parse_() {
        // arrange
        let id_ranges = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,
        824824821-824824827,2121212118-2121212124";
        // act
        let parsed_ranges = parse_id_ranges(id_ranges);
        // assert
        assert_eq!(
            parsed_ranges,
            vec![
                (11, 22),
                (95, 115),
                (998, 1012),
                (1188511880, 1188511890),
                (222220, 222224),
                (1698522, 1698528),
                (446443, 446449),
                (38593856, 38593862),
                (565653, 565659),
                (824824821, 824824827),
                (2121212118, 2121212124)
            ]
        )
    }
    #[test]
    fn find_repeat_ranges_11_22() {
        // arrange
        let id_ranges = "11-22";
        // act
        let parsed_range = parse_id_range(id_ranges);
        // assert
        assert_eq!(parsed_range, (11, 22))
    }
}
