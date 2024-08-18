# Atlas-core

Crate to host the [Atlas77](https://github.com/RedGear-Studio/Atlas77) project. An embeddable functional language.

# atlas_core

``atlas_core`` is a Rust library designed to facilitate lexical analysis and token parsing for programming languages. It provides a flexible and customizable lexer implementation, along with macros to define language-specific tokens, keywords, and symbols.
Features

- Lexer Generation: Automatically generate lexer structures and methods using macros.
- Token Kind Definition: Easily define various token kinds including literals, keywords, and symbols.
- Error Handling: Robust error handling with detailed error messages and support for recoverable errors.
- Customizable: Configure your lexer with specific keywords, symbols, and number parsing options.

# Installation

Add the following to your ``Cargo.toml``:

```toml

[dependencies]
atlas_core = "0.6.0"
```

# Usage
## Defining Symbols

> Everything is under refactoring, so it won't work like that. Stay tuned for update!

Use the ``symbols!`` macro to define symbols used in your language:

```rust

symbols! {
    '+' => Plus,
    '-' => Minus,
    '*' => Asterisk,
    '/' => Slash,
    // Add more symbols as needed
}
```
## Defining Keywords

Use the ``keyword!`` macro to define keywords:

```rust

keyword! {
    "if",
    "else",
    "while",
    // Add more keywords as needed
}
```
## Configuring Number Parsing

Use the ``number!`` macro to configure number parsing options:

```rust

number! {
    enable_f64: true,
    enable_i64: true
}
```

## Generating the Lexer

Use the ``lexer!`` macro to generate the lexer with default or customized configurations:

```rust

lexer!();
```

# Example

Here's an example of using ``atlas_core`` to create a simple lexer:

```rust

use atlas_core::prelude::*;

symbols! {
    '+' => Plus,
    '{' => LBracket,
    '}' => RBracket,
    ';' => SemiColon,
}

keyword! {
    "if",
    "return"
}

number! {
    enable_f64: true,
    enable_i64: true
}

lexer!();

fn main() {
    let source = "if x + y { return 42; }";
    let tokens = Lexer::tokenize("example.rs", source).unwrap();
    
    for token in tokens {
        println!("{:?}", token);
    }
}
```


# Contributing

Contributions are welcome! Please open an issue or submit a pull request.
# License

This project is licensed under the MIT [License](https://github.com/RedGear-Studio/atlas-core/blob/main/LICENSE). See the LICENSE file for details.