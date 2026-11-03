//! A recursive descent parser with one function per grammar rule


use crate::compiler::LolCompiler;
use crate::traits::{Compiler, SyntaxAnalyzer};

impl LolCompiler {
    /// Parsers variable defs
    /// Keeps going as long as there's another #IHAZ, so zero or more
    fn parse_variable(&mut self) -> Result<(), String> {
        while self.current_token() == "#IHAZ" {
            self.parse_variable_define()?;
        }
        Ok(())
    }

    /// Variable names must be a single word (num and char)
    /// Only checks the name, doesn't accept it (parse_text does that)
    fn check_variable(&self) -> Result<(), String> {
        let name = self.current_token();
        if self.is_text() && name.chars().all(|c| c.is_ascii_alphanumeric()) {
            Ok(())
        } else {
            Err(self.syntax_error("a one-word variable name"))
        }
    }

    /// True if the current token can start inner text
    /// Used to know when to stop looping in the body and paragraphs
    fn starts_innertext(&self) -> bool {
        matches!(
            self.current_token().as_str(),
            "#GIMMEH VAR" | "#OBTW" | "#GIMMEH BOLD" | "#MAEK LIST" | "#GIMMEH LINX" | "#NEWLINE"
        ) || self.is_text()
    }
}

impl SyntaxAnalyzer for LolCompiler {
    /// Syntax
    /// `<lolcode> ::= #HAI <comment>* <head>? <variable-define>* <body> #KBYE`
    fn parse_lolcode(&mut self) -> Result<(), String> {
        self.expect("#HAI")?;
        // Comments and head are optional so only parse them if they show up
        while self.current_token() == "#OBTW" {
            self.parse_comment()?;
        }
        if self.current_token() == "#MAEK HEAD" {
            self.parse_head()?;
        }
        self.parse_variable()?;
        self.parse_body()?;
        self.expect("#KBYE")?;

        // Nothing is allowed after #KBYE.
        if !self.current_token().is_empty() {
            return Err(self.syntax_error("the end of the file after #KBYE"));
        }
        Ok(())
    }

    /// Everything below here just parses stuff for the grammar rules
    /// `<head> ::= #MAEK HEAD <title> #MKAY`
    fn parse_head(&mut self) -> Result<(), String> {
        self.expect("#MAEK HEAD")?;
        self.parse_title()?;
        self.expect("#MKAY")
    }


    /// `<title> ::= #GIMMEH TITLE <text> #OIC`
    fn parse_title(&mut self) -> Result<(), String> {
        self.expect("#GIMMEH TITLE")?;
        self.parse_text()?;
        self.expect("#OIC")
    }


    /// `<comment> ::= #OBTW <text>? #TLDR`  (the comment can be empty)
    fn parse_comment(&mut self) -> Result<(), String> {
        self.expect("#OBTW")?;
        if self.is_text() {
            self.parse_text()?;
        }
        self.expect("#TLDR")
    }


    /// `<body> ::= (<paragraph> | <inner-text>)*`
    /// Stops when it hits something that can't be in the body, like #KBYE
    fn parse_body(&mut self) -> Result<(), String> {
        loop {
            if self.current_token() == "#MAEK PARAGRAF" {
                self.parse_paragraph()?;
            } else if self.starts_innertext() {
                self.parse_inner_text()?;
            } else {
                return Ok(());
            }
        }
    }


    /// `<paragraph> ::= #MAEK PARAGRAF <variable-define>* <inner-paragraph> #MKAY`
    /// Paragraphs can't go inside other paragraphs
    fn parse_paragraph(&mut self) -> Result<(), String> {
        self.expect("#MAEK PARAGRAF")?;
        self.parse_variable()?;
        self.parse_inner_paragraph()?;
        self.expect("#MKAY")
    }

  
    /// `<inner-paragraph> ::= <inner-text>*`
    fn parse_inner_paragraph(&mut self) -> Result<(), String> {
        while self.starts_innertext() {
            self.parse_inner_text()?;
        }
        Ok(())
    }

  
    /// `<inner-text> ::= <variable-use> | <comment> | <bold> | <list> | <link> | <newline> | <text>`
    /// Looks at the current token to pick which one to parse
    fn parse_inner_text(&mut self) -> Result<(), String> {
        match self.current_token().as_str() {
            "#GIMMEH VAR" => self.parse_variable_use(),
            "#OBTW" => self.parse_comment(),
            "#GIMMEH BOLD" => self.parse_bold(),
            "#MAEK LIST" => self.parse_list(),
            "#GIMMEH LINX" => self.parse_link(),
            "#NEWLINE" => self.parse_newline(),
            _ => self.parse_text(),
        }
    }


    /// `<variable-define> ::= #IHAZ <name> #ITIZ <value> #MKAY`
    fn parse_variable_define(&mut self) -> Result<(), String> {
        self.expect("#IHAZ")?;
        // Name has to be one word, then accept it like normal text
        self.check_variable()?;
        self.parse_text()?;
        self.expect("#ITIZ")?;
        self.parse_text()?;
        self.expect("#MKAY")
    }

   
    /// `<variable-use> ::= #GIMMEH VAR <name> #OIC`
    /// Doesn't check if the variable exists, semantic does that later
    fn parse_variable_use(&mut self) -> Result<(), String> {
        self.expect("#GIMMEH VAR")?;
        self.check_variable()?;
        self.parse_text()?;
        self.expect("#OIC")
    }

