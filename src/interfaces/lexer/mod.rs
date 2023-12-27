/// Contains all the Tokens needed for a basic Lexer
pub mod token;
/// Contains all the potential Lexer Error
pub mod lex_errors;

use std::path::PathBuf;

use crate::{TokenKind, Token};

/// The `Lexer` trait defines the interface for lexical analysis.
pub trait Lexer {
    /// Tokenizes the source code, converting it into a sequence of tokens.
    ///
    ///
    /// A `Vec` of `WithSpan` objects, where each `WithSpan` contains a `TokenKind::TokenKind` along with its associated
    /// span in the source code.
    fn tokenize(&mut self, path: &'static str) -> Vec<Token>;
}
