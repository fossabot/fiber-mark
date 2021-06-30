use crate::lexer::token;
use std::{borrow::BorrowMut};
use crate::utils::next_while::NextWhile;

impl<'raw> super::Lexer<'raw> {
    pub(crate) fn read_breaks(&mut self) -> Option<token::Token> {
        let start = self.cur_position;
        let source = self.source.borrow_mut();

        let slash_sequence_count = 1 + source.next_while_count(|&(_, chr)| chr == '-');

        if slash_sequence_count < 3 {
            None
        } else {
            Some(token::Token {
                content: token::TokenContent::Breaks,
                range: (start..start + slash_sequence_count),
            })
        }
    }

    pub(crate) fn read_code_block(&mut self) -> Option<token::Token> {
        let start = self.cur_position;
        let source = self.source.borrow_mut();


        let mark_count = 1 + source.next_while_count(|&(_, chr)| chr == '`');

        if mark_count < 3 {
            None
        } else {
            Some(token::Token {
                content: token::TokenContent::Code,
                range: (start..start + mark_count),
            })
        }
    }

    pub(crate) fn read_header(&mut self) -> Option<token::Token> {
        let start = self.cur_position;
        let source = self.source.borrow_mut();

        let level = 1 + source.next_while_count(|&(_, chr)| chr == '#');
        let content = token::TokenContent::Heading(level as u8);

        let token = token::Token {
            content,
            range: (start..start + level),
        };

        Some(token)
    }
}


