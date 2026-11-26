# nomux-theme

The theme loader of [nomux](https://github.com/nomux-sh): the semantic design tokens of the nomux UI, theme files, the contrast rules and the import of an iTerm colour scheme.

A nomux theme is a TOML file that gives values to tokens such as `surface.base`, `text.muted` or `state.needs-you`. A token that a theme does not set comes from the base theme that it `extends`. The crate embeds the theme files of the [themes repository](https://github.com/nomux-sh/themes). The JSON Schema and the token reference are in that repository.

## Status

In development. Versions earlier than 1.0.0 are pre-releases: interfaces can change between minor versions.

The crate has these parts now:

- `token`: the token set. Each token has a name, a kind, a role and a use.
- `shipped`: the themes that the repository ships, embedded in the library. `shipped::themes()` gives each theme with `extends` resolved, and `shipped::theme(id)` gives one theme. The appearance of each theme (`Theme::appearance`) is dark or light, thus a client can follow the setting of the system.
- `ThemeSet`: reads theme files, validates the token names and the value syntax, and resolves `extends`. An unknown token is an error with the nearest token name as a hint. A cycle in `extends` is an error.
- `Theme`: typed access to a resolved theme. Colours are `Rgba`, and there are lengths, durations, font specifications and the metrics of the two densities.
- `contrast_ratio` and `Rgba`: the relative luminance and the contrast ratio of WCAG 2.x.
- `rules`: the visual rules that each theme must obey, for example a text contrast of 4.5:1 on each surface.
- `ItermScheme`: reads an `.itermcolors` file and writes a theme file that sets the terminal tokens.
- The `nomux-theme` command: `check`, `preview`, `schema`, `reference` and `import-iterm`.

## Features

- `shipped` (default): embeds the theme files in the library. The embedded themes add approximately 22 KiB to a stripped release binary on x86_64 Linux. If a client brings its own themes, turn the feature off with `default-features = false` and give the text of the files to `ThemeSet`.

To add themes of a user that extend a shipped theme, insert `shipped::FILES` and the files of the user into one `ThemeSet`.

The crate has no GUI dependency and no `unsafe` code. The minimum supported Rust version is 1.85.

## Example

```rust
use nomux_theme::{Appearance, ColorToken, Density, TextStyle, contrast_ratio, rules, shipped};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let theme = shipped::theme("nomux-dark").ok_or("no theme nomux-dark")?;
    assert_eq!(theme.appearance(), Appearance::Dark);

    let surface = theme.color(ColorToken::SurfaceBase);
    let muted = theme.color(ColorToken::TextMuted);
    assert!(contrast_ratio(muted, surface) >= rules::TEXT_CONTRAST);

    let title = theme.font(TextStyle::Title);
    println!("{} {} {}", title.family, title.size, title.weight);
    println!("{}", theme.density(Density::Compact).row_height);

    for violation in rules::violations(theme) {
        eprintln!("{}: {violation}", theme.id());
    }
    Ok(())
}
```

To load the theme files of a directory, call `ThemeSet::load_dir`. To load a theme from text, for example after the file changed:

```rust
use nomux_theme::{Problem, ThemeSet};

let mut themes = ThemeSet::new();
let error = themes
    .insert("mine", "[theme]\nname = \"Mine\"\nappearance = \"dark\"\n[tokens]\n\"surface.bse\" = \"#16181c\"\n")
    .unwrap_err();
assert_eq!(
    error.problems,
    [Problem::UnknownToken { name: "surface.bse".to_owned(), nearest: Some("surface.base") }]
);
```

## Command

```sh
cargo install nomux-theme
nomux-theme check themes
```

The [README of the repository](https://github.com/nomux-sh/themes#the-nomux-theme-command) describes the commands.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
