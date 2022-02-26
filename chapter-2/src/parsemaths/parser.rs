// in parser.rs - providing code for the parser that builds the AST
// parser.rs uses the output of tokenizer.rs to construct the overall AST

use crate::parsemaths::ast::Node;
use crate::parsemaths::token::{OperPrec, Token};
use crate::parsemaths::tokenizer::Tokenizer;
use std::fmt;

pub(crate) struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    /* Creates a new instance of the parser, creating a Tokenizer instance and stores 1st token */
    pub fn new(expr: &'a str) -> Result<Self, ParseError> {
        // lexer is a new Tokenizer with the express passed to it
        let mut lexer = Tokenizer::new(expr);
        /*
        cur_token is result of a match whereby a successful .next() stores the value in cur_token
        if no value is returned a parse error is returned with a custom message
        */
        let cur_token = match lexer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };
        // function is expecting a Result so the result is passed into an Ok()
        Ok(Parser {
            tokenizer: lexer,
            current_token: cur_token,
        })
    }

    /*
    Generates the AST from the tokens and is the main output of parser.rs
    Invokes the generate_ast() method (priv), a recursive method that processes the AST & returns
    */
    pub fn parse(&mut self) -> Result<Node, ParseError> {
        let ast = self.generate_ast(OperPrec::DefaultZero);
        // if the match is successful it returns a node - if not, propagates the received error
        match ast {
            Ok(ast) => Ok(ast),
            Err(e) => Err(e),
        }
    }
}

/*
Parser private methods
*/
impl<'a> Parser<'a> {
    /*
    Main method to generate the AST from the tokens - recursive

    - Process numeric tokens, negative numeric tokens and expressions in parens with parse_number()
    - Parses each token from expr in a sequence within loop to check if precedence of next 2 operators
    encountered and constructs AST by calling convert_token_to_node().
    - Achieved in a way to ensure the operator with a higher precedence is executed before expr
    with a lower precedence

    e.g.
    ----
    1+2*3 -> Add(Number(1.0), Multiply(Number(2.0), Number(3.0)))
    1*2+3 -> Add(Multiply(Number(1.0), Number(2.0)), Number(3.0))
    */
    fn generate_ast(&mut self, oper_prec: OperPrec) -> Result<Node, ParseError> {
        let mut left_expr = self.parse_number()?;
        while oper_prec < self.current_token.get_oper_prec() {
            // recursion base case
            if self.current_token == Token::EoF {
                break;
            }
            // declares variable as output of converting left_expr token to a node
            let right_expr = self.convert_token_to_node(left_expr.clone())?;
            // shifts the value in right_expr to left_expr to continue recursion
            left_expr = right_expr;
        }
        Ok(left_expr)
    }

    /*
    Retrieves the number tokens
    Takes current token and checks 3 things:
    1 - Is token of form Num(i)
    2 - Does token have a sign e.g. -1+2 -> Add(Negative(Number(1)), Number(2))
    3 - Pairs of parentheses - if an expression is found within paren it treats this as a multiply
    */
    fn parse_number(&mut self) -> Result<Node, ParseError> {
        let token = self.current_token.clone();
        match token {
            Token::Subtract => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            }
            Token::Num(i) => {
                self.get_next_token()?;
                Ok(Node::Number(i))
            }
            Token::LeftParen => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::DefaultZero)?;
                self.check_paren(Token::RightParen)?;
                if self.current_token == Token::LeftParen {
                    let right = self.generate_ast(OperPrec::MulDiv)?;
                    return Ok(Node::Multiply(Box::new(expr), Box::new(right)));
                }
                Ok(expr)
            }
            _ => Err(ParseError::UnableToParse("Unable to parse".to_string())),
        }
    }

    /* Parses operators and converts to AST*/
    fn convert_token_to_node(&mut self, left_expr: Node) -> Result<Node, ParseError> {
        match self.current_token {
            Token::Add => {
                self.get_next_token()?;
                // Access right side of the expression
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Subtract => {
                self.get_next_token()?;
                // Access right side of the expression
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Subtract(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Multiply => {
                self.get_next_token()?;
                // Access right side of the expression
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Multiply(Box::new(left_expr), Box::new(right_expr)))
            }

            Token::Divide => {
                self.get_next_token()?;
                // Access right side of the expression
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Divide(Box::new(left_expr), Box::new(right_expr)))
            }

            Token::Caret => {
                self.get_next_token()?;
                // Access right side of the expression
                let right_expr = self.generate_ast(OperPrec::Power)?;
                Ok(Node::Caret(Box::new(left_expr), Box::new(right_expr)))
            }
            _ => Err(ParseError::InvalidOperator(format!(
                "Please enter a valid operator {:?}",
                self.current_token
            ))),
        }
    }

    /*
    Helper Method
    Checks for matching parentheses in expression
    */
    fn check_paren(&mut self, expect: Token) -> Result<(), ParseError> {
        if expect == self.current_token {
            self.get_next_token()?;
            Ok(())
        } else {
            Err(ParseError::InvalidOperator(format!(
                "Expected {:?}\tGot {:?}",
                expect, self.current_token
            )))
        }
    }

    /*
    Retrieves the next Token from arithmetic expression using the Tokenizer struct
    Sets the current_token field of Parser struct
    If unsuccessful the method returns a ParseError
     */
    fn get_next_token(&mut self) -> Result<(), ParseError> {
        // match used to also catch errors that may appear
        let next_token = match self.tokenizer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };
        // simply assigns the value of Parser.current_token to the result of success match arm
        self.current_token = next_token;
        // Empty tuple in Ok(()) - if no error occurs no concrete value returns
        Ok(())
    }
}

/*
Defining custom error types as an enum
2 options both returning a String to the user of the application
Debug & Display are needed to print the errors
*/
#[derive(Debug)]
pub enum ParseError {
    UnableToParse(String),
    InvalidOperator(String),
}

/*
Implements Display for the custom type otherwise it would not be possible to print the errors
*/
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            self::ParseError::UnableToParse(e) => write!(f, "Error in evaluating {}", e),
            self::ParseError::InvalidOperator(e) => write!(f, "Error in evaluating {}", e),
        }
    }
}

/*
Implements conversion from Box<dyn Error> into the custom ParseError error
*/
impl std::convert::From<std::boxed::Box<dyn std::error::Error>> for ParseError {
    fn from(_evalerr: std::boxed::Box<dyn std::error::Error>) -> Self {
        ParseError::UnableToParse("Unable to parse".into())
    }
}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsemaths::ast::Node::{Add, Number};

    #[test]
    fn test_addition() {
        let mut parser = Parser::new("1+2").unwrap();
        let expected = Add(Box::new(Number(1.0)), Box::new(Number(2.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
}
