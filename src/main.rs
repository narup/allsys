mod lexer;
mod token;

fn main() {
    println!("Welcome to Rusty!");

    let input = String::from("x == 30");
    let mut lexer = lexer::new(input);
    let tokens = lexer.parse();
    for token in tokens {
        println!("{:?}", token);
    }
}
