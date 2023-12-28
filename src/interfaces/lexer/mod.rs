/// Contains all the Tokens needed for a basic Lexer
pub mod token;
/// Contains all the potential Lexer Error
pub mod lexer_errors;

use crate::{Token, lexer_errors::LexerError};

/// The `Lexer` trait defines the interface for lexical analysis.
pub trait Lexer {
    /// Tokenizes the source code, converting it into a sequence of tokens.
    ///
    ///
    /// A `Vec` of `WithSpan` objects, where each `WithSpan` contains a `TokenKind::TokenKind` along with its associated
    /// span in the source code.
    fn tokenize(&mut self) -> Result<Vec<Token>, Box<dyn LexerError>>;
}
