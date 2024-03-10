#[derive(Debug)]
pub enum SExpr {
    Expression { elems: Vec<SExpr> },
    Int(i64),
    Ident(String),
    Plus,
    Dash,
    Star,
    Slash,
    Eq,
    Dollar,
    Let,
}

