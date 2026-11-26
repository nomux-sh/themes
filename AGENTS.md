# AGENTS.md

Rules for coding agents and for people who work in this repository. `CLAUDE.md` and `QWEN.md` refer to this file.

## This repository

`nomux-sh/themes` has the semantic design tokens of the nomux UI, the theme files, and the Rust crate `nomux-theme` that loads and checks a theme. crates.io publishes the crate.

| Path | Responsibility |
|---|---|
| `crates/nomux-theme/src/token.rs` | The token set: the one source of the token names, kinds, roles and uses. |
| `crates/nomux-theme/src/color.rs`, `src/value.rs` | Colour maths (WCAG 2.x) and the value kinds with their syntax. |
| `crates/nomux-theme/src/parse.rs`, `src/validate.rs`, `src/resolve.rs` | A theme file to its tables, the tokens against the token set, and `extends` to a complete theme. |
| `crates/nomux-theme/src/theme.rs`, `src/rules.rs` | Typed access to a resolved theme, and the visual rules that each theme must obey. |
| `crates/nomux-theme/src/iterm.rs` | Import of an iTerm colour scheme. |
| `crates/nomux-theme/src/bin/nomux-theme/` | The `nomux-theme` command: `check`, `preview`, `schema`, `reference`, `import-iterm`. |
| `crates/nomux-theme/tests/` | Tests of the iTerm import and of the shipped themes. |
| `themes/` | The theme files. `base-dark.toml` and `base-light.toml` set each token. |
| `schema/theme.schema.json` | JSON Schema of a theme file. Output of `scripts/generate.sh`. Not edited by hand. |
| `docs/tokens.md` | The token reference. Output of `scripts/generate.sh`. Not edited by hand. |
| `docs/sidebar.md` | The specification of the sidebar of the desktop client. The preview page shows it. |
| `scripts/` | `generate.sh` and `check-generated.sh` for the generated files, `release-notes.sh` for the release workflow. |
| `THIRD_PARTY.md`, `NOTICE` | The sources and the licenses of the palettes that are not from nomux. |
| `Cargo.toml`, `deny.toml`, `rust-toolchain.toml` | Workspace members and shared versions, license and source policy, pinned toolchain. |
| `cliff.toml`, `CHANGELOG.md` | Changelog configuration and the changelog. |
| `.github/workflows/` | `ci.yml` (checks), `pull-request.yml` (sign-off, commit messages), `pages.yml` (publishes the preview page), `release.yml` (publishes a version tag). |

## Rules for this repository

