/// TODO
pub mod lexer_state;
/// A macro to generate a `AtlasLexer` struct along with its associated methods for
/// lexical analysis and tokenization of a source string.
///
/// The generated `AtlasLexer` struct manages the lexer systems and provides methods
/// for configuring the lexer and tokenizing a string input into tokens.
///
/// # Generated Struct: `AtlasLexer`
///
/// The macro generates a struct `AtlasLexer` with the following fields:
///
/// - `sys: Vec<fn(char, &mut LexerState) -> Option<Token>>`
///     - A vector of system functions that define how the lexer processes characters.
///       Each function takes a `char` and a mutable reference to a `LexerState` and
///       returns an `Option<Token>`.
///
/// - `path: &'static str`
///     - The path to the source file or a string identifier for the input being lexed.
///
/// - `current_pos: BytePos`
///     - Tracks the current byte position in the source string during lexing.
///
/// - `source: String`
///     - The source string that the lexer processes.
///
/// # Generated Methods:
///
/// The macro also generates several methods for the `AtlasLexer` struct:
///
/// ## `default() -> Self`
///
/// Creates a default instance of `AtlasLexer` with predefined systems (`default_number` and `default_symbol`)
/// and a source path of `"<stdin>"`. This method initializes the lexer with an empty source string.
///
/// ## `new(path: &'static str, source: String) -> Self`
///
/// Creates a new instance of `AtlasLexer` with the specified source path and source string.
/// The systems are initialized as an empty vector.
///
/// - `path`: The path or identifier for the source being lexed.
/// - `source`: The source string to be lexed.
///
/// ## `set_source(&mut self, source: String) -> &mut Self`
///
/// Sets a new source string for the lexer and returns a mutable reference to the `AtlasLexer` instance,
/// allowing method chaining.
///
/// - `source`: The new source string to be lexed.
///
/// ## `set_path(&mut self, new_path: &'static str) -> &mut Self`
///
/// Sets a new source path or identifier for the lexer and returns a mutable reference to the `AtlasLexer` instance,
/// allowing method chaining.
///
/// - `new_path`: The new path or identifier for the source being lexed.
///
/// ## `add_system(&mut self, s: fn(char, &mut LexerState) -> Option<Token>) -> &mut Self`
///
/// Adds a new system function to the lexer, which defines how specific characters are processed into tokens.
/// Returns a mutable reference to the `AtlasLexer` instance, allowing method chaining.
///
/// - `s`: A function pointer that defines the behavior for processing a character into a token.
///
/// ## `tokenize(&mut self) -> Result<Vec<Token>, ()>`
///
/// Tokenizes the source string using the registered system functions. The lexer iterates over each character
/// in the source string, applying the system functions in order. If a system function successfully produces a
/// token, it is added to the output vector of tokens. If no systems match a character, an error is returned.
///
/// Returns a `Result` containing a vector of tokens on success, or an empty tuple `()` on failure.
///
/// # Example:
///
/// ```compile_fail
/// lexer_builder!();
///
/// fn main() {
///     let mut lexer = AtlasLexer::default();
///     lexer.set_source("123.45".to_string());
///     let tokens = lexer.tokenize().unwrap();
///     println!("{:?}", tokens);
/// }
/// ```
///
/// # Notes:
///
/// - The macro also includes a default system function `default_number` that processes numeric literals & default_symbol that processes symbols.
/// - The generated lexer is designed to be extendable with additional systems to handle different types of tokens.
/// - Error handling is rudimentary and will need to be extended based on the specific requirements of the lexer.
#[macro_export]
macro_rules! lexer_builder {
    () => {
        #[derive(Debug, Default)]
        pub struct AtlasLexer {
            sys: Vec<fn(char, &mut LexerState) -> Option<Token>>,
            path: &'static str,
            pub current_pos: BytePos,
            pub source: String,
        }
        impl AtlasLexer {
            pub fn default() -> Self {
                let mut lexer = AtlasLexer::new("<stdin>", String::new());
                lexer.add_system(default_number).add_system(default_symbol);
                lexer
            }
            pub fn new(path: &'static str, source: String) -> Self {
                Self {
                    sys: vec![],
                    path,
                    current_pos: BytePos::from(0),
                    source,
                }
            }

            pub fn set_source(&mut self, source: String) -> &mut Self {
                self.source = source;
                self
            }

            pub fn set_path(&mut self, new_path: &'static str) -> &mut Self {
                self.path = new_path;
                self
            }

            pub fn add_system(
                &mut self,
                s: fn(char, &mut LexerState) -> Option<Token>,
            ) -> &mut Self {
                self.sys.push(s);
                self
            }

            //A way of handling errors will come later
            pub fn tokenize(&mut self) -> Result<Vec<Token>, ()> {
                let mut tok: Vec<Token> = vec![];
                loop {
                    let ch = self.source.chars().nth(usize::from(self.current_pos));
                    match ch {
                        Some(c) => {
                            let state = LexerState::new(
                                self.current_pos,
                                self.source
                                    .get(usize::from(self.current_pos)..self.source.len())
                                    .unwrap(),
                            );
                            let mut counter = 0;
                            for f in &self.sys {
                                let mut current_state = state.clone();
                                match f(c, &mut current_state) {
                                    Some(f) => {
                                        tok.push(f);
                                        self.current_pos = current_state.current_pos;
                                        break;
                                    }
                                    None => {
                                        counter += 1;
                                        continue;
                                    }
                                }
                            }
                            if counter >= self.sys.len() {
                                return Err(());
                            }
                        }
                        None => break,
                    }
                }
                return Ok(tok);
            }
        }
        fn default_number(c: char, state: &mut LexerState) -> Option<Token> {
            if c.is_numeric() {
                let start = state.current_pos;
                let mut n = String::new();
                n.push(c);
                state.next();
                loop {
                    if let Some(c) = state.peek() {
                        if c.is_numeric() {
                            n.push(*c);
                            state.next();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                if let Some(c) = state.peek() {
                    if *c == '.' {
                        n.push(*c);
                        state.next();
                        loop {
                            if let Some(c) = state.peek() {
                                if c.is_numeric() {
                                    n.push(*c);
                                    state.next();
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }
                Some(Token::new(
                    Span {
                        start,
                        end: state.current_pos,
                        path: "<stdin>",
                    },
                    TokenKind::Literal(Literal::Float(n.parse::<f64>().unwrap())),
                ))
            } else {
                None
            }
        }
        fn default_symbol(c: char, state: &mut LexerState) -> Option<Token> {
            let start = state.current_pos;
            let tok = match c {
                '+' => TokenKind::Plus,
                '-' => TokenKind::Minus,
                '*' => TokenKind::Asterisk,
                '/' => TokenKind::Slash,
                '%' => TokenKind::Percent,
                '=' => TokenKind::Equal,
                //'==' => EqualEqual,
                //'!=' => NotEqual,
                '<' => TokenKind::LessThan,
                '>' => TokenKind::GreaterThan,
                //'<=' => LessThanEqual,
                //'>=' => GreaterThanEqual,
                '!' => TokenKind::Exclamation,
                '&' => TokenKind::Ampersand,
                //'&&' => DoubleAmpersand,
                '|' => TokenKind::Pipe,
                //'||' => DoublePipe,
                '^' => TokenKind::Caret,
                '~' => TokenKind::Tilde,
                //'<<' => LeftShift,
                //'>>' => RightShift,
                '(' => TokenKind::LeftParen,
                ')' => TokenKind::RightParen,
                '[' => TokenKind::LeftBracket,
                ']' => TokenKind::RightBracket,
                '{' => TokenKind::LeftBrace,
                '}' => TokenKind::RightBrace,
                '.' => TokenKind::Dot,
                //'..' => DoubleDot,
                //'...' => Ellipsis,
                ',' => TokenKind::Comma,
                ';' => TokenKind::Semicolon,
                ':' => TokenKind::Colon,
                //'::' => DoubleColon,
                '?' => TokenKind::Question,
                '#' => TokenKind::Hash,
                '$' => TokenKind::Dollar,
                '@' => TokenKind::At,
                '\\' => TokenKind::Backslash,
                '\'' => TokenKind::SingleQuote,
                '"' => TokenKind::DoubleQuote,
                '`' => TokenKind::Backtick,
                _ => return None,
            };
            state.next();
            Some(Token::new(
                Span {
                    start,
                    end: state.current_pos,
                    path: "<stdin>",
                },
                tok,
            ))
        }
    };
}
//A macro for defining the `TokenKind` enum and related types, including predefined symbols 
/// and operators. The macro allows for flexible creation of the `TokenKind` variants based 
/// on character literals and maps them to specific enum variants.
///
/// # Overview
///
/// This macro generates:
/// - A `Token` struct that represents a token in the lexer, holding its span and kind.
/// - An implementation of the `Spanned` trait for `Token`, providing the span of the token.
/// - A `TokenKind` enum that categorizes different types of tokens, including literals,
///   keywords, and various symbols and operators.
/// - A `Literal` enum representing literal values in the source code, such as integers,
///   floats, booleans, identifiers, and string literals.
///
/// # Usage
///
/// The macro can be invoked in two ways:
///
/// 1. **With Symbol-Variant Pairs:**
/// 
///    This usage defines specific character literals as symbols and maps them to corresponding
///    variants in the `TokenKind` enum.
///
///    ```compile_fail
///    symbols! {
///        '+' => Plus,
///        '-' => Minus,
///        '*' => Asterisk,
///        '/' => Slash,
///        '%' => Percent,
///        '=' => Equal,
///        '<' => LessThan,
///        '>' => GreaterThan,
///        '!' => Exclamation,
///        '&' => Ampersand,
///        '|' => Pipe,
///        '^' => Caret,
///        '~' => Tilde,
///        '(' => LeftParen,
///        ')' => RightParen,
///        '[' => LeftBracket,
///        ']' => RightBracket,
///        '{' => LeftBrace,
///        '}' => RightBrace,
///        '.' => Dot,
///        ',' => Comma,
///        ';' => Semicolon,
///        ':' => Colon,
///        '?' => Question,
///        '#' => Hash,
///        '$' => Dollar,
///        '@' => At,
///        '\\' => Backslash,
///        '\'' => SingleQuote,
///        '"' => DoubleQuote,
///        '`' => Backtick
///    }
///    ```
/// 
///    This expands into the `TokenKind` enum with the specified variants for each symbol:
///
///    ```compile_fail
///    #[derive(Debug, Clone, Copy, PartialEq)]
///    pub enum TokenKind {
///        Literal(Literal),
///        Keyword(Intern<String>),
///        Plus,
///        Minus,
///        Asterisk,
///        Slash,
///        Percent,
///        Equal,
///        LessThan,
///        GreaterThan,
///        Exclamation,
///        Ampersand,
///        Pipe,
///        Caret,
///        Tilde,
///        LeftParen,
///        RightParen,
///        LeftBracket,
///        RightBracket,
///        LeftBrace,
///        RightBrace,
///        Dot,
///        Comma,
///        Semicolon,
///        Colon,
///        Question,
///        Hash,
///        Dollar,
///        At,
///        Backslash,
///        SingleQuote,
///        DoubleQuote,
///        Backtick,
///        EoI,
///        SoI
///    }
///    ```
///
/// 2. **Default Symbol Set:**
/// 
///    If no arguments are passed, the macro defaults to a predefined set of commonly used symbols
///    and operators, generating corresponding `TokenKind` variants for each.
///
///    ```compile_fail
///    symbols!();
///    ```
/// 
///    This generates the same `TokenKind` enum as shown in the example above.
///
/// # Generated Types:
///
/// ## `Token` Struct
/// Represents a token with its span and kind.
///
/// - `span: Span`
///     - The span of the token within the source code.
/// - `kind: TokenKind`
///     - The kind of token, represented as a `TokenKind` enum variant.
///
/// ### Implemented Traits:
/// - `Spanned`: Provides a `span()` method to retrieve the token's span.
/// - `Debug`, `Clone`, `Copy`, `PartialEq`: Derives standard traits for comparison, copying, and debugging.
///
/// ## `TokenKind` Enum
/// Represents different kinds of tokens that the lexer can produce, including literals, keywords,
/// and various symbols.
///
/// - `Literal(Literal)`
///     - Represents literal values like numbers, booleans, identifiers, and strings.
/// - `Keyword(Intern<String>)`
///     - Represents a keyword in the source language.
/// - Custom symbol variants (e.g., `Plus`, `Minus`, etc.)
///     - Generated based on the symbols provided in the macro invocation.
///
/// ## `Literal` Enum
/// Represents different types of literal values in the source code.
///
/// - `Int(i64)`
///     - Represents an integer literal.
/// - `Float(f64)`
///     - Represents a floating-point literal.
/// - `Bool(bool)`
///     - Represents a boolean literal.
/// - `Identifier(Intern<String>)`
///     - Represents an identifier (variable or function name).
/// - `StringLiteral(Intern<String>)`
///     - Represents a string literal.
///
/// # Example:
///
/// ```compile_fail
/// symbols!();
/// 
/// fn main() {
///     // Example usage of the generated TokenKind variants.
///     let plus_token = Token::new(Span::new(0, 1), TokenKind::Plus);
///     println!("{:?}", plus_token);
/// }
/// ```
///
/// # Notes:
///
/// - The macro simplifies the creation and management of lexer token types, making it easier to
///   extend or modify the lexer's behavior.
/// - The predefined symbols cover a wide range of common operators and delimiters, but the macro
///   allows for full customization if needed.
#[macro_export]
macro_rules! symbols {
    ($($sym:literal => $variant:ident),* ) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct Token {
            span: Span,
            kind: TokenKind,
        }

        impl Spanned for Token {
            #[inline(always)]
            fn span(&self) -> Span {
                self.span
            }
        }

        impl Token {
            pub fn new(span: Span, kind: TokenKind) -> Self {
                Self { span, kind }
            }
            #[inline(always)]
            pub fn kind(&self) -> TokenKind {
                self.kind
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum Literal {
            ///Default int literal, may change in the parser based on the type of the variable
            Int(i64),

            ///Default float literal, may change in the parser based on the type of the variable
            Float(f64),

            Bool(bool),
            //At this point, types don't exist in the parser, it's just Identifier
            Identifier(Intern<String>),

            StringLiteral(Intern<String>),
        }

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum TokenKind {
            /// A literal see [Literal] for more information
            Literal(Literal),

            /// A keyword
            Keyword(Intern<String>),
            $(
                $variant,
            )*
            EoI,
            SoI
        }

    };
    () => {
        symbols!{
            '+' => Plus,
            '-' => Minus,
            '*' => Asterisk,
            '/' => Slash,
            '%' => Percent,
            '=' => Equal,
            //'==' => EqualEqual,
            //'!=' => NotEqual,
            '<' => LessThan,
            '>' => GreaterThan,
            //'<=' => LessThanEqual,
            //'>=' => GreaterThanEqual,
            '!' => Exclamation,
            '&' => Ampersand,
            //'&&' => DoubleAmpersand,
            '|' => Pipe,
            //'||' => DoublePipe,
            '^' => Caret,
            '~' => Tilde,
            //'<<' => LeftShift,
            //'>>' => RightShift,
            '(' => LeftParen,
            ')' => RightParen,
            '[' => LeftBracket,
            ']' => RightBracket,
            '{' => LeftBrace,
            '}' => RightBrace,
            '.' => Dot,
            //'..' => DoubleDot,
            //'...' => Ellipsis,
            ',' => Comma,
            ';' => Semicolon,
            ':' => Colon,
            //'::' => DoubleColon,
            '?' => Question,
            '#' => Hash,
            '$' => Dollar,
            '@' => At,
            '\\' => Backslash,
            '\'' => SingleQuote,
            '"' => DoubleQuote,
            '`' => Backtick
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn hehe() {
        use crate::prelude::*;
        lexer_builder!();
        symbols! {
            '+' => Plus,
            '-' => Minus,
            '*' => Asterisk,
            '/' => Slash,
            '%' => Percent,
            '=' => Equal,
            //'==' => EqualEqual,
            //'!=' => NotEqual,
            '<' => LessThan,
            '>' => GreaterThan,
            //'<=' => LessThanEqual,
            //'>=' => GreaterThanEqual,
            '!' => Exclamation,
            '&' => Ampersand,
            //'&&' => DoubleAmpersand,
            '|' => Pipe,
            //'||' => DoublePipe,
            '^' => Caret,
            '~' => Tilde,
            //'<<' => LeftShift,
            //'>>' => RightShift,
            '(' => LeftParen,
            ')' => RightParen,
            '[' => LeftBracket,
            ']' => RightBracket,
            '{' => LeftBrace,
            '}' => RightBrace,
            '.' => Dot,
            //'..' => DoubleDot,
            //'...' => Ellipsis,
            ',' => Comma,
            ';' => Semicolon,
            ':' => Colon,
            //'::' => DoubleColon,
            '?' => Question,
            '#' => Hash,
            '$' => Dollar,
            '@' => At,
            '\\' => Backslash,
            '\'' => SingleQuote,
            '"' => DoubleQuote,
            '`' => Backtick
        };
        let mut lexer = AtlasLexer::default();
        lexer
            .set_path("<stdin>")
            .set_source(String::from("256245.325,;25{}"));
        match lexer.tokenize() {
            Ok(toks) => {
                for t in toks {
                    println!("{:?}", t);
                }
            }
            Err(e) => {
                println!("Sale batard {:?}", e);
            }
        }
    }
}
