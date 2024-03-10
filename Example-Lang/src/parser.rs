use crate::ir::SExpr::{self, *};
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
    pub fn parse(source: Peekable<I>) -> Result<SExpr, &'static str> {
        let mut p = Parser {
            source,
            peeked: None,
        };

        let mut exprs = vec![];

        while p.more_to_parse() {
            let elem = p.parse_expr(true)?;
            exprs.push(elem);
        }

        Ok(Expression { elems: exprs })
    }

    fn expect(&mut self, tok: Token, err: &'static str) -> Result<Token, &'static str> {
        let ntok = self.next_token().ok_or(err)?;
        if !tok.discriminants_eq(&ntok) {
            return Err(err);
        }
        Ok(ntok)
    }

    fn parse_expr(&mut self, allow_decls: bool) -> Result<SExpr, &'static str> {
        if self
            .peek_token()
            .is_some_and(|t| !t.discriminants_eq(&Token::OpenParen))
        {
            let tok = self.next_token().ok_or("Unterminated statement.")?;
            let elem = match tok {
                Token::Int(value) => Int(value),
                Token::Ident(ident) => Ident(ident),
                _ => return Err("Unparenthesized complex expression."),
            };
            Ok(elem)
        } else {
            self.next_token().expect("Already peeked");

            let mut stmt = vec![];

            loop {
                let elem = if self.peek_token().is_some_and(|t|t.discriminants_eq(&Token::OpenParen)) {
                    self.parse_expr(false)?
                } else {
                    let tok = self.next_token().ok_or("Unterminated statement.")?;
                    match tok {
                        Token::OpenParen => {
                            let expr = self.parse_expr(false)?;
                            expr
                        }
                        Token::CloseParen => break,
                        Token::Plus => Plus,
                        Token::Dash => Dash,
                        Token::Star => Star,
                        Token::Slash => Slash,
                        Token::Eq => Eq,
                        Token::Dollar => Dollar,
                        Token::Let => {
                            // TODO: Move this so it has to be the first and only thing in the Expression
                            if allow_decls {
                                stmt.push(Let);
                                let Token::Ident(ident) = self.expect(Token::Ident(String::new()), "Expected an identifier after 'let'")? else { unreachable!() };
                                stmt.push(Ident(ident));
                                let expr = self.parse_expr(false)?;
                                stmt.push(expr);
                                self.expect(
                                    Token::CloseParen,
                                    "Expected ')' to end 'let' expression.",
                                )?;
                                break;
                            } else {
                                return Err("'let' expressions not allowed here.");
                            }
                        }
                        Token::Int(value) => Int(value),
                        Token::Ident(ident) => Ident(ident),
                    }
                };
                stmt.push(elem);
            }

            Ok(Expression { elems: stmt })
        }
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
