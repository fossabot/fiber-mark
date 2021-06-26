use super::{Lexer, token};

impl<'raw> Iterator for Lexer<'raw> {
    type Item = token::Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}