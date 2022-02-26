use std::io;

mod parsemaths;
use parsemaths::ast;
use parsemaths::parser::{ParseError, Parser};

fn evaluate(expr: String) -> Result<f64, ParseError> {
    // remove whitespace characters and collect into a collection of type String
    let expr = expr.split_whitespace().collect::<String>();
    let mut maths_parser = Parser::new(&expr)?;
    let ast = maths_parser.parse()?;
    println!("The generated AST is {:?}", ast);

    Ok(ast::eval(ast)?)
}

fn main() {
    println!("Hello! Welcome to Arithmetic Expression Evaluator!");
    println!("You can calculate the value of expressions such as: 2*3+4(4-5)+2^3/4. ");
    println!("Allowed numbers: positive, negative and decimals. ");
    println!("Supported operations: Add, Subtract, Multiply, Divide, PowerOf(^). ");
    println!("Enter your arithmetic expression below:");
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.contains('q') {
                    println!("Thanks for using the Arithmetic Expression Evaluator!");
                    break;
                }
                match evaluate(input) {
                    Ok(val) => println!("The computed number is {}\n", val),
                    Err(_) => {
                        println!("Error in evaluating expression. Please enter valid expression\n");
                    }
                };
            }
            Err(error) => println!("ERROR: {}", error),
        }
    }
}
