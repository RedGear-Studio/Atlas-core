/// A macro to define and initialize a lexer with default or customized configurations.
///
/// This macro generates the `Lexer` struct and implements necessary methods and error handling 
/// to facilitate lexical analysis of a given input. It includes functions for tokenizing input, 
/// consuming characters based on conditions, and handling errors encountered during the lexing process.
///
/// # Syntax
///
/// The macro can be invoked without any parameters:
/// ```compile_fail
/// lexer!();
/// ```
/// This initializes a lexer with default configurations and provides the necessary methods and structures.
///
/// # Generated Structures and Implementations
///
/// The macro generates the following:
///
/// - `struct Lexer<'lex>`: The lexer structure which holds the state and methods for lexical analysis.
///   - Fields:
///     - `path`: The file path of the input.
///     - `current_pos`: The current byte position in the input.
///     - `it`: An iterator over the characters of the input.
///     - `keywords`: A `HashMap` for storing keywords and their corresponding token kinds.
///
/// - `impl Lexer<'_>`: Implementation block for the lexer methods.
///   - `fn tokenize(path: &'static str, contents: &str) -> Result<Vec<Token>, Box<dyn Error>>`: Tokenizes the given input.
///   - `fn new(path: &'static str, contents: &'a str) -> Self`: Initializes a new lexer instance.
///   - `fn next(&mut self) -> Option<char>`: Advances to the next character.
///   - `fn peek(&mut self) -> Option<&char>`: Peeks at the next character without consuming it.
///   - `fn either(&mut self, to_match: char, matched: TokenKind, unmatched: TokenKind) -> TokenKind`: Matches a character against two token kinds.
///   - `fn consume_if<F>(&mut self, f: F) -> bool where F: Fn(char) -> bool`: Consumes the next character if it matches a condition.
///   - `fn consume_if_next<F>(&mut self, f: F) -> bool where F: Fn(char) -> bool`: Consumes the next character if the next character matches a condition.
///   - `fn consume_while<F>(&mut self, f: F) -> Vec<char> where F: Fn(char) -> bool`: Consumes characters while a condition is true.
///
/// - `enum LexError`: Enumeration for representing lexing errors.
///   - Variants:
///     - `UnknownCharacter`: Represents an unknown character error.
///     - `UnexpectedEndOfInput`: Represents an unexpected end of input error.
///     - `UnsupportedNumber`: Represents an unsupported number error.
///
/// - `trait Error`: Trait for defining error handling.
///   - Methods:
///     - `fn code(&self) -> u64`: Returns the error code.
///     - `fn recoverable(&self) -> bool`: Indicates if the error is recoverable.
///     - `fn help(&self) -> Option<String>`: Provides help information for the error.
///     - `fn message(&self) -> String`: Returns the error message.
///
/// - `impl Error for LexError`: Implementation of the `Error` trait for `LexError`.
///
/// - `impl Spanned for LexError`: Implementation of the `Spanned` trait for `LexError`.
///
/// - `impl Display for LexError`: Implementation of the `Display` trait for `LexError`.
///
/// # Example Usage in Lexer
///
/// ```compile_fail
/// lexer!();
///
/// impl Lexer<'_> {
///     fn example(&self) {
///         // Example method using the generated lexer structure and methods.
///     }
/// }
/// ```
#[macro_export]
macro_rules! lexer {
    () => {
use atlas_core::utils::span::*;
use internment::Intern;
use std::{collections::HashMap, fmt::Display, iter::Peekable, str::Chars};

pub struct Lexer<'lex> {
    path: &'static str,
    current_pos: BytePos,
    it: Peekable<Chars<'lex>>,
    keywords: HashMap<Intern<String>, TokenKind>,
}

impl Lexer<'_> {
    pub fn tokenize(path: &'static str, contents: &str) -> Result<Vec<Token>, Box<dyn Error>> {
        let mut lexer = Lexer::new(path, contents);
        let mut tokens = vec![Token::new(
            Span {
                start: BytePos::default(),
                end: BytePos::default(),
                path: path,
            },
            TokenKind::SoI,
        )];

        loop {
            let start_pos = lexer.current_pos;
            let ch = match lexer.next() {
                None => break,
                Some(c) => c,
            };

            match lexer.lex(ch) {
                Ok(kind) => {
                    tokens.push(Token::new(
                        Span {
                            start: start_pos,
                            end: lexer.current_pos,
                            path: lexer.path,
                        },
                        kind,
                    ));
                    if kind == TokenKind::EoI {
                        break;
                    }
                }
                Err(err) => {
                    println!("Error: {}", err);
                    if !err.recoverable() {
                        break;
                    }
                }
            }
        }
        return Ok(tokens);
    }
}



