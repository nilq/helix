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
    use parser::block_tree::BlockTree;

    let code = "
if false
    b? = false
    b  = true

    if a == false
        more_nested = \"hello i am string\"
else
    a = 'no success without succ'
    ";

    println!("source: \n{}", code);

    let mut tree = BlockTree::new(code, 0);
    let indents  = tree.collect_indents();

    let root = tree.make_tree(&indents);

    println!("root: => {:#?}", parser::tokenizer::Tokenizer::tokenize_branch(&root));

    let mut parser = parser::ast::Parser::from(
             parser::tokenizer::Tokenizer::from(
                    parser::tokenizer::flatten_tree(
                            &parser::tokenizer::Tokenizer::tokenize_branch(&root)
                        ),
                ),
        );

    println!(
            "\n=> {:#?}",
            match parser.parse() {
                Ok(c)  => c,
                Err(e) => panic!(e),
            },
        )
}

fn main() {
    //tree();
    //tokenize();
    parse()
}