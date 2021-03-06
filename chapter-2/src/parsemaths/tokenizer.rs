// in tokenizer.rs - providing code for the tokenizer functionality

use crate::parsemaths::token::Token;
use std::iter::Peekable;
use std::str::Chars;

/* structs can hold references, but explicit lifetimes required when used
=> any reference to the Tokenizer struct cannot outlive the reference to the contained chars
data structure for the INPUT */
pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>,
}

/*
   impl<'a>: declares the lifetime
   Tokenizer<'a>: uses the lifetime
*/
impl<'a> Tokenizer<'a> {
    // new_expr is a reference to a string with a lifetime matching the Tokenizer struct
    pub fn new(new_expr: &'a str) -> Self {
        Tokenizer {
            /*
            new_expr: string slice
            new_expr.chars(): iterator over string slice
            new_expr.chars().peekable(): peekable iterator over string slice
            */
            expr: new_expr.chars().peekable(),
        }
    }
}

// Implements Iterator trait on the Tokenizer struct
// Enables the use of the .peek() method to build the logic for the Tokenizer
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        // reads the next character in the store arithmetic expression in the Tokenizer struct field
        let next_char = self.expr.next();
        // next character is evaluated via a match statement - pattern matching to return the token
        match next_char {
            // if a value inclusively between 0 -> 9
            Some('0'..='9') => {
                // mutable String variable assigned to next value in expression
                let mut number = next_char?.to_string();
                // while let is a loop checking an if condition
                // if value of next_char is next char when peeking on string slice
                while let Some(next_char) = self.expr.peek() {
                    // if numeric or decimal, push the next value onto number
                    if next_char.is_numeric() || next_char == &'.' {
                        // consume the next character once known to be numeric value
                        number.push(self.expr.next()?);
                    } else if next_char == &'(' {
                        return None;
                    } else {
                        break;
                    }
                }
                Some(Token::Num(number.parse::<f64>().unwrap()))
            }
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Subtract),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            None => Some(Token::EoF),
            _ => None,
        }
    }
}

// Unit Tests
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_positive_integer() {
        let mut tokenizer = Tokenizer::new("34");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(34.0))
    }

    #[test]
    fn test_decimal_number() {
        let mut tokenizer = Tokenizer::new("34.5");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(34.5))
    }

    #[test]
    #[ignore]
    fn test_invalid_input() {
        let mut tokenizer = Tokenizer::new("#$#");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(34.5))
    }
}
