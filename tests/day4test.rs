#[cfg(test)]
mod tests {
    use aoc2025::day4::{convert_lines_to_board, find_isolated_rolls};
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
        let board = convert_lines_to_board(board_string);
        let x = find_isolated_rolls(board);
        assert_eq!(x, 13)
    }
}
