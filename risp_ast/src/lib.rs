#[derive(Debug, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Apply {
        operation: Operation,
        args: Box<(Expression, Expression)>,
    },
    Integer {
        value: i64,
    },
}
