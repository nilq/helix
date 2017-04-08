pub mod lexer;

fn main() {
    let mut tokenizer = lexer::Tokenizer::new();

    let code = "
    a := 1
    b := 'i am a string'
    ".to_string();

    println!("source: \n {}", code);

    tokenizer.tokenize(code);

    println!("\n=> {:#?}", tokenizer.get_tokens())
}