pub fn convert_worksheet_to_problems_cephalopod(data: &Vec<String>) -> Vec<Vec<String>> {
    let last = data.last().unwrap();
    let len = last.len();
    let mut op_row = last.chars().rev();
    let mut ret: Vec<Vec<String>> = Vec::new();
    let mut current_prob: Vec<String> = Vec::new();

    for col_num in 0..len {
        let col_index = len - col_num - 1;
        let char = op_row.next().unwrap();
        let mut col = String::from("");
        for row_index in 0..(data.len() - 1) {
            let row = data[row_index].to_owned();
            let ch = row.chars().nth(col_index).unwrap();

            col = format!("{}{}", col, ch);
        }
        if !col.trim().is_empty() {
            current_prob.push(col.trim().to_string());
            // println!("{}", col)
        }
        if !char.is_whitespace() {
            let op = last.chars().nth(col_index).unwrap();
            // println!("{}", op);
            // println!();
            current_prob.push(op.to_string());
            ret.push(current_prob);
            current_prob = Vec::new();
        }
    }
    ret
}
pub fn convert_worksheet_to_problems(data: &Vec<String>) -> Vec<Vec<String>> {
    let vec2d: Vec<Vec<String>> = data
        .iter()
        .map(|x| {
            x.split_whitespace()
                .filter(|x| !x.is_empty())
                .map(|x| x.to_string())
                .collect()
        })
        .collect();
    let row_count = vec2d.len();
    let col_count = vec2d[0].len();
    let mut ret: Vec<Vec<String>> = Vec::new();
    for col_num in 0..col_count {
        let column = get_column(&vec2d, row_count, col_num);
        ret.push(column);
    }
    ret
}

fn get_column(vec2d: &Vec<Vec<String>>, row_count: usize, col_num: usize) -> Vec<String> {
    let mut column: Vec<String> = Vec::new();
    for row_num in 0..row_count {
        let value: String = (*vec2d[row_num][col_num]).to_string();
        column.push(value)
    }
    column
}
pub fn generate_ast_from_problem(data: Vec<String>) -> Option<Expression> {
    if data.len() < 3 {
        // let n = format!("{}", data.join("\n"));
        // println!("{}", n);
        return None;
    }
    let tokens: Vec<Token> = data
        .iter()
        .map(|line| Token::from_string(&line).unwrap())
        .collect();
    let mut iter = tokens.iter().rev();
    let operation = Box::new(iter.next().unwrap());
    let left = iter.next().unwrap().to_value().unwrap();
    let right = iter.next().unwrap().to_value().unwrap();
    let mut ret = operation.to_operation(left, right);

    while let Some(next_right) = iter.next() {
        ret = operation.to_operation(ret.unwrap(), next_right.to_value().unwrap())
    }
    ret
}
#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Expression {
    Value(i64),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Token {
    MultiplyToken,
    DivideToken,
    AddToken,
    SubtractToken,
    ValueToken(String),
}

impl Token {
    fn from_string(s: &String) -> Option<Token> {
        match s.as_str() {
            "*" => Some(Token::MultiplyToken),
            "+" => Some(Token::AddToken),
            "-" => Some(Token::SubtractToken),
            "/" => Some(Token::DivideToken),
            _ => Some(Token::ValueToken(s.to_string())),
        }
    }
    fn to_operation(&self, left: Expression, right: Expression) -> Option<Expression> {
        match &self {
            Token::MultiplyToken => Some(Expression::Multiply(Box::new(left), Box::new(right))),
            Token::DivideToken => Some(Expression::Divide(Box::new(left), Box::new(right))),
            Token::AddToken => Some(Expression::Add(Box::new(left), Box::new(right))),
            Token::SubtractToken => Some(Expression::Subtract(Box::new(left), Box::new(right))),
            _ => None,
        }
    }
    fn to_value(&self) -> Option<Expression> {
        match &self {
            Token::ValueToken(x) => Some(Expression::Value(x.parse().unwrap())),
            _ => None,
        }
    }
}

impl Expression {
    pub fn evaluate(&self) -> i64 {
        match self {
            Expression::Value(v) => *v,
            Expression::Add(a, b) => a.evaluate() + b.evaluate(),
            Expression::Subtract(a, b) => a.evaluate() - b.evaluate(),
            Expression::Multiply(a, b) => a.evaluate() * b.evaluate(),
            Expression::Divide(a, b) => a.evaluate() / b.evaluate(),
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Expression::Value(v) => format!("{}", *v),
            Expression::Add(a, b) => format!("{}+{}", a.to_string(), b.to_string()),
            Expression::Subtract(a, b) => format!("{}-{}", a.to_string(), b.to_string()),
            Expression::Multiply(a, b) => format!("{}*{}", a.to_string(), b.to_string()),
            Expression::Divide(a, b) => format!("{}/{}", a.to_string(), b.to_string()),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::day6::{
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
