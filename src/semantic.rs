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
    layout: Layout,
    variables: Vec<(String, String)>,
}

/// Builds the html text (indented)
struct HtmlWriter {
    text: String,
    depth: usize,
    at_line_start: bool,
    glue_next: bool,
}

impl HtmlWriter {
    fn new() -> Self {
        HtmlWriter { text: String::new(), depth: 0, at_line_start: true, glue_next: false }
    }

    /// Starts new line
    fn new_line(&mut self) {
        if !self.text.is_empty() {
            self.text.push('\n');
        }
        self.text.push_str(&"  ".repeat(self.depth));
    }

    /// Used for comments
    fn line(&mut self, s: &str) {
        self.new_line();
        self.text.push_str(s);
        self.at_line_start = true;
    }

    /// Adds text in the flow (sep by space)
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

            "#GIMMEH LINX" => {
                let address = &tree[i + 1].0;
                out.inline(&format!("<a href=\"{}\">{}</a>", address, address));
                3
            }

            "#GIMMEH VAR" => {
                let name = &tree[i + 1].0;
                let value = find_variable(&scopes, name)
                    .ok_or_else(|| "Static semantic error: Something was used before it is defined.".to_string())?;
                out.inline(value);
                3
            }

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
    fn compile(source: &str) -> Result<String, String> {
        LolCompiler::new().try_compile(source)
    }

    #[test]
    fn simple_page() {
        let html = compile("#HAI #MAEK HEAD #GIMMEH TITLE Hi #OIC #MKAY Hello #KBYE").unwrap();
        assert_eq!(
            html,
            "<html>\n  <head>\n    <title>Hi</title>\n  </head>\n  Hello\n</html>\n"
        );
    }

    #[test]
    fn bold_and_newline() {
        let html = compile("#HAI a #GIMMEH BOLD b #OIC #NEWLINE #GIMMEH LINX www.x.com #OIC #KBYE").unwrap();
        assert!(html.contains("a <b>b</b> <br> <a href=\"www.x.com\">www.x.com</a>"));
    }

    #[test]
    fn inner_hide_outer() {
        let source = "#HAI #IHAZ n #ITIZ Josh #MKAY #GIMMEH VAR n #OIC \
                      #MAEK PARAGRAF #IHAZ n #ITIZ Jon #MKAY #GIMMEH VAR n #OIC #MKAY \
                      #GIMMEH VAR n #OIC #KBYE";
        let html = compile(source).unwrap();
        assert_eq!(html, "<html>\n  Josh\n  <p>\n    Jon\n  </p>\n  Josh\n</html>\n");
    }

    #[test]
    fn block_end() {
        let source = "#HAI #MAEK PARAGRAF #IHAZ n #ITIZ Jon #MKAY hi #MKAY #GIMMEH VAR n #OIC #KBYE";
        let err = compile(source).unwrap_err();
        assert!(err.starts_with("Static semantic error"));
    }

    #[test]
    fn undef_error() {
        let err = compile("#HAI\n#GIMMEH VAR who #OIC #KBYE").unwrap_err();
        assert_eq!(err, "Static semantic error: Something was used before it is defined.");
    }
}
