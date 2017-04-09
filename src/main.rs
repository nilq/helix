extern crate docopt;
use          docopt::Docopt;

use std::io;
use std::io::prelude::*;

use std::fs::File;
use std::env;

pub mod parser;
use parser::ast::Statement;

const USAGE: &'static str = "
helix language

usage:
    helix repl
    helix translate <source>
    helix (-h | --help)
    helix --version

options:
    -h --help   display this message
    --version   display version
";

fn repl() {
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut input_line = String::new();

        match io::stdin().read_line(&mut input_line) {
            Ok(_)  => {
                if input_line.trim() == String::from(":quit") ||
                   input_line.trim() == String::from(":q") 
                {
                    println!("=> bye bb <3");

                    std::process::exit(0)
                }
                
                println!("=>\n{:#?}", parse(&input_line));
            },

            Err(e) => panic!(e),
        }
    }
}

fn parse(source: &str) -> Vec<Statement> {
    use parser::block_tree::BlockTree;

    let mut tree = BlockTree::new(source, 0);
    let indents  = tree.collect_indents();

    let root = tree.make_tree(&indents);

    let mut parser = parser::ast::Parser::from(
             parser::tokenizer::Tokenizer::from(
                    parser::tokenizer::flatten_tree(
                            &parser::tokenizer::Tokenizer::tokenize_branch(&root)
                        ),
                ),
        );

    match parser.parse() {
        Ok(c)  => c,
        Err(e) => panic!(e),
    }
}

fn translate(ast: Vec<Statement>) {
    use parser::translater::Translater;

    let transpiler = Translater::new("test".to_string());

    println!("\ntranspiled =>\n");

    for s in ast.iter() {
        if let Some(c) = transpiler.statement(s.clone()) {
            println!("{:#?}", c);
        }
    }
}

#[allow(unused_must_use)]
fn file<'a>(source: &str) -> String {
     let mut source_file = match File::open(source) {
        Ok(f)  => f,
        Err(_) => panic!("failed to open path: {}", source),
    };

    let mut source_buffer = String::new();
    source_file.read_to_string(&mut source_buffer).unwrap();

    source_buffer
}

fn main() {
    let argv: Vec<String> = env::args().collect();

    let args = Docopt::new(USAGE)
                       .and_then(|d| d.argv(argv.into_iter()).parse())
                       .unwrap_or_else(|e| e.exit());

    if args.get_bool("repl") {
        repl()
    } else if args.get_bool("translate") {
         println!("\nabstract syntax tree =>\n{:#?}", parse(&file(args.get_str("<source>"))));
         translate(parse(&file(args.get_str("<source>"))))
    }
}