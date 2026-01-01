pub fn furthest_red_green_tiles(tiles: &Vec<Coordinate>) -> Option<(Coordinate, Coordinate, i64)> {
    let len = tiles.len();
    if len == 0 {
        return None;
    }

    let pairwise_areas = get_all_pairs_sorted_descending(tiles);
    // let largest = pairwise_areas[0];
    // find_segments_hitting_point(line_segments, largest);
    'bounding_box_search: for (a, b, dist) in pairwise_areas {
        println!("Bounding Box {},{} {},{} Area {}", a.x, a.y, b.x, b.y, dist);
        let (box_min_x, box_max_x) = if a.x < b.x { (a.x, b.x) } else { (b.x, a.x) };
        let (box_min_y, box_max_y) = if a.y < b.y { (a.y, b.y) } else { (b.y, a.y) };
        illustrate(tiles, box_min_x, box_max_x, box_min_y, box_max_y);
        let piercing_box_indexes: Vec<Coordinate> = tiles
            .iter()
            .filter_map(|p| {
                // x inside box
                let x_ok = box_min_x < p.x && p.x < box_max_x;
                let y_ok = box_min_y < p.y && p.y < box_max_y;
                if x_ok && y_ok {
                    let coordinate = p.to_owned();
                    return Some(coordinate);
                }
                None
            })
            .collect();
        if piercing_box_indexes.len() > 0 {
            for p in piercing_box_indexes {
                println!(
                    " {},{} to {},{} pierced by {},{}",
                    a.x, a.y, b.x, b.y, p.x, p.y
                )
            }
            continue 'bounding_box_search;
        }
        return Some((a, b, dist));
    }
    None
}

fn illustrate(
    tiles: &[Coordinate],
    box_min_x: i64,
    box_max_x: i64,
    box_min_y: i64,
    box_max_y: i64,
) {
    let (x_list, y_list): (Vec<i64>, Vec<i64>) = tiles.iter().map(|t| (t.x, t.y)).unzip();
    let max_x = *x_list.iter().max().unwrap() + 1;
    let max_y = *y_list.iter().max().unwrap() + 1;
    for y in 0..=max_y {
        for x in 0..=max_x {
            if tiles.contains(&Coordinate { x: x, y: y }) {
                print!("#")
            } else if box_min_x <= x && x <= box_max_x && box_min_y <= y && y <= box_max_y {
                print!("O")
            } else {
                print!(".")
            }
        }
        println!()
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
    Vertical { x: i64, y_bottom: i64, y_top: i64 },
    Horizontal { x_left: i64, x_right: i64, y: i64 },
    Error,
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
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
