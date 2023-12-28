use crate::prelude::error::Error;

/// Derived from the Error trait, it's the Error used for the Lexer Error in the compiler.
pub trait LexerError: Error {}