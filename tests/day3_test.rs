use aoc2025::day3::largest_joltage;
#[test]
fn test_largest_joltage_987654321111111_98() {
    // arrange
    let input = "987654321111111";
    let expected = 98;
    // act
    let actual = largest_joltage(input.to_string(), 2);
    // assert
    assert_eq!(actual, expected.to_string())
}
#[test]
fn test_largest_joltage_811111111111119_89() {
    // arrange
    let input = "811111111111119";
    let expected = 89;
    // act
    let actual = largest_joltage(input.to_string(), 2);
    // assert
    assert_eq!(actual, expected.to_string())
}
#[test]
fn test_largest_joltage_1919_99() {
    // arrange
    let input = "1919";
    let expected = 99;
    // act
    let actual = largest_joltage(input.to_string(), 2);
    // assert
    assert_eq!(actual, expected.to_string())
}
#[test]
fn test_largest_joltage_234234234234278_78() {
    // arrange
    let input = "234234234234278";
    let expected = 78;
    // act
    let actual = largest_joltage(input.to_string(), 2);
    // assert
    assert_eq!(actual, expected.to_string())
}
#[test]
fn test_largest_joltage_9876543210_987654321() {
    // arrange
    let input = "9800076000540003210";
    let expected = 987654321;
    // act
    let actual = largest_joltage(input.to_string(), 9);
    // assert
    assert_eq!(actual, expected.to_string())
}
