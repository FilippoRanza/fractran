use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

mod engine;
mod error_message;
mod syntax_tree;

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(grammar);
pub type SyntaxError<'a> =
    lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'a>, &'static str>;

#[derive(StructOpt)]
struct Arguments {
    file: PathBuf,
}

fn load_file(path: &Path) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut buff = String::new();
    file.read_to_string(&mut buff)?;
    Ok(buff)
}

fn parse(code: &str) -> Result<engine::Engine, SyntaxError> {
    let parser = grammar::FractranParser::new();
    let tree = parser.parse(code)?;
    Ok(engine::compile(tree))
}

fn run(mut engine: engine::Engine) {
    loop {
        println!("{}", engine.current);
        if let engine::EngineStatus::Halt = engine.step_one() {
            break;
        }
    }
}

fn main() -> std::io::Result<()> {
    let args = Arguments::from_args();
    let code = load_file(&args.file)?;
    let compile_res = parse(&code);
    match compile_res {
        Ok(engine) => run(engine),
        Err(err) => error_message::print_error(&args.file, &code, err),
    }

    Ok(())
}
