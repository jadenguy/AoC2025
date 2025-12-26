#[cfg(test)]
mod tests {
    use aoc2025::day6::{
        convert_worksheet_to_problems, convert_worksheet_to_problems_cephalopod,
        generate_ast_from_problem,
    };

    #[test]
    fn test_convert_worksheet_to_problems_cephalopod() {
        // arrange
        let worksheet: Vec<String> = sample_data();
        let problem_a: Vec<String> = ["4", "431", "623", "+"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let problem_b: Vec<String> = ["175", "581", "32", "*"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let problem_c: Vec<String> = ["8", "248", "369", "+"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let problem_d: Vec<String> = ["356", "24", "1", "*"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        // act
        let problems = convert_worksheet_to_problems_cephalopod(&worksheet);
        // assert
        let mut x = problems.iter();
        assert_eq!(x.next(), Some(&problem_a));
        assert_eq!(x.next(), Some(&problem_b));
        assert_eq!(x.next(), Some(&problem_c));
        assert_eq!(x.next(), Some(&problem_d));
        assert_eq!(x.next(), None);
    }
    #[test]
    fn test_convert_worksheet_to_problems() {
        // arrange
        let worksheet: Vec<String> = sample_data();
        let problem_a: Vec<String> = ["123", "45", "6", "*"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let problem_b: Vec<String> = ["328", "64", "98", "+"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let problem_c: Vec<String> = ["51", "387", "215", "*"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let problem_d: Vec<String> = ["64", "23", "314", "+"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        // act
        let problems = convert_worksheet_to_problems(&worksheet);
        // assert
        let mut x = problems.iter();
        assert_eq!(x.next(), Some(&problem_a));
        assert_eq!(x.next(), Some(&problem_b));
        assert_eq!(x.next(), Some(&problem_c));
        assert_eq!(x.next(), Some(&problem_d));
        assert_eq!(x.next(), None);
    }
    #[test]
    fn test_ast_from_problem() {
        // arrange
        let probem: Vec<String> = ["123", "45", "6", "*"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected_product = 33210;
        // act
        let actual_product: i64 = generate_ast_from_problem(probem).unwrap().evaluate();
        // assert
        assert_eq!(actual_product, expected_product)
    }
    #[test]
    fn test_generate_asts_from_worksheet_and_eval() {
        // arrange
        let worksheet: Vec<String> = sample_data();
        let expected_sum_of_eval = 4277556;
        let problems = convert_worksheet_to_problems(&worksheet);
        // act

        let actual_sum_of_eval: i64 = problems
            .iter()
            .map(|problem| {
                generate_ast_from_problem(problem.to_owned())
                    .unwrap()
                    .evaluate()
            })
            .sum();
        assert_eq!(actual_sum_of_eval, expected_sum_of_eval)
    }
    #[test]
    fn test_generate_asts_from_cephalopod_worksheet_and_eval() {
        // arrange
        let worksheet: Vec<String> = sample_data();
        let expected_sum_of_eval = 3263827;
        let problems = convert_worksheet_to_problems_cephalopod(&worksheet);
        // act

        let actual_sum_of_eval: i64 = problems
            .iter()
            .map(|problem| {
                generate_ast_from_problem(problem.to_owned())
                    .unwrap()
                    .evaluate()
            })
            .sum();
        assert_eq!(actual_sum_of_eval, expected_sum_of_eval)
    }
    fn sample_data() -> Vec<String> {
        let lines: Vec<String> =
            "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  "
                .split("\n")
                .map(|x| x.to_string())
                .collect();
        lines
    }
}
