#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextSpan {
    pub start: usize,
    pub end: usize,
    pub literal: String,
    pub line: usize,
    pub column: usize,
}

impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String, line: usize, column: usize) -> Self {
        Self {
            start,
            end,
            literal,
            line,
            column,
        }
    }
}
