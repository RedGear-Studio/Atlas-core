use std::vec;

use crate::prelude::BytePos;

pub struct TestLexer {
    sys: Vec<fn(char, &mut LexerState) -> Option<f32>>,
    _path: &'static str,
    pub current_pos: BytePos,
    pub it: Vec<char>,
    borrow: bool,
}

#[derive(Clone, Copy)]
pub struct LexerState<'lex> {
    pub current_pos: BytePos,
    pub it: &'lex[char],
    borrow: bool    
}
impl<'lex> LexerState<'lex> {
    pub fn new(current_pos: BytePos, it: &'lex[char], borrow: bool) -> Self {
        Self {
            current_pos,
            it,
            borrow
        }
    }
    pub fn next(&mut self) -> Option<char> {
        match self.it.get(<BytePos as Into<usize>>::into(self.current_pos) + 1) {
            Some(c) => {
                self.borrow = true;
                Some(*c)
            },
            None => None
        }
    }
    pub fn peek(&self) -> Option<&char> {
        self.it.get(<BytePos as Into<usize>>::into(self.current_pos) + 1)
    }
    pub fn keep(&mut self) {
        if self.borrow {
            self.borrow = false;
            self.current_pos = BytePos::from(<BytePos as Into<usize>>::into(self.current_pos)+1);
        }
    }
}


impl TestLexer {
    pub fn new() -> Self {
        println!("Hello");
        Self {
            sys: vec![],
            _path: "<stdin>",
            current_pos: BytePos::from(0),
            it: vec!['5', '3', '6', '.', '1'],
            borrow: false
        }
    }

    pub fn add_system(&mut self, s: fn(char, &mut LexerState) -> Option<f32>) {
        println!("added system");
        self.sys.push(s);
    }

    pub fn tokenize(&mut self) -> Option<f32> {
        loop {
            let ch = self.it.get(<BytePos as Into<usize>>::into(self.current_pos));
            match ch {
                Some(c) => {
                    let state = LexerState::new(self.current_pos, self.it.as_slice(), self.borrow);
                    for f in &self.sys {
                        println!("call a fn");
                        match f(*c, &mut state.clone()) {
                            Some(f) => {
                                println!("{}", f);
                                return Some(f)
                            }
                            None => {
                                continue
                            }
                        }
                    }
                }
                None => break
            }
        }
        None
    }

}

fn number(c: char, state: &mut LexerState) -> Option<f32> {
    println!("entered number()");
    if c.is_numeric() {
        let mut n = String::new();
        n.push(c);
        state.keep();
        loop {
            println!("{}", n);
            if let Some(c) = state.next() {
                if c.is_numeric() {
                    state.keep();
                    n.push(c);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        if let Some(c) = state.next() {
            if c == '.' {
                state.keep();
                n.push(c);
                loop {
                    if let Some(c) = state.next() {
                        if c.is_numeric() {
                            state.keep();
                            n.push(c);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        Some(n.parse().unwrap())
    } else {
        None
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test_lexer() {
        use crate::lexer::test_lexer::*;
        let mut lex = TestLexer::new();
        lex.add_system(number);
        assert_eq!(lex.tokenize(), Some(536.1));
    }
}