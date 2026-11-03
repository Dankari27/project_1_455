//a recursive descent parser with one function per grammar rule


use crate::compiler::LolCompiler;
use crate::traits::{Compiler, SyntaxAnalyzer};

impl LolCompiler {
    // parsers variabl defs
    fn parse_variable(&mut self) -> Result<(), String> {
        while self.current_token() == "#IHAZ" {
            self.parse_variable_define()?;
        }
        Ok(())
    }

    // variable names must be a single word (num and char)
    fn check_variable(&self) -> Result<(), String> {
        let name = self.current_token();
        if self.is_text() && name.chars().all(|c| c.is_ascii_alphanumeric()) {
            Ok(())
        } else {
            Err(self.syntax_error("a one-word variable name"))
        }
    }

    
    fn starts_innertext(&self) -> bool {
        matches!(
            self.current_token().as_str(),
            "#GIMMEH VAR" | "#OBTW" | "#GIMMEH BOLD" | "#MAEK LIST" | "#GIMMEH LINX" | "#NEWLINE"
        ) || self.is_text()
    }
}

impl SyntaxAnalyzer for LolCompiler {
    // syntax
    fn parse_lolcode(&mut self) -> Result<(), String> {
        self.expect("#HAI")?;
        while self.current_token() == "#OBTW" {
            self.parse_comment()?;
        }
        if self.current_token() == "#MAEK HEAD" {
            self.parse_head()?;
        }
        self.parse_variable()?;
        self.parse_body()?;
        self.expect("#KBYE")?;

        // nothing is allowed after #KBYE.
        if !self.current_token().is_empty() {
            return Err(self.syntax_error("the end of the file after #KBYE"));
        }
        Ok(())
    }

    // Everything below here just parses stuff for the grammar rules
   
    fn parse_head(&mut self) -> Result<(), String> {
        self.expect("#MAEK HEAD")?;
        self.parse_title()?;
        self.expect("#MKAY")
    }


    fn parse_title(&mut self) -> Result<(), String> {
        self.expect("#GIMMEH TITLE")?;
        self.parse_text()?;
        self.expect("#OIC")
    }


    fn parse_comment(&mut self) -> Result<(), String> {
        self.expect("#OBTW")?;
        if self.is_text() {
            self.parse_text()?;
        }
        self.expect("#TLDR")
    }


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


    fn parse_paragraph(&mut self) -> Result<(), String> {
        self.expect("#MAEK PARAGRAF")?;
        self.parse_variable()?;
        self.parse_inner_paragraph()?;
        self.expect("#MKAY")
    }

  
    fn parse_inner_paragraph(&mut self) -> Result<(), String> {
        while self.starts_innertext() {
            self.parse_inner_text()?;
        }
        Ok(())
    }

  
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


    fn parse_variable_define(&mut self) -> Result<(), String> {
        self.expect("#IHAZ")?;
        self.check_variable()?;
        self.parse_text()?;
        self.expect("#ITIZ")?;
        self.parse_text()?;
        self.expect("#MKAY")
    }

   
    fn parse_variable_use(&mut self) -> Result<(), String> {
        self.expect("#GIMMEH VAR")?;
        self.check_variable()?;
        self.parse_text()?;
        self.expect("#OIC")
    }

    // <bold> ::= #GIMMEH BOLD <bold-text> #OIC   (bold text can be text or variables i think)
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

  
    fn parse_list(&mut self) -> Result<(), String> {
        self.expect("#MAEK LIST")?;
        if self.current_token() != "#GIMMEH ITEM" {
            return Err(self.syntax_error("#GIMMEH ITEM right after #MAEK LIST"));
        }
        self.parse_list_items()?;
        self.expect("#MKAY")
    }

  
    fn parse_list_items(&mut self) -> Result<(), String> {
        while self.current_token() == "#GIMMEH ITEM" {
            self.accept()?;
            self.parse_variable()?;
            self.parse_inner_list()?;
            self.expect("#OIC")?;
        }
        Ok(())
    }

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

  
    fn parse_link(&mut self) -> Result<(), String> {
        self.expect("#GIMMEH LINX")?;
        if !self.is_text() || self.current_token().contains(char::is_whitespace) {
            return Err(self.syntax_error("a web address with no spaces"));
        }
        self.parse_text()?;
        self.expect("#OIC")
    }


    fn parse_newline(&mut self) -> Result<(), String> {
        self.expect("#NEWLINE")
    }

    // does a thing and accepts the piece of plain text
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

    // makes a compiler
    fn make_compiler(source: &str) -> LolCompiler {
        let mut compiler = LolCompiler::new();
        compiler.lexer = Lexer::new(source);
        compiler.advance_next_token().unwrap();
        compiler
    }

    #[test]
    fn parse_boldblock() {
        let mut c = make_compiler("#GIMMEH BOLD hi #OIC");
        assert_eq!(c.parse_bold(), Ok(()));
        assert_eq!(c.parse_tree.len(), 3);
    }

    #[test]
    fn parse_list() {
        let mut c = make_compiler("#MAEK LIST #GIMMEH ITEM a #OIC #GIMMEH ITEM b #OIC #MKAY");
        assert_eq!(c.parse_list(), Ok(()));
    }

    #[test]
    fn list_start_with_item() {
        let mut c = make_compiler("#MAEK LIST hello #MKAY");
        assert!(c.parse_list().is_err());
    }

    #[test]
    fn not_nested() {
        let mut c = make_compiler("#MAEK PARAGRAF #MAEK PARAGRAF hi #MKAY #MKAY");
        assert!(c.parse_paragraph().unwrap_err().starts_with("Syntax error"));
    }

    #[test]
    fn one_word() {
        let mut c = make_compiler("#IHAZ my name #ITIZ Bob #MKAY");
        assert!(c.parse_variable_define().is_err());
    }

    #[test]
    fn no_whitespace() {
        let mut c = make_compiler("#GIMMEH LINX www.a.com www.b.com #OIC");
        assert!(c.parse_link().is_err());
    }

    #[test]
    fn title_inside_head() {
        let mut c = make_compiler("#HAI #GIMMEH TITLE hi #OIC #KBYE");
        assert!(c.parse_lolcode().is_err());
    }

    #[test]
    fn not_allowedkbyte() {
        let mut c = make_compiler("#HAI hi #KBYE more");
        assert!(c.parse_lolcode().is_err());
    }
}
