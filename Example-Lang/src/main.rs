use std::{fs::File, io::{BufReader, Read}};

mod ir;
mod parser;
mod codegen;

fn main() {
    let Some(path) = std::env::args().skip(1).next() else {
        println!("ERROR: No source file given.");
        return;
    };

    let source = match std::fs::read_to_string(&path) {
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

    let source_name = if let Some(idx) = path.rfind('.') {
        &path[..idx]
    } else {
        &path
    };
    let out_path = format!("{source_name}.dream");
    let dasm_path = format!("{source_name}.dasm");

    let mut out_file = match File::create(&out_path) {
        Ok(file) => file,
        Err(err) => {
            println!("ERROR: {err}");
            return;
        }
    };
    codegen::compile(&mut out_file, &exprs);

    let dream_file = BufReader::new(File::open(out_path).unwrap());
    let mut dasm_file = File::create(dasm_path).unwrap();

    // NOTE: We're skipping 56 to ignore the dream files breamble.
    match morpheus::disasemble(dream_file.bytes().skip(56).map(Result::unwrap), &mut dasm_file) {
        Ok(_) => {},
        Err(err) => {
            println!("ERROR: {err:?}");
            return;
        }
    }
}

