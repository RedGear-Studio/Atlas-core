use std::fmt::{Display, Debug};

use crate::utils::span::Spanned;

/// Base error trait from which all errors of the compiler extend.
pub trait Error: Display + Spanned + Debug {
    /// Whether the error is recoverable or not.
    fn recoverable(&self) -> bool;
    /// The message of the error.
    fn message(&self) -> String;
    /// The help message of the error.
    fn help(&self) -> Option<String>;
    /// The code of the error.
    fn code(&self) -> u64;
}