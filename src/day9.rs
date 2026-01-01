use std::collections::HashSet;

type LineSegment = (Coordinate, Coordinate);
pub fn furthest_red_green_tiles(tiles: &Vec<Coordinate>) -> Option<(Coordinate, Coordinate, i64)> {
    let len = tiles.len();
    if len == 0 {
        return None;
    }
    let green_tiles = get_green_tiles(tiles);
    let pairwise_areas = get_all_pairs_sorted_descending(tiles);
    // let largest = pairwise_areas[0];
    // find_segments_hitting_point(line_segments, largest);
    'bounding_box_search: for (a, b, dist) in pairwise_areas {
        println!(
            "Bounding Box {},{} {},{} Area {}",
            a.row, a.col, b.row, b.col, dist
        );
        let (box_min_row, box_max_row) = if a.row < b.row {
            (a.row, b.row)
        } else {
            (b.row, a.row)
        };
        let (box_min_col, box_max_col) = if a.col < b.col {
            (a.col, b.col)
        } else {
            (b.col, a.col)
        };
        illustrate(tiles, box_min_row, box_max_row, box_min_col, box_max_col);
        let piercing_box_indexes: Vec<Coordinate> = tiles
            .iter()
            .filter_map(|p| {
                // x inside box
                let row_ok = box_min_row < p.row && p.row < box_max_row;
                let col_ok = box_min_col < p.col && p.col < box_max_col;
                if row_ok && col_ok {
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
                    a.row, a.col, b.row, b.col, p.row, p.col
                )
            }
            continue 'bounding_box_search;
        }
        return Some((a, b, dist));
    }
    None
}

fn get_green_tiles(tiles: &[Coordinate]) -> Vec<Coordinate> {
    let mut green: Vec<Coordinate> = Vec::new();
    let mut lines: HashSet<LineSegment> = HashSet::new();

    let (row_list, col_list): (Vec<i64>, Vec<i64>) = tiles.iter().map(|t| (t.row, t.col)).unzip();

    let min_row = *row_list.iter().min().unwrap();
    let min_col = *col_list.iter().min().unwrap();
    let max_row = *row_list.iter().max().unwrap();
    let max_col = *col_list.iter().max().unwrap();
    let mut checked: HashSet<Coordinate> = HashSet::new();
    let mut unchecked: HashSet<Coordinate> = HashSet::new();
    for row in min_row..=max_row {
        for col in min_col..=max_col {
            unchecked.insert(Coordinate { row, col });
        }
    }
    let mut left_most: LineSegment = (
        Coordinate {
            row: min_row,
            col: max_col,
        },
        Coordinate {
            row: max_row,
            col: max_col,
        },
    );
    for index in 0..tiles.len() {
        let a = tiles[index];
        let b = match index {
            bi if bi + 1 == tiles.len() => tiles[0],
            _ => tiles[index + 1],
        };
        lines.insert((a, b));
        if
            a.col <  left_most.0.col
            && a.col == b.col
            && a.row - a.col.abs() > 2
        {
            left_most = (a, b) match
            {

            };
        }
    }
    green
}

fn illustrate(
    tiles: &[Coordinate],
    box_min_row: i64,
    box_max_row: i64,
    box_min_col: i64,
    box_max_col: i64,
) {
    let (rowl_list, col_list): (Vec<i64>, Vec<i64>) = tiles.iter().map(|t| (t.row, t.col)).unzip();
    let max_row = *rowl_list.iter().max().unwrap() + 1;
    let max_col = *col_list.iter().max().unwrap() + 1;
    for row in 0..=max_row {
        for col in 0..=max_col {
            if tiles.contains(&Coordinate { row, col }) {
                print!("#")
            } else if box_min_row <= row
                && row <= box_max_row
                && box_min_col <= col
                && col <= box_max_col
            {
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
            let area_calc = (1 + (a.row - b.row).abs()) * (1 + (a.col - b.col).abs());
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
                && let Ok(col) = a.parse::<i64>()
                && let Ok(row) = b.parse::<i64>()
            {
                return Some(Coordinate { row, col });
            }
            None
        })
        .collect()
}
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Coordinate {
    pub row: i64,
    pub col: i64,
}
impl PartialEq<(i64, i64)> for Coordinate {
    fn eq(&self, other: &(i64, i64)) -> bool {
        self.row == other.0 && self.col == other.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    #[test]
    fn test_furthest_tiles() {
        // arrange
        let tiles = parse_tiles(sample_data());
        let expected: HashSet<Coordinate> = HashSet::from_iter([
            Coordinate { row: 11, col: 1 },
            Coordinate { row: 2, col: 5 },
        ]);
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
            HashSet::from_iter([Coordinate { row: 9, col: 5 }, Coordinate { row: 2, col: 3 }]);
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
