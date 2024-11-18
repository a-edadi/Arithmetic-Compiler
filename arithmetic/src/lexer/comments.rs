use super::Lexer;

// Implementation block for handling comments.
impl<'a> Lexer<'a> {
    /// Double slash comments
    pub fn handle_line_comment(&mut self) {
        while let Some(c) = self.current_char() {
            if c == '\n' {
                break;
            }
            self.advance();
        }
    }

    // Block comments {}
    pub fn handle_block_comment(&mut self) {
        self.advance();
        while let Some(c) = self.current_char() {
            if c == '}' {
                self.advance();
                break;
            }
            self.advance();
        }
    }
}
