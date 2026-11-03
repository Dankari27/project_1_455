//! Reads the source one character at a time and groups the characters into tokens

use crate::traits::LexicalAnalyzer;

/// legal annotation
const ANNOTATIONS: &[&str] = &[
    "#HAI",
    "#KBYE",
    "#OBTW",
    "#TLDR",
    "#MAEK HEAD",
    "#MAEK PARAGRAF",
    "#MAEK LIST",
    "#MKAY",
    "#GIMMEH TITLE",
    "#GIMMEH BOLD",
    "#GIMMEH ITEM",
    "#GIMMEH LINX",
    "#GIMMEH VAR",
    "#OIC",
    "#NEWLINE",
    "#IHAZ",
    "#ITIZ",
];

/// Punctuation inside plain text
const TEXT_PUNCTUATION: &str = ",.\"':?!%=/";

/// Holds source text
pub struct Lexer {

    chars: Vec<char>,

    position: usize,
  
    line: usize,
    column: usize,
  
    lexeme: String,
  
    pub token_line: usize,
    pub token_column: usize,
}

impl Lexer {
    /// Creates a new lexer
    /// Splits the source into characters and starts at line 1, column 1
    pub fn new(source: &str) -> Self {
        Lexer {
            chars: source.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
            lexeme: String::new(),
            token_line: 1,
            token_column: 1,
        }
    }

    /// Looks at new character before moving on
    /// Gives None if we're at the end of the source
    fn peek(&self) -> Option<char> {
        self.chars.get(self.position).copied()
    }

    /// Skips white space and other stuff
    /// Spaces, tabs and newlines between tokens don't matter
    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(char::is_whitespace) {
            self.get_char();
        }
    }

    /// Add letters to lexer until a letter doesn't show up
    /// Used for the word part of an annotation like HAI or BOLD
    fn read_word(&mut self) {
        while self.peek().is_some_and(|c| c.is_ascii_alphabetic()) {
            let c = self.get_char();
            self.add_char(c);
        }
    }

    /// Finds the next token
    /// Gives back an empty string at the end of the file
    pub fn scan_token(&mut self) -> Result<String, String> {
        self.skip_whitespace();
        self.lexeme.clear();
        // Remember where this token starts for error messages
        self.token_line = self.line;
        self.token_column = self.column;

        // # means annotation, anything else is text
        match self.peek() {
            None => Ok(String::new()),
            Some('#') => self.scan_annotation(),
            Some(_) => self.scan_text(),
        }
    }

    /// Reads an annotation such as `#HAI` or `#GIMMEH BOLD`.
    /// Annotations aren't case sensitive so they get turned into uppercase
    fn scan_annotation(&mut self) -> Result<String, String> {
        let hash = self.get_char();
        self.add_char(hash);
        self.read_word();

        // #MAEK and #GIMMEH need the next word too 
        if matches!(self.lexeme.to_uppercase().as_str(), "#MAEK" | "#GIMMEH") {
            self.skip_whitespace();
            self.add_char(' ');
            self.read_word();
        }

        if self.lookup(&self.lexeme) {
            Ok(self.lexeme.to_uppercase())
        } else {
            Err("lexical error: not a valid annotation".to_string())
        }
    }

    /// Reads plain text until the next # or the end of the file
    /// Extra space at the end gets cut off
    fn scan_text(&mut self) -> Result<String, String> {
        while self.peek().is_some_and(|c| c != '#') {
            let c = self.get_char();
            self.add_char(c);
        }

        let text = self.lexeme.trim_end().to_string();
        if self.lookup(&text) {
            Ok(text)
        } else {
            // Find out the first character that isn't allowed
            let bad = text.chars().find(|c| !is_text_char(*c)).unwrap_or(' ');
            Err(format!(
                "lexical error: character '{}' is not allowed in text",
                bad
            ))
        }
    }
}

/// True if the character is allowed inside text
/// Letters, numbers, whitespace and the punctuation list up top
fn is_text_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, ' ' | '\t' | '\n' | '\r') || TEXT_PUNCTUATION.contains(c)
}

