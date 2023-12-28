use crate::prelude::error::Error;

/// Derived from the Error trait, it's the Error used for the Parsing Error in the compiler.
pub trait ParseError: Error {
}