impl<'a> Lexer<'a> {
    /// Create a new empty `Lexer`
    /// Is it really how I should do it?
    fn new(path: &'static str, contents: &'a str) -> Self {
        let mut a = Lexer {
            path,
            it: contents.chars().peekable(),
            current_pos: BytePos::default(),
            keywords: HashMap::new(),
        };
        a.populate_keyword();
        a
    }

    fn next(&mut self) -> Option<char> {
        let next = self.it.next();
        if let Some(ch) = next {
            self.current_pos = self.current_pos.shift(ch);
        }
        next
    }

    fn peek(&mut self) -> Option<&char> {
        self.it.peek()
    }

    fn either(&mut self, to_match: char, matched: TokenKind, unmatched: TokenKind) -> TokenKind {
        if self.consume_if(|c| c == to_match) {
            matched
        } else {
            unmatched
        }
    }

    fn consume_if<F>(&mut self, f: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        if let Some(&ch) = self.it.peek() {
            if f(ch) {
                self.next().unwrap();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn consume_if_next<F>(&mut self, f: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        let mut it = self.it.clone();
        match it.next() {
            None => return false,
            _ => (),
        }

        if let Some(&ch) = it.peek() {
            if f(ch) {
                self.next().unwrap();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn consume_while<F>(&mut self, f: F) -> Vec<char>
    where
        F: Fn(char) -> bool,
    {
        let mut chars: Vec<char> = Vec::new();
        while let Some(&ch) = self.peek() {
            if f(ch) {
                self.next().unwrap();
                chars.push(ch);
            } else {
                break;
            }
        }
        chars
    }

}

#[derive(Debug, Clone, Copy)]
pub(crate) enum LexError {
    UnknownCharacter {
        ch: char,
        code: u64,
        span: Span,
        recoverable: bool,
    },
    UnexpectedEndOfInput {
        span: Span,
        recoverable: bool,
        code: u64,
    },
    UnsupportedNumber {
        span: Span,
        recoverable: bool,
        code: u64
    },
}

pub trait Error {
    fn code(&self) -> u64;
    fn recoverable(&self) -> bool;
    fn help(&self) -> Option<String>;
    fn message(&self) -> String;
}

impl Error for LexError {
    fn code(&self) -> u64 {
        match self {
            LexError::UnknownCharacter { code, .. } => *code,
            LexError::UnexpectedEndOfInput { code, .. } => *code,
            LexError::UnsupportedNumber { code, .. } => *code,
        }
    }
    fn recoverable(&self) -> bool {
        match self {
            LexError::UnknownCharacter { recoverable, .. } => *recoverable,
            LexError::UnexpectedEndOfInput { recoverable, .. } => *recoverable,
            LexError::UnsupportedNumber { recoverable, .. } => *recoverable
        }
    }
    ///Todo
    fn help(&self) -> Option<String> {
        None
    }
    fn message(&self) -> String {
        match self {
            LexError::UnknownCharacter { ch, span, .. } => {
                format!("Unknown character: {} here: {}", ch, span)
            }
            LexError::UnexpectedEndOfInput { span, .. } => {
                format!("Unexpected end of input here: {}", span)
            }
            LexError::UnsupportedNumber { span, .. } => {
                format!("It looks like you're trying to parse a number your lexer doesn't support here: {}", span)
            }
        }
    }
}

impl Spanned for LexError {
    fn span(&self) -> Span {
        match self {
            LexError::UnknownCharacter { span, .. } => *span,
            LexError::UnexpectedEndOfInput { span, .. } => *span,
            LexError::UnsupportedNumber { span, .. } => *span
        }
    }
}

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //will change this later to use ariadne maybe for better error messages
        write!(f, "{}", self.message())
    }
}
    };
}


#[macro_export]
/// A macro to define keywords recognized by a lexer and map them to `TokenKind::Keyword` variants.
/// 
/// This macro generates an implementation of the `populate_keyword` method in the `Lexer` struct 
/// to populate a `HashMap` with specified keywords or to initialize an empty `HashMap` if no keywords are provided.
/// 
/// # Syntax
/// 
/// The macro can be invoked in two forms:
/// 
/// 1. **Custom keywords**:
///    ```compile_fail
///    keyword!( "keyword1", "keyword2", ... );
///    ```
///    - Each keyword is a string literal that will be recognized by the lexer and mapped to a `TokenKind::Keyword` variant.
/// 
/// 2. **No keywords**:
///    ```compile_fail
///    keyword!();
///    ```
///    - Initializes an empty `HashMap` for keywords.
/// 
/// # Example
/// 
/// ```compile_fail
/// // Define custom keywords
/// keyword!( "fn", "let", "if" );
/// 
/// // Initialize an empty keyword map
/// keyword!();
/// ```
/// 
/// # Generated Methods
/// 
/// The macro generates the following method within the `Lexer` struct:
/// 
/// - `fn populate_keyword(&mut self)`:
///   - Populates the `keywords` field of the `Lexer` struct with the specified keywords or initializes it as empty.
/// 
macro_rules! keyword {
    ($($x:literal),* $(,)?) => {
        impl Lexer<'_> {
            fn populate_keyword(&mut self) {
                use atlas_core::map;
                self.keywords = map! {
                    $(
                        Intern::new(String::from($x)) => TokenKind::Keyword(Intern::new(String::from($x))),
                    )*
                }
            }
        }
    };
    () => {
        use atlas_core::map;
        impl Lexer {
            fn populate_keyword(&mut self) {
                self.keywords = map!();
            }
        }
    }
}

#[macro_export]
/// A macro to define the symbols recognized by a lexer and map them to specific token variants.
/// 
/// This macro generates an implementation of the `Token` struct, `TokenKind` enum, and 
/// related methods in the `Lexer` struct to handle single-character symbols. It also 
/// includes default implementations if no symbols are specified.
/// 
/// **Note:** This macro does not support multi-character symbols.
/// 
/// # Syntax
/// 
/// The macro can be invoked in two forms:
/// 
/// 1. **Custom symbols**:
///    ```compile_fail
///    symbols!( 'symbol' => Variant, ... );
///    ```
///    - `symbol`: A single-character symbol to be recognized by the lexer.
///    - `Variant`: The corresponding variant in the `TokenKind` enum.
/// 
/// 2. **Default symbols**:
///    ```compile_fail
///    symbols!();
///    ```
///    - Uses a predefined set of single-character symbols and their corresponding `TokenKind` variants.
/// 
/// # Example
/// 
/// ```compile_fail
/// // Define custom symbols
/// symbols!(
///     '+' => Plus,
///     '-' => Minus,
///     '*' => Asterisk
/// );
/// 
/// // Use default symbols
/// symbols!();
/// ```
/// 
/// # Generated Types and Methods
/// 
/// The macro generates the following types and methods:
/// 
/// - `Token` struct: Represents a token with a span and kind.
/// - `TokenKind` enum: Enumerates the different kinds of tokens, including literals, keywords, and symbols.
/// - Implementation of the `Spanned` trait for the `Token` struct.
/// - Methods in the `Lexer` struct to handle symbols and identify tokens.
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

