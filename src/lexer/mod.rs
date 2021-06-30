mod blocks;
mod iterator;
mod token;

use std::iter::{Enumerate, Peekable};
use std::str::Chars;

use crate::constants;
use std::ops::Range;

pub struct Lexer<'raw> {
    raw: &'raw str,
    source: Peekable<Enumerate<Chars<'raw>>>,
    cur_position: usize,

    // lexer state
    // TODO: May use Cow of Range to avoid cloning.
    plain_text_range: Option<Range<usize>>,
    buffer: String,
}

impl<'raw> Lexer<'raw> {
    pub fn new(str: &'raw str) -> Self {
        Lexer {
            raw: str,
            source: str.chars().enumerate().peekable(),
            cur_position: 0,
            plain_text_range: None,
            buffer: String::with_capacity(16),
        }
    }

    pub fn next_token(&mut self) -> Option<token::Token> {
        let len = self.raw.len();
        let mut result: Option<token::Token> = None;

        while result.is_none() && self.cur_position + 1 < len {
            if !self.buffer.is_empty() {
                match self.buffer.as_str() {
                    constants::LINE_ENDING => {
                        let idx = self.cur_position;
                        self.clear_buffer();

                        return Some(token::Token {
                            content: token::TokenContent::EOL,
                            range: (idx..idx + 1),
                        });
                    }
                    _ => {}
                }
            }

            result = match self.source.next() {
                Some((idx, next)) => {
                    self.cur_position = idx;
                    match next {
                        '#' => self.read_header(),
                        '-' => self.read_breaks(),
                        '`' => self.read_code_block(),
                        _ => self.record_buffer(next, |buffer, plain_text_range| match buffer {
                            constants::LINE_ENDING => {
                                match plain_text_range {
                                    Some(plain_text_range) => {
                                        Some(token::Token {
                                            content: token::TokenContent::Text,
                                            // TODO: May be use Cow
                                            range: plain_text_range.clone(),
                                        })
                                    }
                                    _ => None,
                                }
                            }
                            _ => {
                                if let Some(range) = plain_text_range {
                                    if range.end == len {
                                        Some(token::Token {
                                            content: token::TokenContent::Text,
                                            range: range.clone(),
                                        })
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                        }),
                    }
                }
                None => None,
            };
        }

        result
    }

    fn handled_special_char(&mut self) {
        self.plain_text_range = None;
    }

    fn record_buffer<C>(&mut self, char: char, closure: C) -> Option<token::Token>
    where
        C: Fn(&str, Option<&Range<usize>>) -> Option<token::Token>,
    {
        match char {
            '\r' | '\n' => {
                if self.buffer.is_empty() {
                    self.continue_plain_text();
                }
                self.buffer.push(char);
            }
            _ => {
                if !self.buffer.is_empty() {
                    self.buffer.push(char)
                } else {
                    self.record_plain_text();
                }
            }
        }

        if let Some(result) = closure(self.buffer.as_str(), Option::from(&self.plain_text_range)) {
            self.handled_special_char();
            return Some(result);
        }

        None
    }

    fn clear_buffer(&mut self) {
        self.buffer.clear();
    }

    fn continue_plain_text(&mut self) {
        let idx = self.cur_position;
        match &self.plain_text_range {
            Some(range) => {
                self.plain_text_range = Some(range.start..idx);
            }
            _ => {}
        }
    }

    fn record_plain_text(&mut self) {
        let idx = self.cur_position;
        match &self.plain_text_range {
            Some(range) => {
                self.plain_text_range = Some(range.start..idx + 1);
            }
            None => {
                self.plain_text_range = Some(idx..idx + 1);
            }
        }
    }

    #[allow(dead_code)]
    pub(super) fn is_last(&self) -> bool {
        self.cur_position == self.raw.len() - 1
    }

    #[allow(dead_code)]
    pub(super) fn next_idx(&self) -> Option<usize> {
        let idx = self.cur_position;
        if self.is_last() {
            Some(idx + 1)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;

    #[test]
    fn basics() {
        let md_str = r#"
# Title of Markdown
## Sub Title

```
code block here
```

---

Hello World
        "#
        .trim();

        let mut lexer = Lexer::new(md_str);

        while let Some(token) = lexer.next() {
            let content = md_str[token.range.clone()].to_string();
            println!(
                "{}, \n content: \"{}\"",
                token,
                if content == "\n" {
                    "\\n".to_string()
                } else {
                    content
                }
            );
        }
    }
}
