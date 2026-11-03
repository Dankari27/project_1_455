//! Walks the parse tree, looks up variables using static


/// How an html tag is laid out 
#[derive(Clone, Copy)]
enum Layout {
    /// Gets its own lines, 
    /// and its contents are indented (html, head, p, ul).
    Block,
    /// Starts a new line, but its contents stay on that line (title, li).
    Line,
    /// Stays in the middle of the text (b).
    Inline,
}

/// One open block
struct Scope {
    /// Closes block
    close_tag: &'static str,
    /// How the closing tag should be written
    layout: Layout,
    /// Variables made inside this block (name, value)
    variables: Vec<(String, String)>,
}

/// Builds the html text (indented)
struct HtmlWriter {
    /// The html so far
    text: String,
    /// How many blocks deep we are, used for indenting
    depth: usize,
    /// True if the next thing should go on a new line
    at_line_start: bool,
    /// True if the next thing goes right after with no space (like after `<b>`)
    glue_next: bool,
}

impl HtmlWriter {
    /// Empty writer, nothing written yet and no indent
    fn new() -> Self {
        HtmlWriter { text: String::new(), depth: 0, at_line_start: true, glue_next: false }
    }

    /// Starts new line
    /// Adds 2 spaces of indent for every block we're in
    fn new_line(&mut self) {
        // Don't want a blank line at the very top
        if !self.text.is_empty() {
            self.text.push('\n');
        }
        self.text.push_str(&"  ".repeat(self.depth));
    }

    /// Used for comments
    /// Puts the string on its own line, and the next thing goes on a new line after it
    fn line(&mut self, s: &str) {
        self.new_line();
        self.text.push_str(s);
        self.at_line_start = true;
    }

    /// Adds text in the flow (sep by space)
    /// No space if it's the start of a line or right after an opening tag
    fn inline(&mut self, s: &str) {
        if self.at_line_start {
            self.new_line();
        } else if !self.glue_next {
            self.text.push(' ');
        }
        self.text.push_str(s);
        self.at_line_start = false;
        self.glue_next = false;
    }

    /// Writes an opening tag
    /// Block tags indent what comes after, line and inline tags glue their contents on
    fn open(&mut self, tag: &str, layout: Layout) {
        match layout {
            Layout::Block => {
                self.line(tag);
                self.depth += 1;
            }
            Layout::Line => {
                self.line(tag);
                self.at_line_start = false;
                self.glue_next = true;
            }
            Layout::Inline => {
                self.inline(tag);
                self.glue_next = true;
            }
        }
    }

    /// Writes a closing tag
    /// Block tags un-indent first and go on their own line, the others go right after the text
    fn close(&mut self, tag: &str, layout: Layout) {
        match layout {
            Layout::Block => {
                self.depth -= 1;
                self.line(tag);
            }
            Layout::Line => {
                self.text.push_str(tag);
                self.at_line_start = true;
            }
            Layout::Inline => {
                self.text.push_str(tag);
                self.at_line_start = false;
                self.glue_next = false;
            }
        }
    }
}

/// Returns the closest match
/// Checks the innermost block first, so an inner variable hides an outer one with the same name
fn find_variable<'a>(scopes: &'a [Scope], name: &str) -> Option<&'a str> {
    for scope in scopes.iter().rev() {
        for (var_name, value) in scope.variables.iter().rev() { //I learned iter existed, cool
            if var_name == name {
                return Some(value);
            }
        }
    }
    None
}