- A token has a name that says its role, not its value. Do not add a token such as `blue` or `grey-700`.
- Keep the token set small. Add a token only when a part of the UI needs it, and write its role and its use in `token.rs`.
- A removed or renamed token is a breaking change: each theme file and each client depends on the names.
- When you change `token.rs` or a rule constant in `rules.rs`, run `scripts/generate.sh`. Commit the token set, the schema and the token reference in the same commit.
- Do not edit `schema/theme.schema.json` or `docs/tokens.md` by hand.
- `base-dark.toml` and `base-light.toml` set each token. All other themes extend one of them and set only the tokens that differ.
- Each theme in `themes/` must pass `nomux-theme check`. Do not change a rule in `rules.rs` to make a theme pass. Change the theme.
- Colour carries only state. Do not add a second accent, and do not use a state colour for decoration.
- Do not commit font files. The theme files name the font families. The repositories of the clients ship the fonts.
- A palette that is not from nomux needs a license that permits redistribution. Read the license in the source repository, then record the source, the license and the copyright notice in `THIRD_PARTY.md` and `NOTICE`.
- The library has no GUI dependency, and the preview page has no script and no file from the network. Do not add a dependency without a measured reason.
- Pin each action in a workflow to a full commit SHA with a version comment.
- Do not publish the crate from a computer. The release workflow publishes it from a version tag. The steps are in the [release guide](https://github.com/nomux-sh/.github/blob/main/RELEASING.md).

## Checks

The tools and their versions are in [README.md](README.md#tools). Run these commands from the repository root before a push. CI runs the same commands.

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

If you changed `AGENTS.md`, also run this command with a clone of the [`.github`](https://github.com/nomux-sh/.github) repository next to this repository:

```sh
sh ../.github/standards/sync.sh . && git diff --exit-code
```

<!-- nomux:common:start -->
## Organization rules

These rules apply to each repository of the nomux-sh organization. The source of this block is `standards/AGENTS.common.md` in the [`.github`](https://github.com/nomux-sh/.github) repository. Do not edit the block in other repositories. CI compares it with the source.

### Before you change something

- Read the code around the change. Use the names, the structure and the style that the repository already has.
- Use the terms of the [glossary](https://github.com/nomux-sh/docs/blob/main/src/content/docs/reference/glossary.md). One concept has one name in code, CLI, UI and documentation.
- An architecture change needs an ADR in the [`docs`](https://github.com/nomux-sh/docs) repository before the implementation.
- Check versions, APIs and file paths before you use them. Do not guess.

### Scope

- Make only the change that the task asks for. Do not add options, abstractions or helpers that the task does not need.
- Do not add a placeholder that looks like an implementation. If work is deferred, say it in the pull request.
- Validate input at system boundaries. Trust internal code.

### Code

- Keep it simple (KISS). Build only what the task needs now (YAGNI). Have one source for each piece of logic (DRY).
- Write modern, readable code. Do not write clever code that the reader must decode.
- One type, one module and one file have one responsibility. Do not make a type that knows or does everything. Split it before it grows.
- Keep the file tree tidy. A new file goes where its neighbours have the same responsibility. Do not make catch-all directories or files such as `utils`, `common`, `helpers` or `misc`.
- Give each file, type and function a name that says what it is or does. Do not use abbreviations that the glossary does not have.
- Keep functions short and at one level of abstraction. Do not nest deeply. Return early.
- Do not duplicate code. If the same logic is necessary in a second place, move it to one place and call it.
- Comments: none by default. Write one line only for a reason that the code cannot show.
- Do not swallow errors. Do not add retries without a known cause.
- The performance targets of the project come before features. A change that breaks a target is not merged.
- Do not copy code from GPL, AGPL or Commons Clause projects.

### Tests

- Complete the code first. Write the tests last, and run them when the work is complete.
- Write the tests that matter: edge cases, behaviour that can break later, and the proof that the current behaviour is correct.
- Do not write a test only to have a test. Do not test what is obvious or what the compiler already guarantees.
- A test must fail when the behaviour that it guards is removed or moved.
- Do not mock the thing that the test examines.

### CI

- CI must be fast. Reviews and CI are the only things that slow a merge.
- Cache dependencies and build output. Run independent jobs in parallel. Cancel a run when a newer commit arrives.
- Put slow work (long fuzz runs, benchmarks, full matrix builds) in scheduled runs, not in pull request runs.
- When you add a CI step, measure the time that it adds and say it in the pull request.

### Text

- Write code, comments, documentation, commit messages and pull requests in English.
- Use short sentences, the active voice and one word for one concept. Put the condition before the instruction.
- Do not use these words: delve, robust, seamless, seamlessly, leverage, utilize, harness, pivotal, intricate, furthermore, moreover, notably, cutting-edge, best-in-class.

### Commits

- One subject line: `type: description`. No scope. 72 characters at most.
- Types: `build`, `chore`, `ci`, `docs`, `feat`, `fix`, `perf`, `refactor`, `revert`, `style`, `test`. Add `!` before the colon for a breaking change.
- The description says what changed, in plain English. Do not name a phase, a milestone or a ticket.
- Sign off each commit with `git commit -s`. `Signed-off-by` is the only trailer.
- Do not add `Co-authored-by`, a tool name or a "generated with" line to a commit or to a pull request.
- Enable the commit hook one time in each clone: `git config core.hooksPath .githooks`.

### Pull requests

- One logical change in each pull request. The branch name is short and names the change.
- The title has the format of a commit subject, 70 characters at most.
- The description says what changed and why, then shows the commands that you ran and their real result.
- Each commit in the pull request obeys the commit rules. The maintainers merge with a rebase, thus each commit goes to `main` unchanged.
- Do not merge when a check fails. Do not disable a check to make it pass.

### Never commit

- Secrets, tokens, keys or passwords.
- Paths, host names or account names of a local machine.
- Generated build output, editor files or local configuration.

### Before you push

1. Run the checks that the "Checks" section of this file lists.
2. Read your diff from top to bottom.
3. Make sure that the changelog and the documentation agree with the change.
<!-- nomux:common:end -->