    /// `<bold> ::= #GIMMEH BOLD <bold-text> #OIC`  (bold text can be text or variables i think)
    fn parse_bold(&mut self) -> Result<(), String> {
        self.expect("#GIMMEH BOLD")?;
        loop {
            if self.is_text() {
                self.parse_text()?;
            } else if self.current_token() == "#GIMMEH VAR" {
                self.parse_variable_use()?;
            } else {
                break;
            }
        }
        self.expect("#OIC")
    }

  
    /// `<list> ::= #MAEK LIST <list-items> #MKAY`
    /// A list needs at least one item
    fn parse_list(&mut self) -> Result<(), String> {
        self.expect("#MAEK LIST")?;
        if self.current_token() != "#GIMMEH ITEM" {
            return Err(self.syntax_error("#GIMMEH ITEM right after #MAEK LIST"));
        }
        self.parse_list_items()?;
        self.expect("#MKAY")
    }

  
    /// `<list-items> ::= (#GIMMEH ITEM <variable-define>* <inner-list> #OIC)*`
    fn parse_list_items(&mut self) -> Result<(), String> {
        while self.current_token() == "#GIMMEH ITEM" {
            // Already know it's #GIMMEH ITEM so just accept it
            self.accept()?;
            self.parse_variable()?;
            self.parse_inner_list()?;
            self.expect("#OIC")?;
        }
        Ok(())
    }

    /// `<inner-list> ::= (<bold> | <link> | <newline> | <variable-use> | <comment> | <text>)*`
    /// Same as inner text but no lists inside lists
    fn parse_inner_list(&mut self) -> Result<(), String> {
        loop {
            match self.current_token().as_str() {
                "#GIMMEH BOLD" => self.parse_bold()?,
                "#GIMMEH LINX" => self.parse_link()?,
                "#NEWLINE" => self.parse_newline()?,
                "#GIMMEH VAR" => self.parse_variable_use()?,
                "#OBTW" => self.parse_comment()?,
                _ if self.is_text() => self.parse_text()?,
                _ => return Ok(()),
            }
        }
    }

  
    /// `<link> ::= #GIMMEH LINX <address> #OIC`
    /// The address can't have any spaces in it
    fn parse_link(&mut self) -> Result<(), String> {
        self.expect("#GIMMEH LINX")?;
        if !self.is_text() || self.current_token().contains(char::is_whitespace) {
            return Err(self.syntax_error("a web address with no spaces"));
        }
        self.parse_text()?;
        self.expect("#OIC")
    }


    /// `<newline> ::= #NEWLINE`
    fn parse_newline(&mut self) -> Result<(), String> {
        self.expect("#NEWLINE")
    }

    /// Does a thing and accepts the piece of plain text
    /// If it's an annotation or the end of the file instead, it's a syntax error
    fn parse_text(&mut self) -> Result<(), String> {
        if self.is_text() {
            self.accept()
        } else {
            Err(self.syntax_error("text"))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::LolCompiler;
    use crate::lexer::Lexer;
    use crate::traits::SyntaxAnalyzer;

    /// Makes a compiler
    /// Loads the first token already so the parse functions can be tested on their own
    fn make_compiler(source: &str) -> LolCompiler {
        let mut compiler = LolCompiler::new();
        compiler.lexer = Lexer::new(source);
        compiler.advance_next_token().unwrap();
        compiler
    }

    /// Bold should work and put all 3 tokens in the parse tree
    #[test]
    fn parse_boldblock() {
        let mut c = make_compiler("#GIMMEH BOLD hi #OIC");
        assert_eq!(c.parse_bold(), Ok(()));
        assert_eq!(c.parse_tree.len(), 3);
    }

    /// A list with two items should parse fine
    #[test]
    fn parse_list() {
        let mut c = make_compiler("#MAEK LIST #GIMMEH ITEM a #OIC #GIMMEH ITEM b #OIC #MKAY");
        assert_eq!(c.parse_list(), Ok(()));
    }

    /// A list has to start with #GIMMEH ITEM, not text
    #[test]
    fn list_start_with_item() {
        let mut c = make_compiler("#MAEK LIST hello #MKAY");
        assert!(c.parse_list().is_err());
    }

    /// A paragraph inside a paragraph should be a syntax error
    #[test]
    fn not_nested() {
        let mut c = make_compiler("#MAEK PARAGRAF #MAEK PARAGRAF hi #MKAY #MKAY");
        assert!(c.parse_paragraph().unwrap_err().starts_with("Syntax error"));
    }

    /// Variable names with a space in them should fail
    #[test]
    fn one_word() {
        let mut c = make_compiler("#IHAZ my name #ITIZ Bob #MKAY");
        assert!(c.parse_variable_define().is_err());
    }

    /// Links with a space in them should fail
    #[test]
    fn no_whitespace() {
        let mut c = make_compiler("#GIMMEH LINX www.a.com www.b.com #OIC");
        assert!(c.parse_link().is_err());
    }

    /// A title without a head around it should fail
    #[test]
    fn title_inside_head() {
        let mut c = make_compiler("#HAI #GIMMEH TITLE hi #OIC #KBYE");
        assert!(c.parse_lolcode().is_err());
    }

    /// Anything after #KBYE should fail
    #[test]
    fn not_allowedkbyte() {
        let mut c = make_compiler("#HAI hi #KBYE more");
        assert!(c.parse_lolcode().is_err());
    }
}
