/*
 * atlas-core by Gipson62
 *
 * The core of a tool designed to help you make programming language
*/

//#![warn(missing_docs)]

//! # atlas-Core
//!
//! `atlas-core` is the foundational library for a language creation tool designed to assist users in developing languages.
//! This core library currently serves as the building block for the creation of Atlas77, a functional programming language.

/// TODO
//Contain a small lexer that output Token (will be defined later on and probably expanded too)
pub mod lexer;
/// TODO
pub mod utils;

/// For alphas only
pub mod prelude {}

#[macro_export]
macro_rules! map {
    (&key: ty, &val: ty) => {
        {
            let map: HashMap<&key, &val> = HashMap::new();
            map
        }
    };
    ($($key:expr => $val:expr),*) => {
        {
            let mut map = HashMap::new();
            $(map.insert($key, $val);)*
            map
        }
    }
}
