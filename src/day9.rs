use std::collections::HashSet;

pub fn furthest_red_green_tiles(tiles: &Vec<Coordinate>) -> Option<(Coordinate, Coordinate, i64)> {
    let pairwise_areas = get_all_pairs_sorted_descending(tiles);
    let mut line_segments: Vec<(Coordinate, Coordinate)> = Vec::new();
    for index in 0..tiles.len() {
        let a = tiles[index];
        let b = match index {
            i if i + 1 == tiles.len() => tiles[0],
            _ => tiles[index + 1],
        };
        
        line_segments.push((a, b));
    }
    'bounding_box_check: for (a, b, dist) in pairwise_areas {
        println!(
            "Bounding Box {},{} {},{} Area {}",
            a.row, a.col, b.row, b.col, dist
        );
        let (min_col, max_col) = match a.col {
            _ if a.col < b.col => (a.col, b.col),
            _ => (b.col, a.col),
        };
        let (min_row, max_row) = match a.row {
            _ if a.row < b.row => (a.row, b.row),
            _ => (b.row, a.row),
        };

        if line_segments
            .iter()
            .any(|&l| intersects_any(l, min_row, max_row, min_col, max_col))
        {
            continue 'bounding_box_check;
        }
        return Some((a, b, dist));
    }
    None
}

fn intersects_any(
    l: (Coordinate, Coordinate),
    min_row: i64,
    max_row: i64,
    min_col: i64,
    max_col: i64,
) -> bool {
    
}

fn get_green_tiles(tiles: &[Coordinate]) -> HashSet<Coordinate> {
    let mut red_or_green: HashSet<Coordinate> = HashSet::new();
    let (mut row_list, mut col_list): (Vec<i64>, Vec<i64>) =
        tiles.iter().map(|t| (t.row, t.col)).unzip();
    row_list.sort();
    col_list.sort();
    let min_row = *row_list.first().unwrap();
    let max_row = *row_list.last().unwrap();
    let min_col = *col_list.first().unwrap();
    let max_col = *col_list.last().unwrap();
    let mut unchecked: HashSet<Coordinate> = HashSet::new();
    for row in min_row..=max_row {
        for col in min_col..=max_col {
            unchecked.insert(Coordinate { row, col });
        }
    }
    for index in 0..tiles.len() {
        let a = tiles[index];
        let b = match index {
            i if i + 1 == tiles.len() => tiles[0],
            _ => tiles[index + 1],
        };
        for col in a.col..b.col {
            let on_line = Coordinate { row: a.row, col };
            unchecked.remove(&on_line);
            red_or_green.insert(on_line);
        }
        for col in b.col..=a.col {
            let on_line = Coordinate { row: a.row, col };
            unchecked.remove(&on_line);
            red_or_green.insert(on_line);
        }
        for row in a.row..=b.row {
            let on_line = Coordinate { row, col: a.col };
            unchecked.remove(&on_line);
            red_or_green.insert(on_line);
        }
        for row in b.row..=a.row {
            let on_line = Coordinate { row, col: a.col };
            unchecked.remove(&on_line);
            red_or_green.insert(on_line);
        }
    }
    // let's check every neighbor to every piece outside the area
    let mut candidates: Vec<Coordinate> = Vec::new();
    for row in min_row - 1..=max_row + 1 {
        candidates.push(Coordinate {
            row,
            col: min_col - 1,
        });
        candidates.push(Coordinate {
            row,
            col: max_col + 1,
        });
    }
    for col in min_col - 1..=max_col + 1 {
        candidates.push(Coordinate {
            row: min_row - 1,
            col: col,
        });
        candidates.push(Coordinate {
            row: max_row + 1,
            col: col,
        });
    }
    while let Some(candidate) = candidates.pop() {
        for row in candidate.row - 1..=candidate.row + 1 {
            for col in candidate.col - 1..=candidate.col + 1 {
                let neighbor = Coordinate { row, col };
                if unchecked.remove(&neighbor) {
                    candidates.push(neighbor)
                }
            }
        }
    }
    red_or_green.extend(unchecked);
    red_or_green
}

fn count_of_cells_outside_line(
    tiles: &[Coordinate],
    red_or_green: &HashSet<Coordinate>,
    box_min_row: i64,
    box_max_row: i64,
    box_min_col: i64,
    box_max_col: i64,
) -> usize {
    let mut outside = 0;
    let (rowl_list, col_list): (Vec<i64>, Vec<i64>) = tiles.iter().map(|t| (t.row, t.col)).unzip();
    let max_row = *rowl_list.iter().max().unwrap() + 1;
    let max_col = *col_list.iter().max().unwrap() + 1;
    for row in 0..=max_row {
        for col in 0..=max_col {
            let cell = &Coordinate { row, col };
            if tiles.contains(cell) {
                print!("#")
            } else if box_min_row <= row
                && row <= box_max_row
                && box_min_col <= col
                && col <= box_max_col
            {
                if red_or_green.contains(cell) {
                    print!("O")
                } else {
                    print!("!");
                    outside += 1
                }
            } else if red_or_green.contains(cell) {
                print!("G")
            } else {
                print!(".")
            }
        }
        println!()
    }
    outside
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
            Coordinate { col: 11, row: 1 },
            Coordinate { col: 2, row: 5 },
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

        // act
        let (_a, _b, area) =
            furthest_red_green_tiles(&tiles).expect("returned none but should have returned some");
        // assert
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
            .map(|(col, row)| Coordinate { row, col })
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
