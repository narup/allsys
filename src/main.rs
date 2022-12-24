mod lexer;
mod token;

fn main() {
    println!("Welcome to Rusty!");

    let input = String::from("let x = (20.50 * 35 + 3) * (5 + 2/3)\n def my_function(x, y) { x = 2 print(\"hello world\") } \n");
    let mut lexer = lexer::new(input);
    let tokens = lexer.parse();
    for token in tokens {
        println!("{:?}", token);
    }
}
