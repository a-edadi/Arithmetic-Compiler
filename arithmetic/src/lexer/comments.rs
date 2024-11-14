use super::Lexer;

/// Exclude the rest of the line when // is seen
impl<'a> Lexer<'a> {
    pub fn handle_line_comment(&mut self) {
        while let Some(c) = self.current_char() {
            if c == '\n' {
                break;
            }
            self.advance();
        }
    }

    // handle block comments
    pub fn handle_block_comment(&mut self) {
        self.advance(); // Skip the initial '{'
        while let Some(c) = self.current_char() {
            if c == '}' {
                self.advance(); // Skip the closing '}'
                break;
            }
            self.advance();
        }
    }
}
