mod blocks;
mod token;
mod iterator;

use std::{iter::Enumerate};
use std::str::Chars;

pub struct Lexer<'raw> {
    raw: &'raw str,
    source: Enumerate<Chars<'raw>>,
    cur_position: usize,
}

impl<'raw> Lexer<'raw> {
    pub fn new(str: &'raw str) -> Self {
        Lexer {
            raw: str,
            source: str.chars().enumerate(),
            cur_position: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<token::Token> {
        let mut result: Option<token::Token> = None;

        while result.is_none() && self.cur_position + 1 < self.raw.len() {
            result = match self.source.next() {
                Some((idx, next)) => {
                    self.cur_position = idx;
                    match next {
                        '#' => self.read_header(),
                        '-' => self.read_breaks(),
                        '`' => self.read_code_block(),
                        _ => { None }
                    }
                }
                None => {
                    None
                }
            };
        }

        result
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
        "#.trim();

        let mut lexer = Lexer::new(md_str);

        while let Some(token) = lexer.next() {
            println!("{:?}", token);
        }
    }
}
