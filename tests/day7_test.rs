#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use aoc2025::day7::{parse_manifold_strings, process_manifold};

    #[test]
    fn test_parse_manifold_strings() {
        // arrange
        let sample_data = sample_data();
        // act
        let actual_manifold = parse_manifold_strings(sample_data);
        // assert
        let first_emitter: (usize, usize) = *actual_manifold.emitters.iter().nth(0).unwrap();
        let split: HashSet<(usize, usize)> = actual_manifold.splitters;
        assert_eq!(first_emitter, (0, 7));
        assert_eq!(
            split,
            HashSet::from_iter([
                (02, 07),
                (04, 06),
                (04, 08),
                (06, 05),
                (06, 07),
                (06, 09),
                (08, 04),
                (08, 06),
                (08, 10),
                (10, 03),
                (10, 05),
                (10, 09),
                (10, 11),
                (12, 02),
                (12, 06),
                (12, 12),
                (14, 01),
                (14, 03),
                (14, 05),
                (14, 07),
                (14, 09),
                (14, 13),
            ])
        );
    }
    #[test]
    fn test_process_manifold() {
        // arrange
        let sample_data = sample_data();
        let actual_manifold = parse_manifold_strings(sample_data);
        // act
        let processed_output = process_manifold(actual_manifold).0;
        // assert
        assert_eq!(processed_output, 21);
    }
    #[test]
    fn test_process_quantum_manifold() {
        // arrange
        let sample_data = sample_data();
        let actual_manifold = parse_manifold_strings(sample_data);
        // act
        let processed_output = process_manifold(actual_manifold).1;
        // assert
        assert_eq!(processed_output, 40);
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
