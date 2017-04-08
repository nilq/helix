pub mod lexer;

#[allow(unused_must_use)]
fn tokenize() {
    let mut tokenizer = lexer::Tokenizer::new();

    let code = "
    a := 1
    b := 'i am a string'
    ".to_string();

    println!("source: \n{}", code);

    tokenizer.tokenize(code);

    println!("\n=>{:#?}", tokenizer.get_tokens())
}

fn tree() {
    use lexer::block_tree;
    use lexer::block_tree::BlockTree;

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
    println!("\n=>{:#?}", tree.make_tree(&indents))
}

fn main() {
    tree();
    tokenize()
}