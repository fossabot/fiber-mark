use crate::lexer::Lexer;

impl<'raw> Lexer<'raw> {
    pub fn from_string(str: &'raw String) -> Self {
        Lexer::new(str.as_str())
    }
}