/// Turns the parse tree into html
/// Goes through the tokens in order and keeps a stack of open blocks for the closing tags and variables.
/// Gives a static semantic error if a variable is used that doesn't exist
pub fn generate_html(tree: &[(String, usize)]) -> Result<String, String> {
    let mut out = HtmlWriter::new();
    let mut scopes: Vec<Scope> = Vec::new();
    let mut i = 0;

    while i < tree.len() {
        let token = &tree[i].0;

        // Annotations 
        let opening = match token.as_str() {
            "#HAI" => Some(("<html>", "</html>", Layout::Block)),
            "#MAEK HEAD" => Some(("<head>", "</head>", Layout::Block)),
            "#GIMMEH TITLE" => Some(("<title>", "</title>", Layout::Line)),
            "#MAEK PARAGRAF" => Some(("<p>", "</p>", Layout::Block)),
            "#MAEK LIST" => Some(("<ul>", "</ul>", Layout::Block)),
            "#GIMMEH ITEM" => Some(("<li>", "</li>", Layout::Line)),
            "#GIMMEH BOLD" => Some(("<b>", "</b>", Layout::Inline)),
            _ => None,
        };
        if let Some((open_tag, close_tag, layout)) = opening {
            out.open(open_tag, layout);
            scopes.push(Scope { close_tag, layout, variables: Vec::new() });
            i += 1;
            continue;
        }

        i += match token.as_str() {
            // These end a block
            "#MKAY" | "#OIC" | "#KBYE" => {
                if let Some(scope) = scopes.pop() {
                    out.close(scope.close_tag, scope.layout);
                }
                1
            }

            // Comment can be empty, so skip 2 tokens if it is and 3 if it has text
            "#OBTW" => {
                let next = &tree[i + 1].0;
                if next == "#TLDR" {
                    out.line("<!-- -->");
                    2
                } else {
                    out.line(&format!("<!-- {} -->", next));
                    3
                }
            }
            "#NEWLINE" => {
                out.inline("<br>");
                1
            }

            // Address is used for both the href and the text shown
            "#GIMMEH LINX" => {
                let address = &tree[i + 1].0;
                out.inline(&format!("<a href=\"{}\">{}</a>", address, address));
                3
            }

            // Swap the variable for its value, error if it was never defined
            "#GIMMEH VAR" => {
                let name = &tree[i + 1].0;
                let value = find_variable(&scopes, name)
                    .ok_or_else(|| "Static semantic error: Something was used before it is defined.".to_string())?;
                out.inline(value);
                3
            }

            // #IHAZ name #ITIZ value #MKAY, so name is 1 ahead and value is 3 ahead.
            // The variable goes in the block we're in right now
            "#IHAZ" => {
                let name = tree[i + 1].0.clone();
                let value = tree[i + 3].0.clone();
                if let Some(scope) = scopes.last_mut() {
                    scope.variables.push((name, value));
                }
                5
            }
            // Copied as is
            _ => {
                out.inline(token);
                1
            }
        };
    }

    out.text.push('\n');
    Ok(out.text)
}

#[cfg(test)]
mod tests {
    use crate::compiler::LolCompiler;

    /// Compiles source string to html
    /// Goes through the whole compiler so semantic gets a real parse tree
    fn compile(source: &str) -> Result<String, String> {
        LolCompiler::new().try_compile(source)
    }

    /// A page with a head, title and text should come out indented right
    #[test]
    fn simple_page() {
        let html = compile("#HAI #MAEK HEAD #GIMMEH TITLE Hi #OIC #MKAY Hello #KBYE").unwrap();
        assert_eq!(
            html,
            "<html>\n  <head>\n    <title>Hi</title>\n  </head>\n  Hello\n</html>\n"
        );
    }

    /// Bold, newline and link should all stay on the same line as the text
    #[test]
    fn bold_and_newline() {
        let html = compile("#HAI a #GIMMEH BOLD b #OIC #NEWLINE #GIMMEH LINX www.x.com #OIC #KBYE").unwrap();
        assert!(html.contains("a <b>b</b> <br> <a href=\"www.x.com\">www.x.com</a>"));
    }

    /// A variable in a paragraph hides the outer one, then the outer one comes back after
    #[test]
    fn inner_hide_outer() {
        let source = "#HAI #IHAZ n #ITIZ Josh #MKAY #GIMMEH VAR n #OIC \
                      #MAEK PARAGRAF #IHAZ n #ITIZ Jon #MKAY #GIMMEH VAR n #OIC #MKAY \
                      #GIMMEH VAR n #OIC #KBYE";
        let html = compile(source).unwrap();
        assert_eq!(html, "<html>\n  Josh\n  <p>\n    Jon\n  </p>\n  Josh\n</html>\n");
    }

    /// A variable made in a paragraph can't be used after the paragraph ends
    #[test]
    fn block_end() {
        let source = "#HAI #MAEK PARAGRAF #IHAZ n #ITIZ Jon #MKAY hi #MKAY #GIMMEH VAR n #OIC #KBYE";
        let err = compile(source).unwrap_err();
        assert!(err.starts_with("Static semantic error"));
    }

    /// Using a variable that was never made gives the static semantic error
    #[test]
    fn undef_error() {
        let err = compile("#HAI\n#GIMMEH VAR who #OIC #KBYE").unwrap_err();
        assert_eq!(err, "Static semantic error: Something was used before it is defined.");
    }
}
