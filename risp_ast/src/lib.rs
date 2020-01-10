#[derive(Debug)]
pub enum Expression {
    Apply {
        operation: Operation,
        first_arg: Box<Expression>,
        second_arg: Box<Expression>,
    },
    Integer {
        value: i64,
    },
}

#[derive(Debug)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}
