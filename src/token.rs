use super::number::Number;

pub enum Token {
    Ident(String),
    Number(Number),
    LeftParen,
    RightParen,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Assign,
}
