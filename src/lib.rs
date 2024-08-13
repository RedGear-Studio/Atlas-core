/*
 * atlas-core by Gipson62
 *
 * The core of a tool designed to help you make programming language
 * 
 * There will be a lexer macro that build up a fully fledged lexer for you with no efforts.
 * It's currently in development, more information coming soon
*/

#![warn(missing_docs, unused)]

//! # atlas-Core
//!
//! `atlas-core` is the foundational library for a language creation tool designed to assist users in developing languages.
//! This core library currently serves as the building block for the creation of Atlas77, a functional programming language.
//! 
//! Currently, it's only purpose is to generate a Lexer and the way to do it is pretty straightforward
//! # Example:
//! ```
//! use atlas_core::prelude::*;
//! //Generate the base Lexer and useful features used across the whole lexer
//! lexer!()
//! //Generate all the keywords (see the keywords section for more information)
//! keyword!("and", "or", "if", "while")
//! //Generate all the symbol you want and will use in your language (currently no default implementation)
//! //symbol!('$' => DollarSign, '#' => HashTag)
//! //Configure the number parser (more configuration to come and a nice way to enable them too)
//! number!(enable_f64: true, enable_i64: true)
//! ```
//! NB: the order of macros invocation doesn't matter, but their invocations are mandatory (you aren't obliged to give them arguments)

/// Contain a suite of macros to generate a fully fledge lexer tailored to the user needs
pub mod lexer;
/// TODO
pub mod utils;

#[doc = "Used to import the base set of features of this tool"]
pub mod prelude {
    pub use crate::map;
    pub use crate::lexer;
    pub use crate::symbols;
    pub use crate::number;
    pub use crate::keyword;
    pub use internment::Intern;
    pub use crate::utils::{span::*, case::Case};
}

#[macro_export]
/// A macro to create and initialize a `HashMap` with specified key-value pairs or 
/// to declare a mutable `HashMap` with specific key and value types.
///
/// This macro provides two functionalities:
/// 1. Declaration of a mutable `HashMap` with specified key and value types.
/// 2. Initialization of a `HashMap` with a list of key-value pairs.
///
/// # Syntax
///
/// The macro can be invoked in two forms:
///
/// 1. **Declare a mutable `HashMap`**:
///    ```
///    map!(name, &key_type, &val_type);
///    ```
///    - `name`: The name of the `HashMap` variable to declare.
///    - `key_type`: The type of the keys in the `HashMap`.
///    - `val_type`: The type of the values in the `HashMap`.
///
/// 2. **Initialize a `HashMap` with key-value pairs**:
///    ```
///    map!(key1 => val1, key2 => val2, ...);
///    ```
///    - `key => val`: Each key-value pair to insert into the `HashMap`.
///
/// # Example
///
/// ```rust
/// use std::collections::HashMap;
///
/// // Declare a mutable HashMap with specified key and value types
/// map!(my_map, &str, i32);
/// my_map.insert("key1", 10);
/// my_map.insert("key2", 20);
///
/// // Initialize a HashMap with key-value pairs
/// let initialized_map = map!(
///     "one" => 1,
///     "two" => 2,
///     "three" => 3,
/// );
///
/// assert_eq!(initialized_map.get(&"one"), Some(&1));
/// assert_eq!(initialized_map.get(&"two"), Some(&2));
/// assert_eq!(initialized_map.get(&"three"), Some(&3));
/// ```
macro_rules! map {
    ($name:ident, &key: ty, &val: ty) => {
        let mut $name: HashMap<&key, &val> = HashMap::new();
    };
    ($($key:expr => $val:expr),* $(,)?) => {
        {
            let mut map = HashMap::new();
            $(map.insert($key, $val);)*
            map
        }
    }
}
