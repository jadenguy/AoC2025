#[cfg(test)]
mod tests {
    use aoc2025::day6::convert_worksheet_to_problems;
    use aoc2025::day6::generate_ast_from_problem;

    #[test]
    fn test_generate_ast_and_eval() {
        // arrange
        let data: Vec<String> = ["123", "45", "6", "*"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected_product = 33210;
        // act
        let actual_product: i64 = generate_ast_from_problem(data).unwrap().evaluate();
        // assert
        assert_eq!(actual_product, expected_product)
    }
    #[test]
    fn test_generate_asts_from_worksheet() {
        // arrange
        let worksheet: Vec<String> = sample_data();
        let expected_sum_of_eval = 33210;
        // act
        let problems = convert_worksheet_to_problems(worksheet);
        let actual_sum_of_eval: i64 = 0;
        // generate_ast_from_problem(worksheet).unwrap().evaluate();
        // assert
        assert_eq!(actual_sum_of_eval, expected_sum_of_eval)
    }
    fn sample_data() -> Vec<String> {
        let lines: Vec<String> = r#"123 328  51 64
         45 64  387 23
          6 98  215 314
        *   +   *   +  "#
            .split("\n")
            .map(|x| x.trim().to_string())
            .collect();
        lines
    }
}
