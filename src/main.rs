mod lexer;
mod token;

fn main() {
    println!("Welcome to Rusty!");

    let input = String::from("let x = (2 + 3) * (5 + 2/3)\n def myfunction(x, y)");
    let mut lexer = lexer::new(input);
    let tokens = lexer.parse();
    for token in tokens {
        println!("{:?}", token);
    }
}
