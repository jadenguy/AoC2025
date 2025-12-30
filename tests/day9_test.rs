use std::collections::HashSet;

use aoc2025::day9::{Coordinate, furthest_red_green_tiles, furthest_tiles, parse_tiles};

#[test]
fn test_furthest_tiles() {
    // arrange
    let tiles = parse_tiles(sample_data());
    let expected: HashSet<Coordinate> =
        HashSet::from_iter([Coordinate { x: 11, y: 1 }, Coordinate { x: 2, y: 5 }]);
    // act
    let (a, b, area) = furthest_tiles(&tiles).expect("returned none but should have returned some");
    // assert
    assert_eq!(HashSet::from_iter([a, b]), expected);
    assert_eq!(area, 50)
}
#[test]
fn test_furthest_red_green_tiles() {
    // arrange
    let tiles = parse_tiles(sample_data());
    let expected: HashSet<Coordinate> =
        HashSet::from_iter([Coordinate { x: 9, y: 5 }, Coordinate { x: 2, y: 3 }]);
    // act
    let (a, b, area) =
        furthest_red_green_tiles(&tiles).expect("returned none but should have returned some");
    // assert
    assert_eq!(HashSet::from_iter([a, b]), expected);
    assert_eq!(area, 24)
}
#[test]
fn test_parse_tiles() {
    // arrange
    let tile_strings = sample_data();
    // act
    let actual_tiles = parse_tiles(tile_strings);
    // assert
    assert_eq!(
        actual_tiles,
        [
            (7, 1),
            (11, 1),
            (11, 7),
            (9, 7),
            (9, 5),
            (2, 5),
            (2, 3),
            (7, 3),
        ]
    )
}
//
fn sample_data() -> Vec<&'static str> {
    r#"7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3
        "#
    .split("\n")
    .map(|x| x.trim())
    .collect()
}
