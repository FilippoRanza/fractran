use std::io::Read;
use std::fs::File;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

mod syntax_tree;
mod engine;

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(grammar);

#[derive(StructOpt)]
struct Arguments {
    file: PathBuf
}  


fn load_file(path: &Path) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut buff = String::new();
    file.read_to_string(&mut buff)?;
    Ok(buff)
}


fn parse(code: String) -> engine::Engine {
    let parser = grammar::FractranParser::new();
    let tree = parser.parse(&code).unwrap();
    engine::compile(tree)
}  


fn main() -> std::io::Result<()> {

    let args = Arguments::from_args();
    let code = load_file(&args.file)?;
    let mut engine = parse(code);

    loop {
        println!("{}", engine.current);
        if let engine::EngineStatus::Halt = engine.step_one() {
            break;
        }
    }

    Ok(())
}
