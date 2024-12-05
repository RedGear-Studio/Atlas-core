/// TODO
pub mod lexer_state;
/// To be done
#[macro_export]
macro_rules! lexer_builder {
    (
        DefaultSystem {
            number: $number:literal,
            symbol: $symbol:literal,
            keyword: $keyword:literal,
            string: $string:literal,
            whitespace: {
                allow_them: $allow_whitespace:literal,
                use_system: $whitespace:literal$(,)?
            }$(,)?
        },
        Symbols {
            Single {
                $($sym:literal => $variant:ident),* $(,)?
            }, Either {
                $($sym2:literal => $sym3:literal => $variant1:ident, $variant2:ident ),* $(,)?
            }
        },
        Keyword {
            $($x:literal),* $(,)?
        },
        Number {
            trailing {
                $($trail_name:literal => $trail_type:ty => $trail_enum:ident),+ $(,)?
            },
            float: $float:literal,
            u_int: $u_int:literal,
            int: $int:literal $(,)?
        }$(,)?
    ) => {
        tokens!{
            Symbols {
                Single {
                    $($sym => $variant),*
                }, Either {
                    $($sym2 => $sym3 => $variant1, $variant2),*
                }
            },
            Number {$($trail_enum($trail_type),)*}
        }
        keywords!($($x,)*);
        pub type System = fn(char, &mut LexerState) -> Option<Token>;
        #[derive(Debug, Default)]
        pub struct AtlasLexer {
            sys: Vec<System>,
            path: &'static str,
            pub current_pos: BytePos,
            pub source: String,
        }
        impl AtlasLexer {
            pub fn default() -> Self {
                let mut lexer = AtlasLexer::new("<stdin>", String::new());
                if $number {lexer.add_system(default_number);}
                if $symbol {lexer.add_system(default_symbol);}
                if $keyword {lexer.add_system(default_keyword);}
                if $whitespace {lexer.add_system(default_whitespace);}
                if $string {lexer.add_system(default_string);}
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

            pub fn add_system(&mut self, s: fn(char, &mut LexerState) -> Option<Token>) -> &mut Self {
                self.sys.push(s);
                self
            }

            //A way of handling errors will come later
            pub fn tokenize(&mut self) -> Result<Vec<Token>, ()> {
                let mut tok: Vec<Token> = vec![];
                tok.push(Token::new(
                    Span {
                        start: self.current_pos,
                        end: self.current_pos,
                        path: self.path,
                    },
                    TokenKind::SoI,
                ));
                loop {
                    let ch = self.source.chars().nth(usize::from(self.current_pos));
                    match ch {
                        Some(c) => {
                            let state = LexerState::new(
                                self.current_pos,
                                self.source
                                    .get(usize::from(self.current_pos)..self.source.len())
                                    .unwrap(),
                                self.path,
                            );
                            let mut counter = 0;
                            for f in &self.sys {
                                let mut current_state = state.clone();
                                match f(c, &mut current_state) {
                                    Some(f) => {
                                        if !$allow_whitespace {
                                            match f.kind() {
                                                TokenKind::WhiteSpace => {}
                                                TokenKind::CarriageReturn => {}
                                                TokenKind::NewLine => {}
                                                TokenKind::Tabulation => {}
                                                _ => tok.push(f),
                                            }
                                        } else {
                                            tok.push(f);
                                        }
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
                tok.push(Token::new(
                    Span {
                        start: self.current_pos,
                        end: self.current_pos,
                        path: self.path,
                    },
                    TokenKind::EoI,
                ));
                return Ok(tok);
            }
        }
        pub fn default_number(c: char, state: &mut LexerState) -> Option<Token> {
            if c.is_numeric() {
                let start = state.current_pos;
                let mut is_float = false;
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
                if let Some(&'.') = state.peek() {
                        n.push('.');
                        state.next();
                        is_float = true;
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

                Some(Token::new(
                    Span {
                        start,
                        end: state.current_pos,
                        path: state.path,
                    },
                    TokenKind::Literal(if is_float {Literal::Float(n.parse::<f64>().unwrap())} else {Literal::Int(n.parse::<i64>().unwrap())})),
                )
            } else {
                None
            }
        }
        pub fn default_whitespace(c: char, state: &mut LexerState) -> Option<Token> {
            let start = state.current_pos;
            let tok = match c {
                ' ' => TokenKind::WhiteSpace,
                '\t' => TokenKind::Tabulation,
                '\n' => TokenKind::NewLine,
                '\r' => TokenKind::CarriageReturn,
                _ => return None,
            };
            state.next();
            return Some(Token::new(
                Span {
                    start,
                    end: state.current_pos,
                    path: state.path,
                },
                tok,
            ))
        }
        pub fn default_string(c: char, state: &mut LexerState) -> Option<Token> {
            let start = state.current_pos;
            let mut s = String::new();
            if c == '"' {
                println!("string in the making");
                state.next();
                loop {
                    if let Some(ch) = state.peek() {
                        if *ch == '"' {
                            state.next();
                            break;
                        }
                        s.push(*ch);
                        state.next();
                    }
                }
                return Some(Token::new(
                    Span {
                        start,
                        end: state.current_pos,
                        path: state.path,
                    },
                    TokenKind::Literal(Literal::StringLiteral(Intern::new(s))),
                ));
            } else {
                None
            }
        }
        
    };
}

/// To be done
#[macro_export]
macro_rules! tokens {
    (Symbols {
        Single {
            $($sym:literal => $variant:ident),* $(,)?
        }, Either {
            $($sym2:literal =>  $sym3:literal => $variant1:ident, $variant2:ident ),* $(,)?
        }
    }, Number {$($trail_enum:ident($trail_type:ty)),+ $(,)?}) => {
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
            $(
                $trail_enum($trail_type),
            )+
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
            $(
                $variant1,
                $variant2,
            )*
            WhiteSpace,
            NewLine,
            Tabulation,
            CarriageReturn,
            EoI,
            SoI
        }
        //TODO: add support for multi-char symbols
        fn default_symbol(c: char, state: &mut LexerState) -> Option<Token> {
            let start = state.current_pos;
            let tok = match c {
                $(
                    $sym => TokenKind::$variant,
                )*
                $(
                    $sym2 => if let Some(c) = state.peek() {
                        if *c == $sym3 {
                            state.next();
                            TokenKind::$variant1
                        } else {
                            TokenKind::$variant2
                        }
                    } else {
                        TokenKind::$variant2
                    }
                )*
                _ => return None,
            };
            state.next();
            Some(Token::new(
                Span {
                    start,
                    end: state.current_pos,
                    path: state.path,
                },
                tok,
            ))
        }
    };
}
/// To be done
#[macro_export]
macro_rules! keywords {
    ($($x:literal),* $(,)?) => {
        use std::collections::HashMap;
        pub fn default_keyword(c: char, state: &mut LexerState) -> Option<Token> {
            let start = state.current_pos;
            let mut s = String::new();
            if c.is_alphabetic() || c == '_' {
                s.push(c);
                state.next();
                let keywords: HashMap<Intern<String>, TokenKind> = map! {
                    $(
                        Intern::new(String::from($x)) => TokenKind::Keyword(Intern::new(String::from($x))),
                    )*
                };
                loop {
                    if let Some(c) = state.peek() {
                        if c.is_alphabetic() || *c == '_' {
                            s.push(*c);
                            state.next();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                if let Some(k) = keywords.get(&Intern::new(s.clone())) {
                    Some(Token::new(Span {
                        start,
                        end: state.current_pos,
                        path: state.path
                    }, *k))
                } else {
                    return Some(Token::new(Span {
                        start,
                        end:state.current_pos,
                        path: state.path
                    }, TokenKind::Literal(Literal::Identifier(Intern::new(s)))));
                }
            } else {
                None
            }
        }
    };
}

