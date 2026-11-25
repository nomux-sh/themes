//! The `nomux-theme` command: checks theme files and writes the files that
//! come from the token set.

mod check;
mod import_iterm;
mod preview;
mod reference;
mod schema;

use std::process::ExitCode;

const USAGE: &str = "\
Usage: nomux-theme <command>

Commands:
  check <themes-directory>
      Validate each theme file and the visual rules.
  preview <themes-directory> <output.html>
      Write the preview page of all themes.
  schema
      Print the JSON Schema of a theme file.
  reference
      Print the token reference.
  import-iterm <file.itermcolors> <theme-name>
      Print a theme file with the terminal colours of an iTerm colour scheme.
";

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let arguments: Vec<&str> = arguments.iter().map(String::as_str).collect();
    let result = match arguments.as_slice() {
        ["check", directory] => check::run(directory.as_ref()),
        ["preview", directory, output] => preview::run(directory.as_ref(), output.as_ref()),
        ["schema"] => {
            print!("{}", schema::render());
            Ok(())
        }
        ["reference"] => {
            print!("{}", reference::render());
            Ok(())
        }
        ["import-iterm", file, name] => import_iterm::run(file.as_ref(), name),
        _ => {
            eprint!("{USAGE}");
            return ExitCode::from(2);
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}
