#[derive(Debug)]
pub enum UnaryOperator {
    Minus,
    Not,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Minus,
    Plus,
    Slash,
    Star,
}

#[derive(Debug)]
pub enum LogicOperator {
    And,
    Or,
}

pub enum Literal {
    Bool(bool),
    None,
    Number(f64),
    Str(String),
}
