#[cfg(test)]
mod tests {
    use aoc2025::day4::{
        convert_lines_to_board, find_isolated_rolls, find_isolated_rolls_with_output,
    };
    #[test]
    fn test_find_rolls_p1() {
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
    #[test]
    fn test_find_rolls_p2() {
        let mut ret = 0usize;
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
        let mut board = convert_lines_to_board(board_string);
        let mut found = 1;
        while found > 0 {
            let x = find_isolated_rolls_with_output(board);
            found = x.0;
            board = x.1;
            ret += found;
        }

        assert_eq!(ret, 43)
    }
}
