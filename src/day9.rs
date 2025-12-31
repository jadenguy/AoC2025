use std::collections::HashSet;

pub fn furthest_red_green_tiles(tiles: &Vec<Coordinate>) -> Option<(Coordinate, Coordinate, i64)> {
    let len = tiles.len();
    if len == 0 {
        return None;
    }
    // let line_segments: HashSet<LineSegment> = generate_segments(tiles, len);
    // let pairwise_areas = get_all_pairs(tiles);
    // let best: Option<(Coordinate, Coordinate, i64)> =
    //     Some(pairwise_areas.iter().filter_map(|x| process(&x)).first());
    None
}

pub fn generate_segments(tiles: &Vec<Coordinate>, len: usize) -> HashSet<LineSegment> {
    let mut line_segments: HashSet<LineSegment> = HashSet::new();
    for a_index in 0..len {
        let a = tiles[a_index];
        let b = tiles[match a_index {
            i if i == len - 1 => 0,
            i => i + 1,
        }];
        line_segments.insert(LineSegment::from_coodinates(a, b));
    }
    line_segments
}
impl LineSegment {
    fn from_coodinates(a: Coordinate, b: Coordinate) -> LineSegment {
        match (a, b) {
            (a, b) if a.x == b.x && a.y < b.y => LineSegment::Vertical {
                x: a.x,
                y_low: a.y,
                y_high: b.y,
            },
            (a, b) if a.x == b.x => LineSegment::Vertical {
                x: a.x,
                y_low: b.y,
                y_high: a.y,
            },
            (a, b) if a.y == b.y && a.x < b.x => LineSegment::Horizontal {
                x_low: a.x,
                x_high: b.x,
                y: a.y,
            },

            (a, b) if a.y == b.y => LineSegment::Horizontal {
                x_low: b.x,
                x_high: a.x,
                y: a.y,
            },
            _ => LineSegment::Error,
        }
    }
}
pub fn furthest_tiles(tiles: &Vec<Coordinate>) -> Option<(Coordinate, Coordinate, i64)> {
    let mut pairwise_areas = get_all_pairs(tiles);
    if pairwise_areas.len() > 0 {
        let result = pairwise_areas.remove(0);
        return Some(result);
    }
    None
}

fn get_all_pairs(tiles: &Vec<Coordinate>) -> Vec<(Coordinate, Coordinate, i64)> {
    let mut pairwise_areas: Vec<(Coordinate, Coordinate, i64)> = Vec::new();
    for first in 0..tiles.len() {
        let a = tiles[first];
        for second in (first + 1)..tiles.len() {
            let b = tiles[second];
            let area_calc = (1 + (a.x - b.x).abs()) * (1 + (a.y - b.y).abs());
            pairwise_areas.push((a, b, area_calc));
        }
    }
    pairwise_areas.sort_by_key(|x| -x.2);
    pairwise_areas
}
pub fn parse_tiles(tile_strings: Vec<&str>) -> Vec<Coordinate> {
    tile_strings
        .iter()
        .filter_map(|s| {
            if let Some((a, b)) = s.split_once(",")
                && let Ok(x) = a.parse::<i64>()
                && let Ok(y) = b.parse::<i64>()
            {
                return Some(Coordinate { x: x, y: y });
            }
            None
        })
        .collect()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Coordinate {
    pub x: i64,
    pub y: i64,
}
impl PartialEq<(i64, i64)> for Coordinate {
    fn eq(&self, other: &(i64, i64)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum LineSegment {
    Vertical { x: i64, y_low: i64, y_high: i64 },
    Horizontal { x_low: i64, x_high: i64, y: i64 },
    Error,
}
#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::day9::{Coordinate, furthest_red_green_tiles, furthest_tiles, parse_tiles};

    #[test]
    fn test_furthest_tiles() {
        // arrange
        let tiles = parse_tiles(sample_data());
        let expected: HashSet<Coordinate> =
            HashSet::from_iter([Coordinate { x: 11, y: 1 }, Coordinate { x: 2, y: 5 }]);
        // act
        let (a, b, area) =
            furthest_tiles(&tiles).expect("returned none but should have returned some");
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
}
