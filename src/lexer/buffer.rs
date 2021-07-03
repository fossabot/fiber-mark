use crate::lexer::token;
use crate::lexer::Lexer;

impl<'raw> Lexer<'raw> {
    pub(super) fn handle_buffer(&mut self) -> Option<token::Token> {
        if let Some(buf_type) = &self.buffer_type {
            match buf_type {
                token::TokenContent::Heading(level) => token::Token {
                    content: buf_type.clone(),
                    range: (self.cur_position - level..self.cur_position),
                }
                .into(),
                token::TokenContent::Breaks if self.ensure_buf_minimum_len(3) => token::Token {
                    content: buf_type.clone(),
                    range: (self.cur_position - self.buffer.len()..self.cur_position),
                }
                .into(),
                token::TokenContent::CodeFence(level) if self.ensure_buf_minimum_len(3) => {
                    token::Token {
                        content: buf_type.clone(),
                        range: (self.cur_position - level..self.cur_position),
                    }
                    .into()
                }
                token::TokenContent::Text => token::Token {
                    content: buf_type.clone(),
                    range: (self.cur_position - self.buffer.len()..self.cur_position),
                }
                .into(),
                token::TokenContent::NewLine(eol) => token::Token {
                    content: buf_type.clone(),
                    range: (self.cur_position - eol.len()..self.cur_position),
                }
                .into(),
                _ => None,
            }
        } else {
            None
        }
    }

    fn ensure_buf_minimum_len(&self, len: usize) -> bool {
        self.buffer.len() >= len
    }
}