impl LexicalAnalyzer for Lexer {
    /// Returns the next character and then continues
    /// Keeps track of the line and column as it goes
    fn get_char(&mut self) -> char {
        let Some(c) = self.peek() else {
            eprintln!("lexical error: unexpected end of file.");
            std::process::exit(1);
        };
        self.position += 1;
        // Newline means new line and back to column 1
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        c
    }

    /// Adds one character to the token being built
    /// The token gets cleared at the start of each scan_token
    fn add_char(&mut self, c: char) {
        self.lexeme.push(c);
    }

    /// Annotations must be in the list above and text may only use allowed characters
    /// Empty text doesn't count as valid
    fn lookup(&self, s: &str) -> bool {
        if s.starts_with('#') {
            ANNOTATIONS.contains(&s.to_uppercase().as_str())
        } else {
            !s.is_empty() && s.chars().all(is_text_char)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Collects every token in the source (stops at the end or the first error)
    fn all_tokens(source: &str) -> Result<Vec<String>, String> {
        let mut lexer = Lexer::new(source);
        let mut tokens = Vec::new();
        loop {
            let token = lexer.scan_token()?;
            if token.is_empty() {
                return Ok(tokens);
            }
            tokens.push(token);
        }
    }

    /// Real annotations should pass in any case
    #[test]
    fn lookup_accept() {
        let lexer = Lexer::new("");
        assert_eq!(lexer.lookup("#HAI"), true);
        assert_eq!(lexer.lookup("#hai"), true);
        assert_eq!(lexer.lookup("#GiMmEh BoLd"), true);
        assert_eq!(lexer.lookup("#KBYE"), true);
    }

    /// Made up annotations or half of a two word one should fail
    #[test]
    fn lookup_reject() {
        let lexer = Lexer::new("");
        assert_eq!(lexer.lookup("#HEY"), false);
        assert_eq!(lexer.lookup("#HEAD"), false);
        assert_eq!(lexer.lookup("#MAEK"), false);
        assert_eq!(lexer.lookup("#GIMMEH HEAD"), false);
    }

    /// Text with allowed characters passes, bad characters or empty text fails
    #[test]
    fn lookup_check() {
        let lexer = Lexer::new("");
        assert_eq!(lexer.lookup("Hello, world!"), true);
        assert_eq!(lexer.lookup("www.towson.edu/a?b=1"), true);
        assert_eq!(lexer.lookup("a < b"), false);
        assert_eq!(lexer.lookup(""), false);
    }

    /// get_char should go in order and add_char should build up the lexeme
    #[test]
    fn getchar_build_lexeme() {
        let mut lexer = Lexer::new("ab");
        assert_eq!(lexer.get_char(), 'a');
        assert_eq!(lexer.get_char(), 'b');
        lexer.add_char('x');
        lexer.add_char('y');
        assert_eq!(lexer.lexeme, "xy");
    }

    /// Annotations and text should be split up right, even with weird spacing and case
    #[test]
    fn split_annot() {
        let tokens = all_tokens("#hai This is #GIMMEH   bold test #oic . #KBYE").unwrap();
        assert_eq!(tokens, vec!["#HAI", "This is", "#GIMMEH BOLD", "test", "#OIC", ".", "#KBYE"]);
    }

    /// #MAEK and its second word can be on different lines
    #[test]
    fn maek_keyword_nextline() {
        let tokens = all_tokens("#MAEK\n  PARAGRAF").unwrap();
        assert_eq!(tokens, vec!["#MAEK PARAGRAF"]);
    }

    /// A bad annotation should give a lexical error
    #[test]
    fn report_annot() {
        let err = all_tokens("#HAI\n#HEAD").unwrap_err();
        assert_eq!(err, "lexical error: not a valid annotation");
    }

    /// The error should say which character wasn't allowed
    #[test]
    fn report_badtext() {
        let err = all_tokens("#HAI a < b #KBYE").unwrap_err();
        assert!(err.contains("'<'"));
    }
}
