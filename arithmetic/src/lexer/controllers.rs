use super::Lexer;

impl<'a> Lexer<'a> {
    /// Advances to the next position also returns the current char before moving the position
    pub fn advance(&mut self) {
        if self.pos >= self.input.len() {
            return;
        }
        let c = self.current_char();
        self.pos += 1;

        if c == Some('\n') {
            self.line += 1;
            self.column = 0
        } else {
            self.column += 1
        }
    }

    /// Returns Current Char
    pub fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    /// Returns the next char without moving the position of the lexer
    pub fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.pos + 1)
    }

    /// resets the lexer position so the input can be lexed again without the need to re-initialize
    pub fn reset(&mut self) {
        self.pos = 0;
        self.line = 1;
        self.column = 0;
    }
}
