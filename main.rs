use std::error::Error;
use std::io;
use std::fs::File;
use std::io::Read;
use std::iter::Iterator;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]

pub enum Token {
    // Identifiers and numbers are stored as strings and floats.
    Identifier(String),
    NumberLiteral(f64),

    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    Equal,

    SemiColon,
    LParen,
    RParen,

    Let,
}

struct Lexer<'a> {
    // Using Rust's internal Peekable iterator 
    input_iter: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a String) -> Lexer<'a> {
        Lexer { input_iter: input.chars().peekable() }
    }

    fn read_char(&mut self) -> Option<char> {
        // Advances the iterator and returns the next value
        self.input_iter.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        // Returns a reference to the next() value without advancing the iterator
        self.input_iter.peek()
    }

    fn is_letter(c: char) -> bool {
        // Check whether a letter is an actual letter or '_' or a digit
        c.is_alphabetic() || c == '_'
    }

    fn lookup_keyword(key: String) -> Token {
        // Match identifiers and keyword to appropriate token
        match key.as_str() {
            "let" => Token::Let,
            _ => Token::Identifier(key),
        }
    }

    fn skip_whitespace(&mut self) {
        // Skip all whitespace characters
        while let Some(&c) = self.peek_char() {
            // Return reference to the next() value and evaluate
            if c.is_whitespace() {
                // If c is a whitespace characters, continue reading
                let _ = self.read_char();
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self, c: char) -> String {
        // Create a new string to contain all possible characters
        let mut identifiers = String::new();
        identifiers.push(c);
        while let Some(&c) = self.peek_char() {
            // Return reference to the next() value and evaluate
            if c.is_alphanumeric() || c == '_' {
                // Check whether c is a letter or a digit or '_'
                identifiers.push(self.read_char().unwrap());
            } else {
                // None of the above, break
                break;
            }
            }
        return identifiers
    }

    fn read_number(&mut self, c: char) -> f64 {
        // Create a new string to contain all possible characters
        let mut number = String::new();
        number.push(c);
        while let Some(&c) = self.peek_char() {
            // Return a reference to the next element and evaluate
            if c.is_digit(10) || c == '.' {
                // If c is a digit or '.', add to string
                number.push(self.read_char().unwrap());
            } else {
                // None of the above, break
                break;
            }
        }
        // Convert the string to appropriate type
        return number.parse::<f64>().unwrap()
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if let Some(c) = self.read_char() {
            match c {
                '=' => Some(Token::Equal),
                '+' => Some(Token::Plus),
                '-' => Some(Token::Minus),
                '*' => Some(Token::Star),
                '/' => Some(Token::Slash),
                '^' => Some(Token::Caret),
                ';' => Some(Token::SemiColon),
                '(' => Some(Token::LParen),
                ')' => Some(Token::RParen),
                _ => {
                    if Self::is_letter(c) {
                        // Process c in read_identifier() --> process results in lookup_keyword() to find appropriate token
                        Some(Self::lookup_keyword(self.read_identifier(c)))
                    } else if c.is_digit(10) {
                        Some(Token::NumberLiteral(self.read_number(c)))
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get file name from user
    println!("Please enter a filename.");

    let mut filename = String::new();
    io::stdin().read_line(&mut filename)?;

    let mut file = File::open(filename.trim())?;

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    // Loop through tokens
    let mut lexer = Lexer::new(&file_contents);

    loop {
        match lexer.next_token() {
            Some(token_type) => println!("{:?}", token_type),
            None => break,
        }
    }
    Ok(())
}
