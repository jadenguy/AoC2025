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
#[cfg(test)]
mod tests {
    use crate::day2::{
        find_invalid_ids_lexicographically_by_two, lexicographical_lowest_bound,
        lexicographical_upper_bound,
    };

    static VERBOSE: bool = false;

    #[test]
    fn test_lexicographical_lowest_bound_755745207_into_3() {
        let number = "755745207";
        let divisor = 3;
        let expected = "755755755";
        let x = lexicographical_lowest_bound(number, divisor).repeat(divisor);
        assert_eq!(x, expected)
    }
    #[test]
    fn test_lexicographical_upper_bound_755766099_into_3() {
        let number = "755766099";
        let divisor = 3;
        let expected = "755755755";
        let x = lexicographical_upper_bound(number, divisor).repeat(divisor);
        assert_eq!(x, expected)
    }
    #[test]
    fn test_lexicographical_lowest_bound_1_into_2() {
        let number = "1";
        let divisor = 2;
        let expected = "11";
        let x = lexicographical_lowest_bound(number, divisor).repeat(divisor);
        assert_eq!(x, expected)
    }
    #[test]
    fn test_lexicographical_lowest_bound_11_into_2() {
        let number = "11";
        let divisor = 2;
        let expected = "11";
        let x = lexicographical_lowest_bound(number, divisor).repeat(divisor);
        assert_eq!(x, expected)
    }
    #[test]
    fn test_lexicographical_lowest_bound_11_into_3() {
        let number = "11";
        let divisor = 3;
        let expected = "111";
        let x = lexicographical_lowest_bound(number, divisor).repeat(divisor);
        assert_eq!(x, expected)
    }
    #[test]
    fn test_lexicographical_lowest_bound_95_into_2() {
        let number = "95";
        let divisor = 2;
        let expected = "99";
        let x = lexicographical_lowest_bound(number, divisor).repeat(divisor);
        assert_eq!(x, expected)
    }
    #[test]
    fn test_lexicographical_lowest_bound_95_into_3() {
        let number = "95";
        let divisor = 3;
        let expected = "111";
        let x = lexicographical_lowest_bound(number, divisor).repeat(divisor);
        assert_eq!(x, expected)
    }
    #[test]
    fn test_lexicographical_lowest_bound_995_into_3() {
        let number = "995";
        let divisor = 3;
        let expected = "999";
        let x = lexicographical_lowest_bound(number, divisor).repeat(divisor);
        assert_eq!(x, expected)
    }
    #[test]
    fn test_lexicographical_upper_bound_995_into_3() {
        let number = "995";
        let divisor = 3;
        let expected = "888";
        let x = lexicographical_upper_bound(number, divisor).repeat(divisor);
        assert_eq!(x, expected)
    }
    #[test]
    fn test_lexicographical_upper_bound_995_into_2() {
        let number = "995";
        let divisor = 2;
        let expected = "99";
        let x = lexicographical_upper_bound(number, divisor).repeat(divisor);
        assert_eq!(x, expected)
    }
    #[test]
    fn find_invalid_ids_lexicographically_123_1010() {
        // arrange
        let id_range = "123-1010";
        // act
        let invalid_ids = find_invalid_ids_lexicographically_by_two(id_range, VERBOSE);
        // assert
        assert_eq!(invalid_ids, [1010])
    }
    #[test]
    fn find_invalid_ids_lexicographically_111_222() {
        // arrange
        let id_range = "111-222";
        // act
        let invalid_ids = find_invalid_ids_lexicographically_by_two(id_range, VERBOSE);
        // assert
        assert_eq!(invalid_ids, [])
    }
    #[test]
    fn find_invalid_ids_lexicographically_4487_9581() {
        // arrange
        let id_range = "4487-9581";
        // act
        let invalid_ids = find_invalid_ids_lexicographically_by_two(id_range, VERBOSE);
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
    fn find_invalid_ids_lexicographically_910543_1082670() {
        // arrange
        let id_range = "910543-1082670";
        // act
        let invalid_ids = find_invalid_ids_lexicographically_by_two(id_range, VERBOSE);
        // assert
        assert_eq!(
            invalid_ids,
            [
                910910, 911911, 912912, 913913, 914914, 915915, 916916, 917917, 918918, 919919,
                920920, 921921, 922922, 923923, 924924, 925925, 926926, 927927, 928928, 929929,
                930930, 931931, 932932, 933933, 934934, 935935, 936936, 937937, 938938, 939939,
                940940, 941941, 942942, 943943, 944944, 945945, 946946, 947947, 948948, 949949,
                950950, 951951, 952952, 953953, 954954, 955955, 956956, 957957, 958958, 959959,
                960960, 961961, 962962, 963963, 964964, 965965, 966966, 967967, 968968, 969969,
                970970, 971971, 972972, 973973, 974974, 975975, 976976, 977977, 978978, 979979,
                980980, 981981, 982982, 983983, 984984, 985985, 986986, 987987, 988988, 989989,
                990990, 991991, 992992, 993993, 994994, 995995, 996996, 997997, 998998, 999999
            ]
        )
    }
    #[test]
    fn find_invalid_ids_lexicographically_72798_159206() {
        // arrange
        let id_range = "72798-159206";
        // act
        let invalid_ids = find_invalid_ids_lexicographically_by_two(id_range, VERBOSE);
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
        let invalid_ids = find_invalid_ids_lexicographically_by_two(id_range, VERBOSE);
        // assert
        assert_eq!(invalid_ids, [])
    }
    #[test]
    fn find_invalid_ids_lexicographically_95_115() {
        // arrange
        let id_range = "95-115";
        // act
        let invalid_ids = find_invalid_ids_lexicographically_by_two(id_range, VERBOSE);
        // assert
        assert_eq!(invalid_ids, [99])
    }
    #[test]
    fn find_invalid_ids_lexicographically_11_22() {
        // arrange
        let id_range = "11-22";
        // act
        let invalid_ids = find_invalid_ids_lexicographically_by_two(id_range, VERBOSE);
        // assert
        assert_eq!(invalid_ids, [11, 22])
    }
    #[test]
    fn find_invalid_ids_lexicographically_sample_data() {
        // arrange
        let id_ranges = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
            1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,
            2121212118-2121212124"
            .split(',')
            .map(|x| x.trim());
        // act

        let invalid_ids = id_ranges
            .map(|id_range| {
                find_invalid_ids_lexicographically_by_two(id_range, VERBOSE)
                    .iter()
                    .sum::<u64>()
            })
            .sum::<u64>();
        // assert
        assert_eq!(invalid_ids, 1227775554)
    }
    use std::collections::HashSet;

    use crate::day2::find_invalid_ids_lexicographically;

    #[test]
    fn find_invalid_ids_lexicographically_sample_data_n_count() {
        // arrange
        let id_ranges = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
            1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,
            2121212118-2121212124"
            .split(',')
            .map(|x| x.trim());
        // act

        let invalid_ids = id_ranges
            .map(|id_range| {
                find_invalid_ids_lexicographically(id_range, VERBOSE)
                    .iter()
                    .sum::<u64>()
            })
            .sum::<u64>();
        // assert
        assert_eq!(invalid_ids, 4174379265)
    }
    #[test]
    fn find_invalid_ids_lexicographically_answer_n_count() {
        // arrange
        let mut id_ranges: Vec<&str> = "4487-9581,755745207-755766099,
            954895848-955063124,4358832-4497315,15-47,
            1-12,9198808-9258771,657981-762275,6256098346-6256303872,
            142-282,13092529-13179528,96201296-96341879,19767340-19916378,
            2809036-2830862,335850-499986,172437-315144,764434-793133,
            910543-1082670,2142179-2279203,6649545-6713098,6464587849-6464677024,
            858399-904491,1328-4021,72798-159206,89777719-90005812,91891792-91938279,
            314-963,48-130,527903-594370,24240-60212\n"
            .split(',')
            .map(|x| x.trim())
            .collect();
        // act
        id_ranges.sort_by_key(|a| (a.len(), a.to_string()));
        let invalid_unique: HashSet<u64> = HashSet::from_iter(
            id_ranges
                .iter()
                .map(|id_range| find_invalid_ids_lexicographically(id_range, VERBOSE))
                .flatten(),
        );
        let mut x: Vec<u64> = invalid_unique.iter().map(|n| n.to_owned()).collect();
        x.sort();
        let mut y: u64 = 0;
        x.into_iter().for_each(|v| {
            y += v;
            if VERBOSE {
                println!("{} added to sum, now {}", v, y);
            }
        });
        let invalid_sum: u64 = invalid_unique.iter().sum();
        // assert
        assert_eq!(invalid_sum, 31755323497);
        assert_eq!(invalid_sum, y)
    }
    #[test]
    fn find_invalid_ids_lexicographically_95_115_n_count() {
        // arrange
        let id_range = "95-115";
        let mut expected: HashSet<u64> = HashSet::new();
        expected.insert(99);
        expected.insert(111);
        // act
        let invalid_ids = find_invalid_ids_lexicographically(id_range, VERBOSE);
        // assert
        assert_eq!(invalid_ids, expected)
    }
    #[test]
    fn find_invalid_ids_lexicographically_755745207_755766099() {
        // arrange
        let id_range = "755745207-755766099";
        let mut expected: HashSet<u64> = HashSet::new();
        expected.insert(755755755);
        // act
        let invalid_ids = find_invalid_ids_lexicographically(id_range, VERBOSE);
        // assert
        assert_eq!(invalid_ids, expected)
    }
    #[test]
    fn find_invalid_ids_lexicographically_314_963() {
        // arrange
        let id_range = "314-963";
        let expected: HashSet<u64> = HashSet::from_iter([333, 444, 555, 666, 777, 888]);
        // act
        let invalid_ids = find_invalid_ids_lexicographically(id_range, VERBOSE);
        // assert
        assert_eq!(invalid_ids, expected)
    }
}
