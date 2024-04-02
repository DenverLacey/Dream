use crate::ir::{Expr, Operator};
use std::iter::Peekable;

pub struct Parser<I: Iterator> {
    source: Peekable<I>,
    peeked: Option<Token>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = char>,
{
    fn more_to_parse(&mut self) -> bool {
        self.peek_token().is_some()
    }

    fn peek_token(&mut self) -> Option<&Token> {
        if self.peeked.is_some() {
            return self.peeked.as_ref();
        }

        while self.source.peek().map(|c| c.is_whitespace()) == Some(true) {
            self.source.next().expect("Already peeked");
        }

        let tok = match self.source.peek()? {
            '(' => self.tokenize_single(Token::OpenParen),
            ')' => self.tokenize_single(Token::CloseParen),
            '+' => self.tokenize_single(Token::Plus),
            '-' => self.tokenize_single(Token::Dash),
            '*' => self.tokenize_single(Token::Star),
            '/' => self.tokenize_single(Token::Slash),
            '=' => self.tokenize_single(Token::Eq),
            '$' => self.tokenize_single(Token::Dollar),
            c if c.is_ascii_digit() => self.tokenize_int(),
            _ => self.tokenize_word(),
        };

        self.peeked = Some(tok);
        self.peeked.as_ref()
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.peeked.is_none() {
            self.peek_token()?;
        }
        self.peeked.take()
    }

    fn tokenize_single(&mut self, tok: Token) -> Token {
        self.source.next().expect("No more chars left in source.");
        tok
    }

    fn tokenize_int(&mut self) -> Token {
        let mut word = String::new();
        while self.source.peek().map(|&c| c.is_ascii_digit()) == Some(true) {
            let c = self.source.next().expect("Already peeked");
            word.push(c);
        }

        let int = word.parse().expect("We guaranteed only ascii digits.");
        Token::Int(int)
    }

    fn tokenize_word(&mut self) -> Token {
        let mut word = String::new();

        while self
            .source
            .peek()
            .map(|&c| !c.is_whitespace() && c != '(' && c != ')')
            == Some(true)
        {
            let c = self.source.next().expect("Already peeked");
            word.push(c);
        }

        match word.as_str() {
            "let" => Token::Let,
            _ => Token::Ident(word),
        }
    }
}

impl<I> Parser<I>
where
    I: Iterator<Item = char>,
{
    pub fn parse(source: Peekable<I>) -> Result<Vec<Expr>, &'static str> {
        let mut p = Parser {
            source,
            peeked: None,
        };

        let mut exprs = vec![];

        while p.more_to_parse() {
            let elem = p.parse_expr(true, true)?;
            exprs.push(elem);
        }

        Ok(exprs)
    }

    fn expect(&mut self, tok: Token, err: &'static str) -> Result<Token, &'static str> {
        let ntok = self.next_token().ok_or(err)?;
        if !tok.discriminants_eq(&ntok) {
            return Err(err);
        }
        Ok(ntok)
    }

    fn eat(&mut self, tok: Token) -> bool {
        let ptok = match self.peek_token() {
            Some(ptok) => ptok,
            None => return false,
        };
        tok.discriminants_eq(&ptok)
    }

    fn parse_expr(&mut self, allow_decls: bool, parens_required: bool) -> Result<Expr, &'static str> {
        if parens_required {
            self.expect(Token::OpenParen, "Expected '(' to begin expression.")?;
        }

        let parens = self.eat(Token::OpenParen);

        let expr = match self.next_token().ok_or("Expected operator.")? {
            Token::Int(value) => Expr::Int(value),
            Token::Ident(ident) => Expr::Ident(ident),
            Token::Plus => {
                if !parens_required && !parens {
                    return Err("'+' operation requires parentheses.");
                }
                self.parse_op(Operator::Plus)?
            }
            Token::Dash => {
                if !parens_required && !parens {
                    return Err("'-' operation requires parentheses.");
                }
                self.parse_op(Operator::Dash)?
            }
            Token::Star => {
                if !parens_required && !parens {
                    return Err("'*' operation requires parentheses.");
                }
                self.parse_op(Operator::Star)?
            }
            Token::Slash => {
                if !parens_required && !parens {
                    return Err("'/' operation requires parentheses.");
                }
                self.parse_op(Operator::Slash)?
            }
            Token::Eq => {
                if !parens_required && !parens {
                    return Err("'=' operation requires parentheses.");
                }
                self.parse_op(Operator::Eq)?
            }
            Token::Dollar => {
                if !parens_required && !parens {
                    return Err("'$' operation requires parentheses.");
                }
                self.parse_op(Operator::Dollar)?
            }
            Token::Let => {
                if allow_decls {
                    let Token::Ident(ident) = self.expect(Token::Ident(String::new()), "Expected an identifier after 'let'")? else { unreachable!() };
                    let expr = self.parse_expr(false, false)?;
                    Expr::Let(ident, Box::new(expr))
                } else {
                    return Err("'let' declaration not allowed here.");
                }
            }
            Token::OpenParen => unreachable!(),
            Token::CloseParen => unreachable!(),
        };

        if parens_required {
            self.expect(Token::CloseParen, "Expected ')' to end expression")?;
        }

        Ok(expr)
    }

    fn parse_op(&mut self, op: Operator) -> Result<Expr, &'static str> {
        let mut operands = vec![];

        while self.peek_token().is_some_and(|t| !t.discriminants_eq(&Token::CloseParen)) {
            let operand = self.parse_expr(false, false)?;
            operands.push(operand);
        }

        Ok(Expr::Operation(op, operands))
    }
}

#[derive(Clone, Debug)]
enum Token {
    OpenParen,
    CloseParen,
    Plus,
    Dash,
    Star,
    Slash,
    Eq,
    Dollar,
    Let,
    Int(i64),
    Ident(String),
}

impl Token {
    fn discriminants_eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}
