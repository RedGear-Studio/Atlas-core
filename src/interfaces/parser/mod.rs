use crate::Token;

use self::{parser_errors::ParseError, nodes::Program};

/// Contains all the potential Parser errors
pub mod parser_errors;

pub mod nodes;
pub mod data_types;

/// The `Parser` trait defines the interface for parsing source code and generating an abstract syntax tree (AST).
pub trait Parser {
    /// Parses a sequence of tokens, generating an abstract syntax tree (AST).
    fn parse(&mut self, tokens: Vec<Token>) -> Result<Program, Box<dyn ParseError>>;
}