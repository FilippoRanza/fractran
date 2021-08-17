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
#[structopt(about = "FRACTRAN interpreter.")]
struct Arguments {
    #[structopt(help = "FRACTRAN source file.")]
    file: PathBuf,
    #[structopt(
        short = "-d",
        long = "--debug",
        help = "Enable debug mode: show  current step index and current value at each step on stdout.",
        parse(from_flag)
    )]
    debug: bool,
    #[structopt(
        short = "-l",
        long = "--limit",
        help = "Set the max number of steps to run. By default run until halt condition is met."
    )]
    limit: Option<usize>,
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

fn run(engine: engine::Engine, debug: bool, limit: Option<usize>) {
    let out = if debug {
        engine::debug_program(engine, limit)
    } else {
        engine::run_program(engine, limit)
    };
    println!("{}", out);
}

fn main() -> std::io::Result<()> {
    let args = Arguments::from_args();
    let code = load_file(&args.file)?;
    let compile_res = parse(&code);
    match compile_res {
        Ok(engine) => run(engine, args.debug, args.limit),
        Err(err) => error_message::print_error(&args.file, &code, err),
    }

    Ok(())
}
