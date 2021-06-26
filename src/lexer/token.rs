use std::ops::Range;

#[derive(Debug)]
pub struct Token {
    pub content: TokenContent,
    pub range: Range<usize>,
}

#[derive(Debug)]
pub enum TokenContent {
    Breaks,
    Heading(u8),
    Code
}
