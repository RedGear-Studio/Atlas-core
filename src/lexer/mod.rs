/// TODO
pub mod lexer_state;
/// Lay the foundation for the new lexer
#[macro_export]
macro_rules! hehe {
    () => {
        use crate::{lexer::lexer_state::LexerState, utils::span::*};
        use internment::Intern;
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
                lexer.add_system(default_number);
                //.add_system(default_symbol);
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
                            let state = LexerState::new(self.current_pos, self.source.as_str());
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
            todo!()
        }
    };
}
//crate::tmp_symbols!();

/// Temporary symbol macro
#[macro_export]
macro_rules! tmp_symbols {
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
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn hehe() {
        crate::hehe!();
        crate::tmp_symbols!();
        let mut lexer = AtlasLexer::default();
        lexer
            .set_path("<stdin>")
            .set_source(String::from("256245.325"));
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
