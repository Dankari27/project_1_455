// reads the source one character at a time and groups the characters into tokens

use crate::traits::LexicalAnalyzer;

// legal annotation
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

// punctuation inside plain text
const TEXT_PUNCTUATION: &str = ",.\"':?!%=/";

// holds source text
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
    // creates new lexer 
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

    // look at new character before moving on
    fn peek(&self) -> Option<char> {
        self.chars.get(self.position).copied()
    }

    // skips white space and other stuff
    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(char::is_whitespace) {
            self.get_char();
        }
    }

    // add letters to lexer until a letter doens't show up
    fn read_word(&mut self) {
        while self.peek().is_some_and(|c| c.is_ascii_alphabetic()) {
            let c = self.get_char();
            self.add_char(c);
        }
    }

    // finds the next token. 
    pub fn scan_token(&mut self) -> Result<String, String> {
        self.skip_whitespace();
        self.lexeme.clear();
        self.token_line = self.line;
        self.token_column = self.column;

        match self.peek() {
            None => Ok(String::new()),
            Some('#') => self.scan_annotation(),
            Some(_) => self.scan_text(),
        }
    }

    // reads an annotation such as `#HAI` or `#GIMMEH BOLD`.
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

    
    fn scan_text(&mut self) -> Result<String, String> {
        while self.peek().is_some_and(|c| c != '#') {
            let c = self.get_char();
            self.add_char(c);
        }

        let text = self.lexeme.trim_end().to_string();
        if self.lookup(&text) {
            Ok(text)
        } else {
            // find out the first character that isnt allowed.
            let bad = text.chars().find(|c| !is_text_char(*c)).unwrap_or(' ');
            Err(format!(
                "lexical error: character '{}' is not allowed in text",
                bad
            ))
        }
    }
}

// true if the character is allowed inside text
fn is_text_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, ' ' | '\t' | '\n' | '\r') || TEXT_PUNCTUATION.contains(c)
}

impl LexicalAnalyzer for Lexer {
    // returns the next character and then contiues
    fn get_char(&mut self) -> char {
        let Some(c) = self.peek() else {
            eprintln!("lexical error: unexpected end of file.");
            std::process::exit(1);
        };
        self.position += 1;
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        c
    }

    // adds one character to the token being built.
    fn add_char(&mut self, c: char) {
        self.lexeme.push(c);
    }

    // Annotations must be in the list above; text may only use allowed characters.
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

    // Collects every token in the source (stops at the end or the first error).
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

    #[test]
    fn lookup_accept() {
        let lexer = Lexer::new("");
        assert_eq!(lexer.lookup("#HAI"), true);
        assert_eq!(lexer.lookup("#hai"), true);
        assert_eq!(lexer.lookup("#GiMmEh BoLd"), true);
        assert_eq!(lexer.lookup("#KBYE"), true);
    }

    #[test]
    fn lookup_reject() {
        let lexer = Lexer::new("");
        assert_eq!(lexer.lookup("#HEY"), false);
        assert_eq!(lexer.lookup("#HEAD"), false);
        assert_eq!(lexer.lookup("#MAEK"), false);
        assert_eq!(lexer.lookup("#GIMMEH HEAD"), false);
    }

    #[test]
    fn lookup_check() {
        let lexer = Lexer::new("");
        assert_eq!(lexer.lookup("Hello, world!"), true);
        assert_eq!(lexer.lookup("www.towson.edu/a?b=1"), true);
        assert_eq!(lexer.lookup("a < b"), false);
        assert_eq!(lexer.lookup(""), false);
    }

    #[test]
    fn getchar_build_lexeme() {
        let mut lexer = Lexer::new("ab");
        assert_eq!(lexer.get_char(), 'a');
        assert_eq!(lexer.get_char(), 'b');
        lexer.add_char('x');
        lexer.add_char('y');
        assert_eq!(lexer.lexeme, "xy");
    }

    #[test]
    fn split_annot() {
        let tokens = all_tokens("#hai This is #GIMMEH   bold test #oic . #KBYE").unwrap();
        assert_eq!(tokens, vec!["#HAI", "This is", "#GIMMEH BOLD", "test", "#OIC", ".", "#KBYE"]);
    }

    #[test]
    fn maek_keyword_nextline() {
        let tokens = all_tokens("#MAEK\n  PARAGRAF").unwrap();
        assert_eq!(tokens, vec!["#MAEK PARAGRAF"]);
    }

    #[test]
    fn report_annot() {
        let err = all_tokens("#HAI\n#HEAD").unwrap_err();
        assert_eq!(err, "lexical error: not a valid annotation");
    }

    #[test]
    fn report_badtext() {
        let err = all_tokens("#HAI a < b #KBYE").unwrap_err();
        assert!(err.contains("'<'"));
    }
}
