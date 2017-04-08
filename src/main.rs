pub mod parser;

#[allow(unused_must_use)]
fn tokenize() {
    let mut tokenizer = parser::tokenizer::Tokenizer::new();

    let code = "
    a = 1
    b = 'i am a string'
    ".to_string();

    println!("source: \n{}", code);

    tokenizer.tokenize(code);

    println!("\n=> {:#?}", tokenizer.get_tokens())
}

fn tree() {
    use parser::block_tree::BlockTree;

    let code = "
    outer
        inner1
        inner1
            inner2
        inner1
    outer
        inner1
    ";

    let mut tree = BlockTree::new(code, 0);
    let indents  = tree.collect_indents();

    println!("source: \n{}", code);

    let root = tree.make_tree(&indents);

    println!("\n=> {:#?}", parser::tokenizer::Tokenizer::tokenize_branch(&root))
}

#[allow(unused_must_use)]
fn parse() {
    let mut tokenizer = parser::tokenizer::Tokenizer::new();

    let code = "
    hello
    a(1, 2, 3) + 1 + (2 + 100)
    ".to_string();

    println!("source: \n{}", code);

    tokenizer.tokenize(code);

    let mut parser = parser::ast::Parser::from(tokenizer);

    println!("\n=> {:#?}", parser.parse())
}

fn main() {
    tree();
    tokenize();
    parse()
}