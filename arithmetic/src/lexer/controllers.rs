use super::Lexer;

impl<'a> Lexer<'a> {
    /// Advances to the next position also returns the current char before moving the position
    pub fn advance(&mut self) {
        if self.current_pos >= self.input.len() {
            return;
        }
        let c = self.current_char();
        self.current_pos += 1;

        if c == Some('\n') {
            self.line += 1;
        }
    }

    /// Returns Current Char
    pub fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }

    /// Returns the next char without moving the position of the lexer
    pub fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos + 1)
    }

    /// resets the lexer position so the input can be lexed again without the need to re-initialize
    pub fn reset(&mut self) {
        self.current_pos = 0;
        self.line = 1;
    }

    /// getter function. used by other modules
    /// alternative -> in the lexer we change the field to pub current_pos
    pub fn get_position(&self) -> usize {
        self.current_pos
    }

    /// getter function. used by other modules
    /// alternative -> in the lexer we change the field to pub line
    pub fn get_line(&self) -> usize {
        self.line
    }
}
