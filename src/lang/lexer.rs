/**
Represents a token which can be used by the parser
# Examples
```
"1" -> Token::Number(1)
"+" -> Token::Plus()
"1+2" -> [Token::Number(1), Token::Plus(), Token::Number(2)]
```
 */
#[derive(Eq, PartialEq, Debug)]
pub enum Token {
    /** ( */
    LParen(),
    /** ) */
    RParen(),
    /** \+ */
    Plus(),
    /** \- */
    Minus(),
    /** \* */
    Times(),
    /** / */
    Divide(),

    /** 0 - 9 */
    Number(i8),
    /** . */
    Point(),

    /** \/\/ */
    Comment(),
    /** Empty line */
    None(),
    /** Unknown token */
    NaN(),
    /** Whitespace  */
    Whitespace(),
}

/**
Represents a lexer that can scan lines and create a stream of tokens
 */
pub struct Lexer {
    lines: Vec<String>,
    line: i8,
    count: i8,
    token: i8,
}

/**
Default implementation of the lexer

# Example
```
let mut lexer: Lexer = Lexer::new(vec![String::from("1+2")]);
let result: Vec<Token> = lexer.next_line();
println!("{:?}"); // [Token::Number(1), Token::Plus(), Token::Number(2)]
```
 */
impl Lexer {
    pub fn new(lines: Vec<String>) -> Self {
        return Lexer {
            lines,
            line: 0,
            count: 0,
            token: 0,
        };
    }

    pub fn next_line(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        self.line += 1;

        let mut line = String::new();
        let mut count = 0;
        for entry in self.lines.as_slice() {
            if count == self.line - 1 {
                line = entry.clone();
                break;
            }
            count += 1;
        }

        if line.is_empty() {
            return vec![Token::None()];
        }

        if line.starts_with("//") {
            return vec![Token::Comment()];
        }

        for c in line.chars() {
            match c {
                '(' => tokens.push(Token::LParen()),
                ')' => tokens.push(Token::RParen()),
                '+' => tokens.push(Token::Plus()),
                '-' => tokens.push(Token::Minus()),
                '*' => tokens.push(Token::Times()),
                '/' => tokens.push(Token::Divide()),

                '0' => tokens.push(Token::Number(0)),
                '1' => tokens.push(Token::Number(1)),
                '2' => tokens.push(Token::Number(2)),
                '3' => tokens.push(Token::Number(3)),
                '4' => tokens.push(Token::Number(4)),
                '5' => tokens.push(Token::Number(5)),
                '6' => tokens.push(Token::Number(6)),
                '7' => tokens.push(Token::Number(7)),
                '8' => tokens.push(Token::Number(8)),
                '9' => tokens.push(Token::Number(9)),

                '.' => tokens.push(Token::Point()),
                ' ' => tokens.push(Token::Whitespace()),
                _ => tokens.push(Token::NaN())
            }
        }

        return tokens;
    }
}

#[cfg(test)]
pub mod tests {
    use crate::lang::lexer::{Lexer, Token};

    #[test]
    fn test_lexer_with_comment() {
        let mut lexer = Lexer::new(vec![String::from("// 1")]);
        let mut result = vec![Token::Comment()];

        assert_eq!(lexer.next_line(), result);
        result.push(Token::Number(1));
        assert_ne!(lexer.next_line(), result);
    }

    #[test]
    fn test_basic_lexer() {
        let mut lexer = Lexer::new(vec![String::from("(+-) */.")]);
        let result = vec![Token::LParen(), Token::Plus(), Token::Minus(),
                          Token::RParen(), Token::Whitespace(), Token::Times(), Token::Divide(), Token::Point()];

        assert_eq!(lexer.next_line(), result);
    }

    #[test]
    fn test_lexer_with_number() {
        let mut lexer = Lexer::new(vec![String::from("1*(-2.3+2)")]);
        let result = vec![Token::Number(1), Token::Times(), Token::LParen(),
                          Token::Minus(), Token::Number(2), Token::Point(), Token::Number(3),
                          Token::Plus(), Token::Number(2), Token::RParen()];

        assert_eq!(lexer.next_line(), result);
    }
}
