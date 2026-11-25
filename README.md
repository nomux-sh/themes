# themes

The nomux themes: the schema of the semantic design tokens, the base light and dark themes, more themes, and the crate that loads and checks a theme (`nomux-theme`).

This repository is a part of [nomux](https://github.com/nomux-sh), a native and lightweight agentic development environment. Sessions stay alive on the host when the client disconnects.

## Status

In development. Versions earlier than 1.0.0 are pre-releases: interfaces can change between minor versions.

The repository has these parts now:

- The token set: 83 semantic design tokens for the full nomux UI. [docs/tokens.md](docs/tokens.md) gives each token, its role and where the UI can use it.
- The theme files in [themes/](themes), and the JSON Schema of a theme file in [schema/theme.schema.json](schema/theme.schema.json).
- The crate [`nomux-theme`](crates/nomux-theme): it loads a theme file, resolves `extends`, validates the tokens, gives typed access to the values and examines the visual rules. It also imports an iTerm colour scheme.
- The preview page: <https://nomux-sh.github.io/themes/>. It shows each theme with mock-ups of the nomux UI, each token and each contrast ratio. Each push to `main` publishes it.

These parts are not in the repository yet:

- Colours for the identity of a Project, an agent or a Team. The token set has state colours and one accent only.
- Preview screenshots from the component gallery of the desktop client, and a visual regression check. They start with the desktop client.
- Font files. The theme files name the font families (Inter and JetBrains Mono). The repositories of the clients ship the fonts.
- Light variants of the palettes that are not from nomux.

## Themes

| File | Appearance | Extends | Palette |
|---|---|---|---|
| `base-dark.toml` | dark | | nomux. Sets each token. |
| `base-light.toml` | light | | nomux. Sets each token. |
| `nomux-dark.toml` | dark | `base-dark` | nomux. The default dark theme. |
| `nomux-light.toml` | light | `base-light` | nomux. The default light theme. |
| `catppuccin-mocha.toml` | dark | `base-dark` | [Catppuccin](https://github.com/catppuccin/catppuccin), MIT. |
| `gruvbox-dark.toml` | dark | `base-dark` | [Gruvbox](https://github.com/morhetz/gruvbox), MIT/X11. |
| `tokyo-night.toml` | dark | `base-dark` | [Tokyo Night](https://github.com/tokyo-night/tokyo-night-vscode-theme), MIT. |

[THIRD_PARTY.md](THIRD_PARTY.md) has the license notices of the palettes that are not from nomux.

## Theme file

A theme is a TOML file. Its identifier is the file name without `.toml`.

```toml
[theme]
name = "nomux Dark"
appearance = "dark"       # light | dark
extends = "base-dark"     # the identifier of the base theme

[tokens]
"surface.base" = "#16181c"
"text.muted" = "#9aa1ab"
"accent" = "#6b8afd"
"state.needs-you" = "#e5b445"
```

- Put each token name in quotes. TOML reads a name without quotes as a table.
- A token that the file does not set comes from the base theme. The base theme can extend a third theme. A cycle is an error.
- The appearance must be equal to the appearance of the base theme.
- A theme without `extends` must set each token.

| Kind | Syntax | Example |
|---|---|---|
| colour | `#rgb`, `#rgba`, `#rrggbb` or `#rrggbbaa` | `"#16181c"`, `"#e6e8eb14"` |
| length | a number and `px` | `"4px"`, `"0.5px"` |
| duration | a whole number and `ms` | `"120ms"` |
| font family | an array of family names | `["Inter", "system-ui", "sans-serif"]` |
| font weight | an integer from 1 to 1000 | `500` |
| easing | `cubic-bezier` with four numbers | `"cubic-bezier(0, 0, 0.2, 1)"` |

An editor that knows JSON Schema can complete the token names. With the [Taplo](https://taplo.tamasfe.dev) language server, put this line at the top of the file:

```toml
#:schema https://raw.githubusercontent.com/nomux-sh/themes/main/schema/theme.schema.json
```

## Add a theme

1. Make sure that the license of the palette permits redistribution. Read the license in the source repository.
2. Copy `themes/nomux-dark.toml` or `themes/nomux-light.toml` to `themes/<identifier>.toml`. The identifier has only lowercase letters, digits and hyphens.
3. Set `name`, `appearance` and `extends` (`base-dark` or `base-light`).
4. Set the colour tokens of the palette. [docs/tokens.md](docs/tokens.md) gives the role of each token. Leave the typography, spacing and motion tokens to the base theme.
5. Run the check, and correct each line that it prints:

   ```sh
   cargo run -p nomux-theme -- check themes
   ```

   The check examines the token names, the value syntax and the visual rules. The most important rules: each text level has a contrast of 4.5:1 or more on each surface, each state colour has 3:1 or more, and `fill.hover` and `fill.selected` are `text.primary` with an alpha from 4% to 8%. If a colour of the palette does not pass on a surface, select a different tone of the palette for the surface or for the colour. Do not change a rule.
6. Write the preview page and look at the theme:

   ```sh
   cargo run -p nomux-theme -- preview themes site/index.html
   ```

7. If the palette is not your own work, add its source, its license and its copyright notice to `THIRD_PARTY.md` and `NOTICE`.
8. Add the theme to the table in this file, then open a pull request.

To start from an iTerm colour scheme, import it. The command prints a theme file that sets the terminal tokens:

```sh
cargo run -p nomux-theme -- import-iterm scheme.itermcolors "My Theme" > themes/my-theme.toml
```

## Token set

The UI reads one token set and has no colour, font, length or duration that is not a token. The token set applies the visual rules of nomux:

- Three elevation levels (button, raised, floating). Regions differ by a tone step, not by a border.
- Colour carries only state: working (blue), needs you (yellow), done and not read (green), error (red) and idle (grey), and one accent. All other UI colours are neutral.
- Hover and selection are a neutral tint of the text colour. The text colour does not change.
- Muted text keeps a contrast of 4.5:1. It is not made with opacity.
- One UI font and one monospace font, a 4 px grid with two densities, and motion from 120 ms to 200 ms.

The token set is in `crates/nomux-theme/src/token.rs`. The schema and the token reference come from it:

```sh
scripts/generate.sh
```

## The `nomux-theme` command

```text
nomux-theme check <themes-directory>
nomux-theme preview <themes-directory> <output.html>
nomux-theme schema
nomux-theme reference
nomux-theme import-iterm <file.itermcolors> <theme-name>
```

`check` exits with status 1 and prints one line for each problem when a theme file is not correct or a theme does not obey a rule. In this repository, start the command with `cargo run -p nomux-theme --`.

The library API is in the [README of the crate](crates/nomux-theme/README.md).

## Layout

| Path | Content |
|---|---|
| `crates/nomux-theme/` | The crate: the library and the `nomux-theme` command. |
| `themes/` | The theme files. |
| `schema/theme.schema.json` | JSON Schema of a theme file. Generated. |
| `docs/tokens.md` | The token reference. Generated. |
| `scripts/` | Scripts for the generated files and for the release notes. |
| `THIRD_PARTY.md`, `NOTICE` | Sources and licenses of the palettes that are not from nomux. |

## Tools

| Tool | Version | Used for |
|------|---------|----------|
| Rust | 1.100.0 (pinned in `rust-toolchain.toml`) | build, test |
| [cargo-nextest](https://nexte.st) | 0.9.152 | tests |
| [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) | 0.22.1 | license and advisory check |
| [cargo-machete](https://github.com/bnjbvr/cargo-machete) | 0.10.1 | unused dependency check |
| [git-cliff](https://git-cliff.org) | 2.14.2 | changelog |
| [actionlint](https://github.com/rhysd/actionlint) | 1.7.12 | workflow lint |

## Checks

Run these commands from the repository root before you open a pull request:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run --workspace
cargo test --workspace --doc
cargo deny check
cargo machete
cargo run --locked -p nomux-theme -- check themes
scripts/check-generated.sh
cargo package -p nomux-theme --locked
actionlint
```

CI runs the same checks with the reusable workflows of the [`nomux-sh/.github`](https://github.com/nomux-sh/.github) repository. `scripts/check-generated.sh` fails if the schema or the token reference does not agree with the token set.

## Releases

The crate `nomux-theme` has the version of the repository. A version tag (`vX.Y.Z`) starts the release workflow, which publishes the crate to crates.io and makes the GitHub release. The steps are in the [release guide](https://github.com/nomux-sh/.github/blob/main/RELEASING.md). The theme files have no version of their own: a client reads them from a tag of this repository.

## Security

To report a vulnerability, obey the [security policy](https://github.com/nomux-sh/.github/blob/main/SECURITY.md) of the nomux-sh organization.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option. The license applies to the code and to the theme files.

Three theme files use palettes of other projects. [THIRD_PARTY.md](THIRD_PARTY.md) has their sources and their license notices.

Copyright The nomux Authors.
