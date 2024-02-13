use crate::Token;

use self::nodes::Program;
use super::error::Error;

pub mod nodes;
pub mod data_types;

/// The `Parser` trait defines the interface for parsing source code and generating an abstract syntax tree (AST).
pub trait Parser {
    /// Parses a sequence of tokens, generating an abstract syntax tree (AST).
    fn parse(tokens: Vec<Token>, path: &'static str) -> Result<Program<'_>, Box<dyn ParseError>>;
}

pub trait ParseError: Error {}
