//! The traits given in the project 1 instructions

/// Trait for lolcompiler front end
pub trait Compiler {
    /// Begin the comp
    /// Takes the whole source file and runs it through every step
    fn compile(&mut self, source: &str);

    /// Get the next token from lex
    fn next_token(&mut self) -> String;

    /// Run the syntax
    /// Starts at the top grammar rule and goes down from there
    fn parse(&mut self);

    /// Get the current token
    fn current_token(&self) -> String;

    /// Set the current token
    fn set_current_token(&mut self, tok: String);
}

/// Trait for the lexer, works one character at a time
pub trait LexicalAnalyzer {
    /// Return the next character from input
    fn get_char(&mut self) -> char;

    /// Add a character to the current
    fn add_char(&mut self, c: char);

    /// Lookup a token (return true if valid)
    fn lookup(&self, s: &str) -> bool;
}

/// Syntax
/// One function for each grammar rule. They all give back an error message if the rule isn't followed
pub trait SyntaxAnalyzer {
    /// The whole program, `#HAI` to `#KBYE`
    fn parse_lolcode(&mut self) -> Result<(), String>;
    /// The head block that holds the title
    fn parse_head(&mut self) -> Result<(), String>;
    /// The title inside the head
    fn parse_title(&mut self) -> Result<(), String>;
    /// A comment, `#OBTW` to `#TLDR`
    fn parse_comment(&mut self) -> Result<(), String>;
    /// Everything in the page that isn't the head
    fn parse_body(&mut self) -> Result<(), String>;
    /// A paragraph block
    fn parse_paragraph(&mut self) -> Result<(), String>;
    /// The stuff inside a paragraph
    fn parse_inner_paragraph(&mut self) -> Result<(), String>;
    /// One piece of text or annotation that can go in the body
    fn parse_inner_text(&mut self) -> Result<(), String>;
    /// Making a variable with `#IHAZ`
    fn parse_variable_define(&mut self) -> Result<(), String>;
    /// Using a variable with `#GIMMEH VAR`
    fn parse_variable_use(&mut self) -> Result<(), String>;
    /// Bold text
    fn parse_bold(&mut self) -> Result<(), String>;
    /// A list block
    fn parse_list(&mut self) -> Result<(), String>;
    /// All the items in a list
    fn parse_list_items(&mut self) -> Result<(), String>;
    /// The stuff inside one list item
    fn parse_inner_list(&mut self) -> Result<(), String>;
    /// A web link
    fn parse_link(&mut self) -> Result<(), String>;
    /// A line break
    fn parse_newline(&mut self) -> Result<(), String>;
    /// Plain text
    fn parse_text(&mut self) -> Result<(), String>;
}
