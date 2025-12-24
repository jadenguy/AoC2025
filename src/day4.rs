use std::collections::HashMap;
type Board = HashMap<Coordinate, char>;
pub fn find_isolated_rolls(board: Board) -> usize {
    find_isolated_rolls_with_output(board).0
}
pub fn find_isolated_rolls_with_output(board: Board) -> (usize, Board) {
    let mut updated: Board = board.iter().map(|(k, v)| (k.clone(), *v)).collect();
    let mut ret: Vec<Coordinate> = Vec::new();
    let (rows, cols): (Vec<i32>, Vec<i32>) = board.keys().map(|e| <(i32, i32)>::from(e)).unzip();
    let max_row = rows.iter().max().unwrap().to_owned();
    let max_col = cols.iter().max().unwrap().to_owned();
    for row_num in 0..=max_row {
        for col_num in 0..=max_col {
            let coord = Coordinate {
                row_num: row_num,
                col_num: col_num,
            };
            if let Some(n) = board.get(&coord) {
                if n.to_owned() == '@' {
                    let neighbor_count = count_neighbors(&board, row_num, col_num, &coord);
                    if add_if(&mut ret, &coord, neighbor_count) {
                        updated.remove(&coord);
                    }
                }
            }
        }
    }
    (ret.len(), updated)
}

fn add_if(ret: &mut Vec<Coordinate>, coord: &Coordinate, neighbor_count: u8) -> bool {
    let is_under = neighbor_count < 4;
    if is_under {
        ret.push(coord.to_owned());
    }
    is_under
}

fn count_neighbors(board: &Board, row_num: i32, col_num: i32, coord: &Coordinate) -> u8 {
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
#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
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
pub fn convert_lines_to_board(board_rows: Vec<String>) -> Board {
    HashMap::from_iter(
        board_rows
            .iter()
            .enumerate()
            .map(|(row_num, row)| {
                row.chars().enumerate().map(move |(col_num, cell)| {
                    (
                        Coordinate {
                            row_num: row_num as i32,
                            col_num: col_num as i32,
                        },
                        cell,
                    )
                })
            })
            .flatten(),
    )
}
pub
fn print_board(board: &std::collections::HashMap<Coordinate, char>, max_row: i32, max_col: i32) {
    print!("{}[2J", 27 as char);
    for row_num in 0..=max_row {
        for col_num in 0..=max_col {
            let coord = Coordinate {
                row_num: row_num,
                col_num: col_num,
            };
            if let Some(n) = board.get(&coord) {
                print!("{}", n);
            } else {
                print!(" ")
            }
        }
        println!();
    }
}
