mod lexer;

fn main() {
    println!("Welcome to Rusty!");
    lexer::print_tokens();

    let l = lexer::new("+()*-".to_string());
    println!("Final output: {}", l.next_token())
}
