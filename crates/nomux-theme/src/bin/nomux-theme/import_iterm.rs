//! The `import-iterm` command.

use std::fs;
use std::path::Path;

use nomux_theme::ItermScheme;

pub fn run(file: &Path, name: &str) -> Result<(), String> {
    let text = fs::read_to_string(file)
        .map_err(|error| format!("cannot read {}: {error}", file.display()))?;
    let scheme =
        ItermScheme::parse(&text).map_err(|error| format!("{}: {error}", file.display()))?;
    print!("{}", scheme.to_theme_file(name));
    Ok(())
}
