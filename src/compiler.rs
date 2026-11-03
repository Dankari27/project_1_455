//! The main compiler front end

use crate::lexer::Lexer;
use crate::semantic;
use crate::traits::{Compiler, SyntaxAnalyzer};

/// This holds stuff the compiler needs to function
pub struct LolCompiler {
    /// converts to tokens
    pub lexer: Lexer,
    /// EOF
    current: String,
    /// current tokens start
    current_line: usize,
    current_column: usize,
    /// accepted token
    pub parse_tree: Vec<(String, usize)>,
    /// finished html
    pub html: String,
}

impl LolCompiler {
    /// Empty compiler instance
    /// Lexer starts with no source, it gets the actual one in try_compile
    pub fn new() -> Self {
        LolCompiler {
            lexer: Lexer::new(""),
            current: String::new(),
            current_line: 1,
            current_column: 1,
            parse_tree: Vec::new(),
            html: String::new(),
        }
    }

    /// Hands back
    /// the html if it worked, or the first error message (lexical, syntax or semantic) if it didn't
    pub fn try_compile(&mut self, source: &str) -> Result<String, String> {
        self.lexer = Lexer::new(source);
        self.parse_tree.clear();
        // Need the first token loaded before parsing can start
        self.advance_next_token()?;
        self.parse_lolcode()?;
        semantic::generate_html(&self.parse_tree)
    }

    /// Moves to next token
    /// Also saves where the token started so errors can say the line and column
    pub fn advance_next_token(&mut self) -> Result<(), String> {
        let token = self.lexer.scan_token()?;
        self.current_line = self.lexer.token_line;
        self.current_column = self.lexer.token_column;
        self.set_current_token(token);
        Ok(())
    }

    /// Accepts the current token into parse tree
    /// Stores it with its line number and then moves on to the next one
    pub fn accept(&mut self) -> Result<(), String> {
        self.parse_tree.push((self.current.clone(), self.current_line));
        self.advance_next_token()
    }

    /// Accepts the current token if it matches grammar
    /// If it doesn't match it's a syntax error
    pub fn expect(&mut self, expected: &str) -> Result<(), String> {
        if self.current == expected {
            self.accept()
        } else {
            Err(self.syntax_error(expected))
        }
    }

    /// True if token is plain text
    /// Annotations start with # and an empty token means end of file, so neither counts
    pub fn is_text(&self) -> bool {
        !self.current.is_empty() && !self.current.starts_with('#')
    }

    /// Error message
    /// Builds the syntax error with the line, column, what we wanted and what we got instead
    pub fn syntax_error(&self, expected: &str) -> String {
        // Empty token means we ran out of file
        let found = if self.current.is_empty() {
            "the end of the file".to_string()
        } else {
            format!("'{}'", self.current)
        };
        format!(
            "Syntax error at line {}, column {}: expected {}, but found {}.",
            self.current_line, self.current_column, expected, found
        )
    }
}

impl Compiler for LolCompiler {
    /// Compiles source
    /// Saves the html if it worked, otherwise prints the error and exits
    fn compile(&mut self, source: &str) {
        self.html = self.try_compile(source).unwrap_or_else(|message| {
            eprintln!("{}", message);
            std::process::exit(1);
        });
    }

    /// Gets next token from lexer
    /// Exits if the lexer finds something that isn't allowed
    fn next_token(&mut self) -> String {
        self.lexer.scan_token().unwrap_or_else(|message| {
            eprintln!("{}", message);
            std::process::exit(1);
        })
    }

    /// Parse program
    /// Exits on the first syntax error
    fn parse(&mut self) {
        if let Err(message) = self.parse_lolcode() {
            eprintln!("{}", message);
            std::process::exit(1);
        }
    }

    /// Returns token currently looked at
    fn current_token(&self) -> String {
        self.current.clone()
    }

    /// Replaces current token
    fn set_current_token(&mut self, tok: String) {
        self.current = tok;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tokens should come out in order and then empty at the end
    #[test]
    fn next_token_source() {
        let mut compiler = LolCompiler::new();
        compiler.lexer = Lexer::new("#HAI hello #KBYE");
        assert_eq!(compiler.next_token(), "#HAI");
        assert_eq!(compiler.next_token(), "hello");
        assert_eq!(compiler.next_token(), "#KBYE");
        assert_eq!(compiler.next_token(), "");
    }

    /// Setting a token and reading it back should give the same thing
    #[test]
    fn current_token_setread() {
        let mut compiler = LolCompiler::new();
        compiler.set_current_token("#OIC".to_string());
        assert_eq!(compiler.current_token(), "#OIC");
    }

    /// Compile should save the finished html in the compiler
    #[test]
    fn compile_storehtml() {
        let mut compiler = LolCompiler::new();
        compiler.compile("#HAI hello #KBYE");
        assert_eq!(compiler.html, "<html>\n  hello\n</html>\n");
    }
}
