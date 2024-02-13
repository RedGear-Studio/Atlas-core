/// Contains all the Tokens needed for a basic Lexer
pub mod token;

use crate::Token;
use super::error::Error;

/// The `Lexer` trait defines the interface for lexical analysis.
pub trait Lexer<'lex> {
    /// Tokenizes the source code, converting it into a sequence of tokens.
    fn tokenize(path: &'static str, contents: &'lex str) -> Result<Vec<Token>, Box<dyn LexerError>>;
}

pub trait LexerError: Error {}
