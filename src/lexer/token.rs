use std::fmt::{Display, Formatter};
use std::ops::Range;

#[derive(Debug, Eq, PartialEq)]
pub enum TokenContent {
    Breaks,
    Heading(u8),
    Code,
    Text,
    EOL,
}

impl TokenContent {
    pub(crate) fn is_single_special_char(ch: char) -> bool {
        ['#', '-', '`'].contains(&ch)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    pub content: TokenContent,
    pub range: Range<usize>,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[Token] start: {}, end: {}, content: {:?}",
            self.range.start, self.range.end, self.content
        )
    }
}
