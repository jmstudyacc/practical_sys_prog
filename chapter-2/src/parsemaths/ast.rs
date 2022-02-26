// in ast.rs - providing code for the AST

use std::error;

/*
List of permitted AST node types that can be evaluated
Can be arithmetic operators or numbers
*/
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Caret(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Number(f64),
}

/*
Code to evaluate the information in the node, introducing the basic rules of maths logic
Function returns Box<dyn error:Error> to tell the compiler we don't know what will return
Whatever returns must implement the Error trait - dyn error Error = trait object
The use of dyn indicates a trait object (Dynamic Dispatch)
*/
pub fn eval(expr: Node) -> Result<f64, Box<dyn error::Error>> {
    use self::Node::*;
    match expr {
        Number(i) => Ok(i),
        Add(expr1, expr2) => Ok(eval(*expr1)? + eval(*expr2)?),
        Subtract(expr1, expr2) => Ok(eval(*expr1)? - eval(*expr2)?),
        Multiply(expr1, expr2) => Ok(eval(*expr1)? * eval(*expr2)?),
        Divide(expr1, expr2) => Ok(eval(*expr1)? / eval(*expr2)?),
        Negative(expr1) => Ok(-(eval(*expr1)?)),
        Caret(expr1, expr2) => Ok(eval(*expr1)?.powf(eval(*expr2)?)),
    }
}

// Unit tests
mod tests {
    use super::*;
    #[test]
    fn test_expr1() {
        use crate::parsemaths::parser::Parser;

        let ast = Parser::new("1+2-3").unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 0.0);
    }
    #[test]
    fn test_expr2() {
        use crate::parsemaths::parser::Parser;

        let ast = Parser::new("3+2-1*5/4").unwrap().parse().unwrap();
        let val = eval(ast).unwrap();
        assert_eq!(val, 3.75);
    }
}
