// in token.rs - providing code for the token data structures

/*
Enum to define the precedence of the operators accepted by the parser
Order is from Lowest to Highest
*/

/* enum chosen as it can store multiple data types from a set of predefined variables
data structure for the OUTPUT */
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Add,        // '+'
    Subtract,   // '-'
    Multiply,   // '*'
    Divide,     // '/'
    Caret,      // '^'
    LeftParen,  // '('
    RightParen, // ')'
    Num(f64),   // '1.0'
    EoF,        // ''
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum OperPrec {
    DefaultZero, // default -> lowest precedence
    AddSub,      // applied if operation is add/sub
    MulDiv,      // applied if operation is mul/div
    Power,       // applied if operation is caret
    Negative,    // applied if operation is negative number
}

impl Token {
    pub fn get_oper_prec(&self) -> OperPrec {
        use self::OperPrec::*;
        use self::Token::*;

        match *self {
            Add | Subtract => AddSub,
            Multiply | Divide => MulDiv,
            Caret => Power,
            _ => DefaultZero,
        }
    }
}
