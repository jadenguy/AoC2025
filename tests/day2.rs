#[cfg(test)]
mod tests {
    use aoc2025::day2::find_invalid_ids_lexicographically;
    #[test]
    fn find_invalid_ids_lexicographically_123_1010() {
        // arrange
        let id_range = "123-1010";
        // act
        let invalid_ids = find_invalid_ids_lexicographically(id_range, true);
        // assert
        assert_eq!(invalid_ids, [1010])
    }
    #[test]
    fn find_invalid_ids_lexicographically_4487_9581() {
        // arrange
        let id_range = "4487-9581";
        // act
        let invalid_ids = find_invalid_ids_lexicographically(id_range, true);
        // assert
        assert_eq!(
            invalid_ids,
            [
                4545, 4646, 4747, 4848, 4949, 5050, 5151, 5252, 5353, 5454, 5555, 5656, 5757, 5858,
                5959, 6060, 6161, 6262, 6363, 6464, 6565, 6666, 6767, 6868, 6969, 7070, 7171, 7272,
                7373, 7474, 7575, 7676, 7777, 7878, 7979, 8080, 8181, 8282, 8383, 8484, 8585, 8686,
                8787, 8888, 8989, 9090, 9191, 9292, 9393, 9494
            ]
        )
    }
    #[test]
    fn find_invalid_ids_lexicographically_222_333() {
        // arrange
        let id_range = "222-333";
        // act
        let invalid_ids = find_invalid_ids_lexicographically(id_range, true);
        // assert
        assert_eq!(invalid_ids, [])
    }
    #[test]
    fn find_invalid_ids_lexicographically_95_115() {
        // arrange
        let id_range = "95-115";
        // act
        let invalid_ids = find_invalid_ids_lexicographically(id_range, true);
        // assert
        assert_eq!(invalid_ids, [99])
    }
    #[test]
    fn find_invalid_ids_lexicographically_11_22() {
        // arrange
        let id_range = "11-22";
        // act
        let invalid_ids = find_invalid_ids_lexicographically(id_range, true);
        // assert
        assert_eq!(invalid_ids, [11, 22])
    }
    #[test]
    fn find_invalid_ids_lexicographically_sample_data() {
        // arrange
        let id_ranges = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".split(',');
        // act

        let invalid_ids = id_ranges
            .map(|id_range| {
                find_invalid_ids_lexicographically(id_range, true)
                    .iter()
                    .sum::<u64>()
            })
            .sum::<u64>();
        // assert
        assert_eq!(invalid_ids, 1227775554)
    }

    // #[test]
    // fn parse_range_11_22() {
    //     // arrange
    //     let id_ranges = "11-22";
    //     // act
    //     let parsed_range = parse_id_range(id_ranges);
    //     // assert
    //     assert_eq!(parsed_range, (11, 22))
    // }

    // #[test]
    // fn parse_() {
    //     // arrange
    //     let id_ranges = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
    //     1698522-1698528,446443-446449,38593856-38593862,565653-565659,
    //     824824821-824824827,2121212118-2121212124";
    //     // act
    //     let parsed_ranges = parse_id_ranges(id_ranges);
    //     // assert
    //     assert_eq!(
    //         parsed_ranges,
    //         vec![
    //             (11, 22),
    //             (95, 115),
    //             (998, 1012),
    //             (1188511880, 1188511890),
    //             (222220, 222224),
    //             (1698522, 1698528),
    //             (446443, 446449),
    //             (38593856, 38593862),
    //             (565653, 565659),
    //             (824824821, 824824827),
    //             (2121212118, 2121212124)
    //         ]
    //     )
    // }
    // #[test]
    // fn find_invalid_ids_in_range_10_10() {
    //     // arrange
    //     let range = (10, 10);
    //     // act
    //     let invalid_ids = find_invalid_ids(range);
    //     // assert
    //     assert_eq!(invalid_ids, [])
    // }
    // #[test]
    // fn find_invalid_ids_in_range_111_111() {
    //     // arrange
    //     let range = (111, 111);
    //     // act
    //     let invalid_ids = find_invalid_ids(range);
    //     // assert
    //     assert_eq!(invalid_ids, [])
    // }
    // #[test]
    // fn find_invalid_ids_in_range_11_11() {
    //     // arrange
    //     let range = (11, 11);
    //     // act
    //     let invalid_ids = find_invalid_ids(range);
    //     // assert
    //     assert_eq!(invalid_ids, [11])
    // }
    // #[test]
    // fn find_invalid_ids_in_range_11_22() {
    //     // arrange
    //     let range = (11, 22);
    //     // act
    //     let invalid_ids = find_invalid_ids(range);
    //     // assert
    //     assert_eq!(invalid_ids, [11])
    // }
    // #[test]
    // fn find_invalid_ids_in_range_1_2() {
    //     // arrange
    //     let range = (1, 2);
    //     // act
    //     let invalid_ids = find_invalid_ids(range);
    //     // assert
    //     assert_eq!(invalid_ids, [])
    // }
}
