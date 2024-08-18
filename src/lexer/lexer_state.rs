use std::{iter::Peekable, str::Chars};

use crate::prelude::BytePos;

#[derive(Clone)]
pub struct LexerState<'lex> {
    pub current_pos: BytePos,
    txt: Peekable<Chars<'lex>>,
}

impl<'lex> LexerState<'lex> {
    pub fn new(current_pos: BytePos, txt: &'lex str) -> Self {
        Self {
            current_pos,
            txt: txt.chars().peekable(),
        }
    }
    pub fn next(&mut self) -> Option<char> {
        self.current_pos = self.current_pos.shift_by(1);
        self.txt.next()
    }
    pub fn peek(&mut self) -> Option<&char> {
        self.txt.peek()
    }
}