        impl Lexer<'_> {
            fn lex(&mut self, ch: char) -> Result<TokenKind, LexError> {
                match ch {
                    '\n' | '\t' | ' ' | '\r' => {
                        if !self.peek().is_none() {
                            let ch = self.next().unwrap();
                            self.lex(ch)
                        } else {
                            Err(LexError::UnexpectedEndOfInput {
                                span: Span {
                                    start: self.current_pos,
                                    end: self.current_pos,
                                    path: self.path,
                                },
                                recoverable: false,
                                code: 2,
                            })
                        }
                    },
                    $(
                        $sym => Ok(TokenKind::$variant),
                    )*
                    x if x.is_numeric() => {
                        match self.number(x) {
                            Some(n) => {
                                Ok(n)
                            },
                            None => {
                                Err(LexError::UnsupportedNumber {
                                    code: 3,
                                    span: Span {
                                        start:self.current_pos,
                                        end:self.current_pos.shift(x),
                                        path:self.path
                                    },
                                    recoverable: true
                                })
                            }
                        }
                    },
                    ch if ch.is_alphabetic() || ch == '_' => Ok(self.identifier(ch).unwrap()),
                    '"' => {
                        let mut string = String::new();
                        string.push_str(
                            self.consume_while(|ch| ch != '"')
                                .iter()
                                .collect::<String>()
                                .as_ref(),
                        );
                        self.next().unwrap();
                        Ok(TokenKind::Literal(Literal::StringLiteral(Intern::new(
                            string,
                        ))))
                    }
                    c => Err(LexError::UnknownCharacter {
                        ch: c,
                        code: 0,
                        span: Span {
                            start: self.current_pos,
                            end: self.current_pos.shift(c),
                            path: self.path,
                        },
                        recoverable: true,
                    }),
                }
            }
            
