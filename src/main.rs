use std::io;
use std::io::Write;

mod lexer;
mod token;

fn main() {
    println!("Welcome to Rusty!");
    println!("Type Ctrl+C to exit the shell");
    loop {
        print!("rty>");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Error reading the input");
        
        if input.len() == 0 || input.len() == 1 {
            continue;
        }

        let mut lexer = lexer::new(input);
        let tokens = lexer.parse();

        println!("Tokens for your input:");
        for token in tokens {
            println!("{:?}", token);
        }
    }
}
