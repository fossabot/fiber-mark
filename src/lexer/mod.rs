mod constructors;
mod iterator;
mod test_util;
mod token;

use std::iter::{Enumerate, Peekable};
use std::str::Chars;

pub struct Lexer<'raw> {
    source: Peekable<Enumerate<Chars<'raw>>>,

    /// The current position isn't always the available index of raw string.
    /// For example, if the `source` has been consumed completely, the `cur_position` will be
    /// `len(source string) + 1`.
    cur_position: usize,

    /// Some temp special char sequence while handling.
    buffer: String,
    buffer_type: Option<token::TokenContent>,
}

impl<'raw> Lexer<'raw> {
    pub fn new(str: &'raw str) -> Self {
        Lexer {
            source: str.chars().enumerate().peekable(),
            cur_position: 0,
            buffer: String::with_capacity(16),
            buffer_type: None,
        }
    }

    pub fn next_token(&mut self) -> Option<token::Token> {
        let mut result: Option<token::Token> = None;

        while result.is_none() {
            if let Some(current_item) = self.source.next() {
                let (idx, char) = current_item;
                self.cur_position = idx;

                if let Some(tkn_content) = self.read_buf_type(&char) {
                    // update buffer
                    self.buffer.push(char);
                    self.buffer_type = tkn_content.into();
                } else {
                    // verify buffer
                    if let Some(buf_tkn) = self.handle_buffer() {
                        result = buf_tkn.into();
                        self.clear_buffer();
                    }

                    // update buffer
                    self.buffer.push(char);
                    self.buffer_type = self.read_buf_type(&char);
                    debug_assert!(
                        self.buffer_type.is_some(),
                        "Lexer's buffer type is none after clearing buffer"
                    );
                }
            } else {
                // verify rest buffer
                self.cur_position += 1;
                if let Some(buf_tkn) = self.handle_buffer() {
                    result = buf_tkn.into();
                    self.clear_buffer();
                }

                // raw string has been handled completely.
                break;
            }
        }

        result
    }

    fn handle_buffer(&mut self) -> Option<token::Token> {
        if let Some(buf_type) = &self.buffer_type {
            match buf_type {
                token::TokenContent::Heading(level) => token::Token {
                    content: buf_type.clone(),
                    range: (self.cur_position - level..self.cur_position),
                },
                token::TokenContent::Breaks => token::Token {
                    content: buf_type.clone(),
                    range: (self.cur_position - self.buffer.len()..self.cur_position),
                },
                token::TokenContent::CodeFence(level) => token::Token {
                    content: buf_type.clone(),
                    range: (self.cur_position - level..self.cur_position),
                },
                token::TokenContent::Text => token::Token {
                    content: buf_type.clone(),
                    range: (self.cur_position - self.buffer.len()..self.cur_position),
                },
                token::TokenContent::NewLine(eol) => token::Token {
                    content: buf_type.clone(),
                    range: (self.cur_position - eol.len()..self.cur_position),
                },
            }
            .into()
        } else {
            None
        }
    }

    fn read_buf_type(&self, char: &char) -> Option<token::TokenContent> {
        if let Some(buf_tkn) = &self.buffer_type {
            buf_tkn.should_continue(char)
        } else {
            token::TokenContent::single_char_type(char).into()
        }
    }

    fn clear_buffer(&mut self) {
        if self.buffer.is_empty() {
            return;
        }

        self.buffer.clear();
        self.buffer_type = None;
    }
}

#[cfg(test)]
mod tests {
    use super::token::Token;
    use super::Lexer;
    use crate::lexer::test_util::read_md_file;
    use crate::lexer::token::NewLineType::UnixStyle;
    use crate::lexer::token::TokenContent::{Breaks, CodeFence, Heading, NewLine, Text};

    #[test]
    fn print_md_file_token_stream() {
        let md_str = read_md_file("./petite-vue.md");
        let lexer = Lexer::from_string(&md_str);
        for item in lexer {
            println!("{}", item);
        }
    }

    #[test]
    fn by_real_md_file() {
        let md_str = read_md_file("./test.md");

        let lexer = Lexer::from_string(&md_str);

        let result: Vec<Token> = lexer.into_iter().collect();
        let expected = vec![
            Token {
                content: Heading(1),
                range: 0..1,
            },
            Token {
                content: Text,
                range: 1..19,
            },
            Token {
                content: NewLine(UnixStyle),
                range: 19..20,
            },
            Token {
                content: Heading(2),
                range: 20..22,
            },
            Token {
                content: Text,
                range: 22..32,
            },
            Token {
                content: NewLine(UnixStyle),
                range: 32..33,
            },
            Token {
                content: NewLine(UnixStyle),
                range: 33..34,
            },
            Token {
                content: CodeFence(3),
                range: 34..37,
            },
            Token {
                content: NewLine(UnixStyle),
                range: 37..38,
            },
            Token {
                content: Text,
                range: 38..53,
            },
            Token {
                content: NewLine(UnixStyle),
                range: 53..54,
            },
            Token {
                content: CodeFence(3),
                range: 54..57,
            },
            Token {
                content: NewLine(UnixStyle),
                range: 57..58,
            },
            Token {
                content: NewLine(UnixStyle),
                range: 58..59,
            },
            Token {
                content: Breaks,
                range: 59..62,
            },
            Token {
                content: NewLine(UnixStyle),
                range: 62..63,
            },
            Token {
                content: NewLine(UnixStyle),
                range: 63..64,
            },
            Token {
                content: Text,
                range: 64..75,
            },
        ];

        assert_eq!(expected, result);
    }
}
