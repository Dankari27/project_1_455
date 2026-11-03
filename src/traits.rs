//! The traits given in the project 1 instructions

/// Trait for lolcompiler front end
pub trait Compiler {
    /// Begin the comp
    fn compile(&mut self, source: &str);

    /// Get the next token from lex
    fn next_token(&mut self) -> String;

    /// Run the syntax 
    fn parse(&mut self);

    /// Get the current token 
    fn current_token(&self) -> String;

    /// Set the current token 
    fn set_current_token(&mut self, tok: String);
}

pub trait LexicalAnalyzer {
    /// Return the next character from input
    fn get_char(&mut self) -> char;

    /// Add a character to the current
    fn add_char(&mut self, c: char);

    /// Lookup a token (return true if valid)
    fn lookup(&self, s: &str) -> bool;
}

/// Syntax
pub trait SyntaxAnalyzer {
    fn parse_lolcode(&mut self) -> Result<(), String>;
    fn parse_head(&mut self) -> Result<(), String>;
    fn parse_title(&mut self) -> Result<(), String>;
    fn parse_comment(&mut self) -> Result<(), String>;
    fn parse_body(&mut self) -> Result<(), String>;
    fn parse_paragraph(&mut self) -> Result<(), String>;
    fn parse_inner_paragraph(&mut self) -> Result<(), String>;
    fn parse_inner_text(&mut self) -> Result<(), String>;
    fn parse_variable_define(&mut self) -> Result<(), String>;
    fn parse_variable_use(&mut self) -> Result<(), String>;
    fn parse_bold(&mut self) -> Result<(), String>;
    fn parse_list(&mut self) -> Result<(), String>;
    fn parse_list_items(&mut self) -> Result<(), String>;
    fn parse_inner_list(&mut self) -> Result<(), String>;
    fn parse_link(&mut self) -> Result<(), String>;
    fn parse_newline(&mut self) -> Result<(), String>;
    fn parse_text(&mut self) -> Result<(), String>;
}
