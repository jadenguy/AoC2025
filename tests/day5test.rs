#[cfg(test)]
mod tests {
    use aoc2025::day5::{count_fresh_ingredients, parse_db};
    #[test]
    fn test_parse_database() {
        let lines: Vec<&str> = r#"3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32"#
        .split("\n")
        .map(|x| x.trim())
        .collect();
        let expected_ranges = vec![(3, 5), (10, 14), (16, 20), (12, 18)];
        let available_ids = vec![1, 5, 8, 11, 17, 32];
        let db = parse_db(lines);
        assert_eq!(db.available_ingredient_ids, available_ids);
        assert_eq!(db.fresh_ingredient_id_ranges, expected_ranges);
    }
    #[test]
    fn test_count_fresh_avaliable_ingredients() {
        // arrange
        let lines: Vec<&str> = r#"3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32"#
        .split("\n")
        .map(|x| x.trim())
        .collect();
        let db = parse_db(lines);
        let expected_valid_id_count = 3;
        // act
        let fresh_ingredient_count: usize = count_fresh_ingredients(db);
        // assert
        assert_eq!(fresh_ingredient_count, expected_valid_id_count)
    }
}
