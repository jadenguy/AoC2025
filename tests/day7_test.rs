#[cfg(test)]
mod tests {
    use aoc2025::day7::{parse_manifold_strings, process_manifold};

    #[test]
    fn test_parse_manifold_strings() {
        // arrange
        let sample_data = sample_data();
        // act
        let actual_manifold = parse_manifold_strings(sample_data);
        // assert
        let start: (i32, i32) = *actual_manifold
            .iter()
            .find(|(_, val)| **val == 'S')
            .unwrap()
            .0;
        let split: Vec<(i32, i32)> = actual_manifold
            .iter()
            .filter(|(_, v)| **v == '^')
            .map(|(&k, _)| k)
            .collect();
        assert_eq!(start, (0, 7));
        assert!(split.iter().any(|&x| x == (2, 7)));
    }
    #[test]
    fn test_process_manifold() {
        // arrange
        let sample_data = sample_data();
        let actual_manifold = parse_manifold_strings(sample_data);
        // act
        let processed_output = process_manifold(actual_manifold).len();
        // assert
        assert_eq!(processed_output, 21);
    }
    fn sample_data() -> Vec<&'static str> {
        r#".......S.......
           ...............
           .......^.......
           ...............
           ......^.^......
           ...............
           .....^.^.^.....
           ...............
           ....^.^...^....
           ...............
           ...^.^...^.^...
           ...............
           ..^...^.....^..
           ...............
           .^.^.^.^.^...^.
           ..............."#
            .split("\n")
            .map(|x| x.trim())
            .collect()
    }
}
