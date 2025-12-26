use std::vec;

pub fn convert_worksheet_to_problems(data: Vec<String>) -> Vec<Vec<String>> {
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
}
