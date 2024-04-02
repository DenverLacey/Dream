#[derive(Debug)]
pub enum Expr {
    Int(i64),
    Ident(String),
    Operation(Operator, Vec<Expr>),
    Let(String, Box<Expr>),
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Dash,
    Star,
    Slash,
    Eq,
    Dollar,
}

