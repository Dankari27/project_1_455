//! Compiles a lolcode markdown (.lol) file into an html page


mod compiler;
mod lexer;
mod parser;
mod semantic;
mod traits;

use compiler::LolCompiler;
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use std::{env, fs};
use traits::Compiler;

/// Starts the program. Reads the .lol file, compiles it, saves the html and opens it in the browser
fn main() {
    let args: Vec<String> = env::args().collect();

    // Do we have one lol file?
    let input = check_args(&args).unwrap_or_else(|message| {
        eprintln!("{}", message);
        process::exit(1);
    });

    let source = fs::read_to_string(&input).unwrap_or_else(|_| {
        eprintln!("could not read");
        process::exit(1);
    });

    // Exit on any error, so we only get past here if everything worked
    let mut compiler = LolCompiler::new();
    compiler.compile(&source);

    // Save the html next to the input file with the same name
    let output = input.with_extension("html");
    if fs::write(&output, &compiler.html).is_err() {
        eprintln!("could not write");
        process::exit(1);
    }
    println!("Compiled '{}' into '{}'.", input.display(), output.display());

    // This skips opening the browser 
    if env::var_os("LOLCOMPILER_NO_BROWSER").is_none() {
        open_browser(&output);
    }
}

/// Checks that one argument was given and is the proper file type
/// Gives back the path if it's good, or an error message if it's not
fn check_args(args: &[String]) -> Result<PathBuf, String> {
    // args[0] is the program name so we need exactly 2
    if args.len() != 2 {
        return Err("error: expected exactly one input file".to_string());
    }
    let path = PathBuf::from(&args[1]);
    // Extension can be any case (.lol, .LOL, etc)
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) if ext.eq_ignore_ascii_case("lol") => Ok(path),
        _ => Err("Usage error: only .lol files can be compiled.".to_string()),
    }
}

/// Opens the html file in the default browser
/// Uses a different command depending on the OS (start, open, or xdg-open)
fn open_browser(path: &Path) {
    // Browser needs the full path or it might not find the file
    let full_path = std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf());
    let mut command = if cfg!(target_os = "windows") {
        let mut cmd = Command::new("cmd");
        cmd.args(["/C", "start", ""]);
        cmd
    } else if cfg!(target_os = "macos") {
        Command::new("open")
    } else {
        Command::new("xdg-open")
    };
    if command.arg(&full_path).spawn().is_err() {
        eprintln!("the HTML was created, but the browser could not be opened");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Transforms list of words
    /// Turns &str into String so it looks like what env::args gives us
    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    /// A .lol file should be accepted and the same path given back
    #[test]
    fn accept_lolfile() {
        assert_eq!(check_args(&args(&["lolcompiler", "Test2.lol"])), Ok(PathBuf::from("Test2.lol")));
    }

    /// Anything that isn't a .lol file should give a usage error
    #[test]
    fn reject_txtfile() {
        assert!(check_args(&args(&["lolcompiler", "Test1.txt"])).unwrap_err().starts_with("Usage error"));
    }

    /// Compiles one file from the test folder and returns the html or the error.
    fn compile_file(name: &str) -> Result<String, String> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("test").join(name);
        let source = fs::read_to_string(path).unwrap();
        LolCompiler::new().try_compile(&source)
    }

    /// Runs every test file and checks it gives html or the right kind of error
    #[test]
    fn expected_result() {
        // Format in file, what the result should start with
        let expected = [
            ("Test2.lol", "<html>"),
            ("Test3.lol", "<html>"),
            ("Test4.lol", "lexical error"),
            ("Test5.lol", "Syntax error"),
            ("Test6.lol", "<html>"),
            ("Test7.lol", "Static semantic error"),
            ("Test8.lol", "<html>"),
            ("Test9.lol", "<html>"),
            ("Test10.lol", "<html>"),
            ("Test11.lol", "<html>"),
            ("Test12.lol", "<html>"),
            ("Test13.lol", "<html>"),
            ("Test14.lol", "Syntax error"),
            ("Test15.lol", "Syntax error"),
            ("Test16.lol", "Syntax error"),
            ("Test17.lol", "lexical error"),
            ("Test18.lol", "Syntax error"),
            ("Test19.lol", "Static semantic error"),
            ("Test20.lol", "<html>"),
            ("Test21.lol", "lexical error"),
            ("Test22.lol", "Syntax error"),
            ("Test23.lol", "Syntax error"),
            ("Test24.lol", "<html>"),
        ];
        for (file, start) in expected {
            let result = compile_file(file).unwrap_or_else(|err| err);
            assert!(result.starts_with(start), "{} gave: {}", file, result);
        }
    }

    /// Test8 uses variables, so make sure their values show up in the html
    #[test]
    fn test8() {
        let html = compile_file("Test8.lol").unwrap();
        assert!(html.contains("is 42"));
        assert!(html.contains("really happiness"));
    }

    /// Test10 has a link, so make sure it turns into a proper `<a>` tag
    #[test]
    fn test10() {
        let html = compile_file("Test10.lol").unwrap();
        let url = "https://www.youtube.com/watch?v=aboZctrHfK8";
        assert!(html.contains(&format!("<a href=\"{}\">{}</a>", url, url)));
    }

    /// No files or too many files should both be errors
    #[test]
    fn reject_wrongnum() {
        assert!(check_args(&args(&["lolcompiler"])).is_err());
        assert!(check_args(&args(&["lolcompiler", "a.lol", "b.lol"])).is_err());
    }
}
