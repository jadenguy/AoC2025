pub fn furthest_red_green_tiles(tiles: &Vec<Coordinate>) -> Option<(Coordinate, Coordinate, i64)> {
    let len = tiles.len();
    if len == 0 {
        return None;
    }
    let line_segments: Vec<LineSegment> = generate_segments(tiles, len);
    let pairwise_areas = get_all_pairs_sorted_descending(tiles);
    // let largest = pairwise_areas[0];
    // find_segments_hitting_point(line_segments, largest);
    for (a, b, dist) in pairwise_areas {
        let (min_x, max_x) = if a.x < b.x { (a.x, b.x) } else { (b.x, a.x) };
        let (min_y, max_y) = if a.y < b.y { (a.y, b.y) } else { (b.y, a.y) };
        let up = LineSegment::from_i64(min_x, min_y, min_x, max_y);
        let right = LineSegment::from_i64(min_x, max_y, max_x, max_y);
        let down = LineSegment::from_i64(max_x, max_y, max_x, min_y);
        let left = LineSegment::from_i64(max_x, min_y, max_x, min_y);
        let hits_up = find_intersecting_segments(&line_segments, up);
        for i in hits_up {
            let seg = line_segments[i];
            println!("{}", seg.to_string())
        }
    }

    None
}

fn find_intersecting_segments(line_segments: &Vec<LineSegment>, up: LineSegment) -> Vec<usize> {
    let hits_up: Vec<usize> = line_segments
        .iter()
        .enumerate()
        .filter_map(|(i, n)| -> Option<usize> {
            if n.intersects_perpendicular(up) {
                return None;
            }
            return Some(i);
        })
        .collect();
    hits_up
}

pub fn generate_segments(tiles: &Vec<Coordinate>, len: usize) -> Vec<LineSegment> {
    let mut line_segments: Vec<LineSegment> = Vec::new();
    for a_index in 0..len {
        let a = tiles[a_index];
        let b = tiles[match a_index {
            i if i == len - 1 => 0,
            i => i + 1,
        }];
        line_segments.push(LineSegment::from_coordinates(a, b));
    }
    line_segments
}
impl LineSegment {
    fn to_string(&self) -> String {
        match &self {
            LineSegment::Horizontal { x_low, x_high, y } => {
                format!("Horizontal from {},{} to {},{}", x_low, y, x_high, y)
            }
            LineSegment::Vertical { x, y_low, y_high } => {
                format!("Vertical from {},{} to {},{}", x, y_low, x, y_high)
            }
            _ => format!("false"),
        }
    }
    fn from_i64(a_x: i64, a_y: i64, b_x: i64, b_y: i64) -> LineSegment {
        LineSegment::from_coordinates(Coordinate { x: a_x, y: a_y }, Coordinate { x: b_x, y: b_y })
    }
    fn from_coordinates(a: Coordinate, b: Coordinate) -> LineSegment {
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
    fn intersects_perpendicular(&self, other: Self) -> bool {
        match (self, other) {
            (
                &LineSegment::Vertical { x, y_low, y_high },
                LineSegment::Horizontal { x_low, x_high, y },
            )
            | (
                &LineSegment::Horizontal { x_low, x_high, y },
                LineSegment::Vertical { x, y_low, y_high },
            ) if x_low <= x && x <= x_high && y_low <= y && y <= y_high => true,
            _ => false,
        }
    }
    fn intersects_including_perpendicular(&self, other: Self) -> bool {
        match (self, other) {
            (
                &LineSegment::Vertical { x, y_low, y_high },
                LineSegment::Horizontal { x_low, x_high, y },
            )
            | (
                &LineSegment::Horizontal { x_low, x_high, y },
                LineSegment::Vertical { x, y_low, y_high },
            ) if x_low <= x && x <= x_high && y_low <= y && y <= y_high => true,
            (
                &LineSegment::Vertical {
                    x: x1,
                    y_low: yl1,
                    y_high: yh1,
                },
                LineSegment::Vertical {
                    x: x2,
                    y_low: yl2,
                    y_high: yh2,
                },
            ) if x1 == x2 && yl1 <= yh2 && yl2 <= yh1 => true,

            (
                &LineSegment::Horizontal {
                    x_low: xl1,
                    x_high: xh1,
                    y: y1,
                },
                LineSegment::Horizontal {
                    x_low: xl2,
                    x_high: xh2,
                    y: y2,
                },
            ) if (y1 == y2) && xl1 <= xh2 && xl2 <= xh1 => true,
            _ => false,
        }
    }
}
pub fn furthest_tiles(tiles: &Vec<Coordinate>) -> Option<(Coordinate, Coordinate, i64)> {
    let mut pairwise_areas = get_all_pairs_sorted_descending(tiles);
    if pairwise_areas.len() > 0 {
        let result = pairwise_areas.remove(0);
        return Some(result);
    }
    None
}
fn get_all_pairs_sorted_descending(tiles: &Vec<Coordinate>) -> Vec<(Coordinate, Coordinate, i64)> {
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
