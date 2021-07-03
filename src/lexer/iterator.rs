use super::{token, Lexer};

pub struct LexerIntoIterator<'raw> {
    lexer: Lexer<'raw>,
}

impl<'raw> IntoIterator for Lexer<'raw> {
    type Item = token::Token;
    type IntoIter = LexerIntoIterator<'raw>;

    fn into_iter(self) -> Self::IntoIter {
        LexerIntoIterator { lexer: self }
    }
}

impl<'raw> Iterator for LexerIntoIterator<'raw> {
    type Item = token::Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next_token()
    }
}
