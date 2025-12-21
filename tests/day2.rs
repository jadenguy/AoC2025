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
    fn find_invalid_ids_lexicographically_111_222() {
        // arrange
        let id_range = "111-222";
        // act
        let invalid_ids = find_invalid_ids_lexicographically(id_range, true);
        // assert
        assert_eq!(invalid_ids, [])
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
    fn find_invalid_ids_lexicographically_72798_159206() {
        // arrange
        let id_range = "72798-159206";
        // act
        let invalid_ids = find_invalid_ids_lexicographically(id_range, true);
        // assert
        assert_eq!(
            invalid_ids,
            [
                100100, 101101, 102102, 103103, 104104, 105105, 106106, 107107, 108108, 109109,
                110110, 111111, 112112, 113113, 114114, 115115, 116116, 117117, 118118, 119119,
                120120, 121121, 122122, 123123, 124124, 125125, 126126, 127127, 128128, 129129,
                130130, 131131, 132132, 133133, 134134, 135135, 136136, 137137, 138138, 139139,
                140140, 141141, 142142, 143143, 144144, 145145, 146146, 147147, 148148, 149149,
                150150, 151151, 152152, 153153, 154154, 155155, 156156, 157157, 158158, 159159
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
}
