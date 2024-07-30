/// This Lexer currently only accept single symbol, but identifier, numbers, strings and chars are treated by default.
/// You can list all the reserved keywords you want here (Just need to put the string)
#[macro_export]
macro_rules! lexer {
    ($($x:expr), *) => {
        use crate::map;
use crate::utils::span::*;
use core::fmt;
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
            }
            '(' => Ok(TokenKind::LParen),
            ')' => Ok(TokenKind::RParen),
            '{' => Ok(TokenKind::LBrace),
            '}' => Ok(TokenKind::RBrace),
            '[' => Ok(TokenKind::LBracket),
            ']' => Ok(TokenKind::RBracket),
            '+' => Ok(TokenKind::Plus),
            '_' => Ok(TokenKind::Underscore),
            '-' => Ok(self.either('>', TokenKind::RArrow, TokenKind::Minus)),
            '*' => Ok(TokenKind::Star),
            //TODO: Add support for multiline comments
            '/' => {
                if self.consume_if(|c| c == '/') {
                    self.consume_while(|c| c != '\n');
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
                } else {
                    Ok(TokenKind::Slash)
                }
            }
            '\\' => {
                //Add support for escaping characters
                Ok(TokenKind::Backslash)
            }
            '%' => Ok(TokenKind::Percent),
            '^' => Ok(TokenKind::Caret),
            '<' => {
                if self.consume_if(|c| c == '=') {
                    Ok(TokenKind::LtEq)
                } else {
                    Ok(self.either('-', TokenKind::LArrow, TokenKind::LAngle))
                }
            }
            '>' => Ok(self.either('=', TokenKind::GtEq, TokenKind::RAngle)),
            '=' => {
                if self.consume_if(|ch| ch == '>') {
                    Ok(TokenKind::FatArrow)
                } else {
                    Ok(self.either('=', TokenKind::DoubleEq, TokenKind::Eq))
                }
            }
            '&' => Ok(TokenKind::Ampersand),
            '|' => Ok(TokenKind::Pipe),
            '!' => Ok(self.either('=', TokenKind::NEq, TokenKind::Bang)),
            //Logical
            ':' => Ok(self.either(':', TokenKind::DoubleColon, TokenKind::Colon)),
            ';' => Ok(TokenKind::SemiColon),
            ',' => Ok(TokenKind::Comma),
            '.' => Ok(self.either('.', TokenKind::DoubleDot, TokenKind::Dot)),
            '@' => Ok(TokenKind::At),
            '#' => Ok(TokenKind::HashTag),
            '~' => Ok(TokenKind::Tilde),
            '?' => Ok(TokenKind::Question),
            '$' => Ok(TokenKind::Dollar),
            //Identifiers
            ch if ch.is_alphabetic() || ch == '_' => Ok(self.identifier(ch).unwrap()),
            x if x.is_numeric() => Ok(self.number(x).unwrap()),
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

    fn number(&mut self, c: char) -> Option<TokenKind> {
        let mut number = String::new();
        number.push(c);

        let num: String = self.consume_while(|a| a.is_numeric()).into_iter().collect();
        number.push_str(&num);

        if self.peek() == Some(&'.') && self.consume_if_next(|c| c.is_numeric()) {
            number.push('.');

            let num: String = self.consume_while(|a| a.is_numeric()).into_iter().collect();
            number.push_str(&num);

            Some(TokenKind::Literal(Literal::Float(
                number.parse::<f64>().unwrap(),
            )))
        } else {
            Some(TokenKind::Literal(Literal::Int(
                number.parse::<i64>().unwrap(),
            )))
        }
    }

    fn populate_keyword(&mut self) {
        self.keywords = map! {
            $(
                Intern::new(String::from(x)) => TokenKind::Keyword(Intern::new(String::from(x))),
            )*
        }
    }
}

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

    //One character tokens
    /// ";"
    SemiColon,
    /// ","
    Comma,
    /// "."
    Dot,
    /// "("
    LParen,
    /// ")"
    RParen,
    /// "{"
    LBrace,
    /// "}"
    RBrace,
    /// "["
    LBracket,
    /// "]"
    RBracket,
    /// "@"
    At,
    /// "#"
    HashTag,
    /// "~"
    Tilde,
    /// "?"
    Question,
    /// ":"
    Colon,
    /// "$"
    Dollar,
    /// "="
    Eq,
    /// "!"
    Bang,
    /// "<"
    LAngle,
    /// ">"
    RAngle,
    /// "-"
    Minus,
    /// "&"
    Ampersand,
    /// "|"
    Pipe,
    /// "+"
    Plus,
    /// "*"
    Star,
    /// "/"
    Slash,
    /// "^"
    Caret,
    /// "%"
    Percent,
    /// "_"
    Underscore,
    /// "\"
    Backslash,

    //Two character tokens
    /// "=>"
    FatArrow,
    /// "<-"
    LArrow,
    /// "->"
    RArrow,
    /// "::"
    DoubleColon,
    /// ".."
    DoubleDot,
    /// "!="
    NEq,
    /// "<="
    LtEq,
    /// ">="
    GtEq,
    /// "=="
    DoubleEq,

    //Others
    /// Represents an unknown character (not supported by the current Lexer)
    UnknownChar(char),

    /// Start Of File
    SoI,
    /// End Of File
    EoI,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Literal::*;
        match self {
            Int(i) => write!(f, "{}", i),
            Float(fl) => write!(f, "{}", fl),
            Bool(b) => write!(f, "{}", b),
            Identifier(i) => write!(f, "{}", i),
            StringLiteral(s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenKind::*;
        match self {
            Keyword(k) => write!(f, "Keyword: {}", k),
            Literal(l) => write!(f, "Literal: {}", l),
            UnknownChar(c) => write!(f, "UnknownChar: {}", c),
            _ => write!(f, "{:?}", self),
        }
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
        }
    }
    fn recoverable(&self) -> bool {
        match self {
            LexError::UnknownCharacter { recoverable, .. } => *recoverable,
            LexError::UnexpectedEndOfInput { recoverable, .. } => *recoverable,
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
        }
    }
}

impl Spanned for LexError {
    fn span(&self) -> Span {
        match self {
            LexError::UnknownCharacter { span, .. } => *span,
            LexError::UnexpectedEndOfInput { span, .. } => *span,
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
