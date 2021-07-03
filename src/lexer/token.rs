use std::fmt::{Display, Formatter};
use std::ops::Range;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum NewLineType {
    /// `\r\n`
    WindowsStyle,

    /// `\n`
    UnixStyle,
}

impl NewLineType {
    pub fn len(&self) -> usize {
        match self {
            NewLineType::WindowsStyle => 2,
            NewLineType::UnixStyle => 1,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TokenContent {
    Breaks,
    Heading(usize),
    CodeFence(usize),
    Text,
    NewLine(NewLineType),
}

impl TokenContent {
    #[allow(dead_code)]
    pub(crate) fn is_single_special_char(ch: &char) -> bool {
        ['#', '-', '`', '\r', '\n'].contains(&ch)
    }

    #[allow(dead_code)]
    pub(crate) fn single_char_type(ch: &char) -> TokenContent {
        match ch {
            '#' => TokenContent::Heading(1).into(),
            '-' => TokenContent::Breaks.into(),
            '`' => TokenContent::CodeFence(1).into(),
            '\r' => TokenContent::NewLine(NewLineType::WindowsStyle).into(),
            '\n' => TokenContent::NewLine(NewLineType::UnixStyle).into(),
            _ => TokenContent::Text,
        }
    }

    pub(super) fn should_continue(&self, temp_char: &char) -> Option<Self> {
        match self {
            TokenContent::Breaks if temp_char == &'-' => TokenContent::Breaks.into(),
            TokenContent::Heading(level) if temp_char == &'#' => TokenContent::Heading(level + 1).into(),
            TokenContent::CodeFence(level) if temp_char == &'`' => TokenContent::CodeFence(level + 1).into(),
            TokenContent::Text => {
                if !TokenContent::is_single_special_char(temp_char) {
                    TokenContent::Text.into()
                } else {
                    None
                }
            }
            TokenContent::NewLine(new_line) => {
                if new_line == &NewLineType::WindowsStyle && temp_char == &'\n' {
                    TokenContent::NewLine(NewLineType::WindowsStyle).into()
                } else {
                    None
                }
            }
            _ => { None }
        }
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
