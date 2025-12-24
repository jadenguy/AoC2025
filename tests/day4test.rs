#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use aoc2025::day4::{Coordinate, find_isolated_rolls};
    #[test]
    fn test_find_rolls() {
        let board_string: Vec<String> = r#"
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@."#
            .split("\n")
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty())
            .collect();
        let board: HashMap<Coordinate, char> = HashMap::from_iter(
            board_string
                .iter()
                .enumerate()
                .map(|(row_num, row)| {
                    // println!();
                    // print!("{}: ", row_num);
                    let ret = row.chars().enumerate().map(move |(col_num, cell)| {
                        // print!("({},{}) {}", row_num, col_num, cell);
                        (
                            Coordinate {
                                row_num: row_num as i32,
                                col_num: col_num as i32,
                            },
                            cell,
                        )
                    });
                    ret
                })
                .flatten(),
        );
        let x = find_isolated_rolls(board);
        assert_eq!(x.iter().count(), 13)
    }
}
