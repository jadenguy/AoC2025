use std::collections::HashMap;

pub fn find_isolated_rolls(board: HashMap<Coordinate, char>) -> Vec<Coordinate> {
    let mut ret: Vec<Coordinate> = Vec::new();
    let (min_row, min_col) = <(i32, i32)>::from(board.keys().min().unwrap());
    let (max_row, max_col) = <(i32, i32)>::from(board.keys().max().unwrap());
    for row_num in min_row..=max_row {
        for col_num in min_col..=max_col {
            let coord = Coordinate {
                row_num: row_num,
                col_num: col_num,
            };
            if let Some(n) = board.get(&coord) {
                if n.to_owned() == '@' {
                    let neighbor_count = count_neighbors(&board, row_num, col_num, &coord);
                    add_if(&mut ret, row_num, col_num, coord, neighbor_count, n);
                }
            }
        }
    }
    println!();
    println!("{},{} to {},{}", min_row, min_col, max_row, max_col);
    ret
}

fn add_if(
    ret: &mut Vec<Coordinate>,
    row_num: i32,
    col_num: i32,
    coord: Coordinate,
    neighbor_count: u8,
    n: &char,
) {
    if neighbor_count < 4 {
        println!("({},{}) {}", row_num, col_num, n);
        ret.push(coord);
    }
}

fn count_neighbors(
    board: &HashMap<Coordinate, char>,
    row_num: i32,
    col_num: i32,
    coord: &Coordinate,
) -> u8 {
    let mut neighbor_count: u8 = 0;
    for n_row in (row_num - 1)..=(row_num + 1) {
        for n_col in (col_num - 1)..=(col_num + 1) {
            let n_coord = Coordinate {
                row_num: n_row,
                col_num: n_col,
            };
            if let Some(o) = board.get(&n_coord) {
                if *coord != n_coord && o.to_owned() == '@' {
                    neighbor_count += 1;
                }
            }
        }
    }
    neighbor_count
}
#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Coordinate {
    pub row_num: i32,
    pub col_num: i32,
}
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Position(pub Coordinate, pub char);

impl From<&Coordinate> for (i32, i32) {
    fn from(e: &Coordinate) -> (i32, i32) {
        let &Coordinate { row_num, col_num } = e;
        (row_num, col_num)
    }
}