            fn identifier(&mut self, c: char) -> Option<TokenKind> {
                let mut ident = String::new();
                ident.push(c);

                while let Some(&ch) = self.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        ident.push(self.next().unwrap());
                    } else {
                        break;
                    }
                }
                let id = Intern::new(ident.to_owned());

                if let Some(k) = self.keywords.get(&id) {
                    Some(k.clone())
                } else {
                    Some(TokenKind::Literal(Literal::Identifier(id)))
                }
            }
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
        }
    }
}

#[macro_export]
/// A macro to define number handling methods in a lexer, with configurable support for 
/// floating-point and integer literals.
///
/// This macro generates an implementation of the `Lexer` struct that includes methods 
/// for recognizing and parsing numeric literals from the input. It provides options to 
/// enable or disable support for floating-point (`f64`) and integer (`i64`) literals.
///
/// # Syntax
///
/// The macro can be invoked in three forms:
///
/// 1. **Full configuration**:
///    ```compile_fail
///    number!(enable_f64: bool, enable_i64: bool);
///    ```
///    - `enable_f64`: Enables or disables support for `f64` literals.
///    - `enable_i64`: Enables or disables support for `i64` literals.
///
/// 2. **Single boolean configuration**:
///    ```compile_fail
///    number!(bool);
///    ```
///    - This form enables or disables both `f64` and `i64` support with the same boolean value.
///
/// 3. **Default configuration**:
///    ```compile_fail
///    number!();
///    ```
///    - This form enables support for both `f64` and `i64` literals by default.
///
/// # Generated Methods
///
/// The macro generates the following methods within the `Lexer` struct:
///
/// - `fn number(&mut self, c: char) -> Option<TokenKind>`
///   - Recognizes and constructs a numeric literal starting with the given character `c`.
/// - `fn is_number(&self, s: &str) -> Option<TokenKind>`
///   - Determines if the given string `s` is a valid numeric literal and returns the 
///     corresponding `TokenKind`.
macro_rules! number {
    (enable_f64: $f64:expr, enable_i64: $i64:expr) => {
        impl Lexer<'_> {
        fn number(&mut self, c: char) -> Option<TokenKind> {
            let mut number = String::new();
            number.push(c);
    
            let num: String = self.consume_while(|a| a.is_numeric()).into_iter().collect();
            number.push_str(&num);
    
            if self.peek() == Some(&'.') && self.consume_if_next(|c| c.is_numeric()) {
                number.push('.');
    
                let num: String = self.consume_while(|a| a.is_numeric()).into_iter().collect();
                number.push_str(&num);
    
            }
            self.is_number(&number)
        }
        fn is_number(&self, s: &str) -> Option<TokenKind> {
            
            if $i64 {
                if let Ok(i) = s.parse::<i64>() {
                    Some(TokenKind::Literal(Literal::Int(i)))
                } else {
                    None
                }
            } else if $f64 {
                if let Ok(f) = s.parse::<f64>() {
                Some(TokenKind::Literal(Literal::Float(f)))
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
    };
    ($x:expr) => {
        number!(enable_f64: $x, enable_i64: $x)
    };
    () => {
        number!(enable_f64: true, enable_i64: true)
    }
}