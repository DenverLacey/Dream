mod ir;
mod parser;

fn main() {
    let path = match std::env::args().skip(1).next() {
        None => {
            println!("ERROR: No source file given.");
            return;
        }
        Some(path) => path,
    };

    let source = match std::fs::read_to_string(path) {
        Ok(source) => source,
        Err(err) => {
            println!("ERROR: Couldn't read source file: {err}");
            return;
        }
    };

    let source = source.chars().peekable();
    let exprs = match parser::Parser::parse(source) {
        Ok(exprs) => exprs,
        Err(err) => {
            println!("ERROR: {err}");
            return;
        }
    };

    println!("expr = {exprs:?}");
}

