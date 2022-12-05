use automata::*;
use regex::Regex;
use std::path::Path;
use symbol_table::{SymbolIdx, SymbolTable};

pub mod automata;
pub mod symbol_table;

#[derive(Debug, Clone)]
struct Tokenized(Vec<Lexeme>);

impl std::fmt::Display for Tokenized {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for lexeme in self.0.iter() {
            match lexeme {
                Lexeme::Identifier(idx) => writeln!(f, "Identifier: {:?}", idx)?,
                Lexeme::Constant(idx) => writeln!(f, "Constant: {:?}", idx)?,
                Lexeme::Token(token) => writeln!(f, "Token: {}", token)?,
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Lexer<'a> {
    tokens: Vec<String>,
    pub symbol_table: SymbolTable<'a>,
}

#[derive(Debug, Clone)]
enum Lexeme {
    Identifier(SymbolIdx),
    Constant(SymbolIdx),
    Token(String),
}

#[derive(Debug, Clone)]
struct LexerError {
    line: usize,
    pos: usize,
    rest_of_line: String,
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Can't parse token on line {} at position {}. Rest of line: {}",
            self.line, self.pos, self.rest_of_line
        )
    }
}

impl<'a> Lexer<'a> {
    fn new(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let file = std::fs::read_to_string(path)?;
        let mut tokens: Vec<String> = file.lines().map(String::from).collect();
        tokens.sort_by_key(|v| v.len());
        tokens.reverse();
        Ok(Lexer {
            tokens,
            symbol_table: SymbolTable::new(100),
        })
    }

    fn tokenize(&mut self, input: &'a str) -> Result<Tokenized, LexerError> {
        let mut tokenized = Vec::new();
        for (line_nr, mut line) in input.lines().enumerate() {
            line = line.trim_end();
            let line_len = line.len();

            while line.len() != 0 {
                match self.parse(&mut line) {
                    Some(lexeme) => {
                        tokenized.push(lexeme);
                    }
                    None => {
                        return Err(LexerError {
                            line: line_nr,
                            pos: line_len - line.len(),
                            rest_of_line: line.to_string(),
                        })
                    }
                }
            }
        }
        Ok(Tokenized(tokenized))
    }

    fn parse(&mut self, input: &mut &'a str) -> Option<Lexeme> {
        *input = input.trim_start();
        self.parse_token(input)
            .or_else(|| self.parse_constant_fsm(input))
            .or_else(|| self.parse_identifier_fsm(input))
    }

    fn parse_token(&self, input: &mut &str) -> Option<Lexeme> {
        for token in self.tokens.iter() {
            if token.len() <= input.len() && *token == input[..token.len()] {
                *input = &input[token.len()..];
                return Some(Lexeme::Token(token.clone()));
            }
        }
        None
    }

    fn parse_identifier(&mut self, input: &mut &'a str) -> Option<Lexeme> {
        let re = Regex::new(r"^[a-zA-Z_$][a-zA-Z_$0-9]*").unwrap();
        let mat = re.find(input)?;

        let sy_idx = self.symbol_table.add(&input[..mat.end()]);
        *input = &input[mat.end()..];

        Some(Lexeme::Identifier(sy_idx))
    }

    fn parse_constant(&mut self, input: &mut &'a str) -> Option<Lexeme> {
        let re = Regex::new(r"^[0-9][0-9]*").unwrap();
        let mat = re.find(input)?;
        let sy_idx = self.symbol_table.add(&input[..(mat.end() - 1)]);

        *input = &input[mat.end()..];
        Some(Lexeme::Constant(sy_idx))
    }

    fn parse_identifier_fsm(&mut self, input: &mut &'a str) -> Option<Lexeme> {
        let mut fa = identifier::StateMachine::new();

        for c in input.chars() {
            fa.transition(c);
            if !fa.valid() {
                break;
            }
        }

        let mat = fa.accepted();

        if mat.is_empty() {
            return None;
        }

        let sy_idx = self.symbol_table.add(&input[..mat.len()]);
        *input = &input[mat.len()..];

        Some(Lexeme::Constant(sy_idx))
    }

    fn parse_constant_fsm(&mut self, input: &mut &'a str) -> Option<Lexeme> {
        let mut fa = constant::StateMachine::new();

        for c in input.chars() {
            fa.transition(c);
            if !fa.valid() {
                break;
            }
        }

        let mat = fa.accepted();

        if mat.is_empty() {
            return None;
        }

        let sy_idx = self.symbol_table.add(&input[..mat.len()]);
        *input = &input[mat.len()..];

        Some(Lexeme::Constant(sy_idx))
    }
}

fn main() {
    let token_file = "Tokens.in";
    let input_program = "p2.txt";

    let mut lexer = Lexer::new(token_file).expect("Cannot find tokens");

    let program = std::fs::read_to_string(input_program).expect("Cannot find input program");

    let tokenized = lexer.tokenize(&program);

    println!("{}", tokenized.unwrap());
    println!("{}", lexer.symbol_table);
}
