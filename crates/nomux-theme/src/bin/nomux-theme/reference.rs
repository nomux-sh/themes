//! The `reference` command: the token reference (`docs/tokens.md`), from the
//! token set.

use nomux_theme::{ValueKind, rules, token};

const INTRODUCTION: &str = "\
<!-- This file comes from the token set in crates/nomux-theme/src/token.rs. Do not edit it. Run scripts/generate.sh. -->

# Tokens

The nomux UI reads one set of semantic design tokens. A client does not have a colour, a font, a length or a duration that is not a token.

A token has a dotted name that says its role, for example `surface.base` or `state.needs-you`. The name does not say a value: there is no `blue` and no `grey-700`.

A theme file gives values to tokens. The [README](../README.md#add-a-theme) shows how to write one. The crate [`nomux-theme`](../crates/nomux-theme) loads and checks theme files.

## Design rules

The token set applies the visual rules of nomux. The `check` command of `nomux-theme` examines the rules that have \"checked\" in this list.

- **Three elevation levels.** Controls rise: `surface.button` (level 1), `surface.raised` (level 2), `surface.floating` (level 3). The layout stays flat: regions differ by a tone step (`surface.sunken`, `surface.base`), not by a border. Do not put a box in a box.
- **Opaque surfaces and opaque text.** Checked: each surface, each text level and `terminal.background` have no transparency. Blur is permitted only on level 3.
- **Tone steps.** Checked: a surface is not equal to a surface that the UI puts below it or next to it. Thus a button stays visible on the base and on a card.
- **Text contrast.** Checked: `text.primary`, `text.secondary` and `text.muted` have a contrast of @TEXT@:1 or more on each surface, also below `fill.hover` and `fill.selected`. Muted text gets its level from its colour, not from opacity.
- **Colour carries only state.** There are five state colours and one accent. All other UI colours are neutral. Checked: each state colour, the accent and the focus ring have a contrast of @MARK@:1 or more on each surface. The colours of terminals, diffs and syntax are colours of the content, not of the UI.
- **Neutral hover and selection.** Checked: `fill.hover` and `fill.selected` are `text.primary` with an alpha from @FILL_MIN@% to @FILL_MAX@%. The text colour does not change on a selected row.
- **Levels of open groups.** The strip of an open group in the project rail has one more layer of `fill.level` over `surface.sunken` for each level, to @LEVELS@ levels. Checked: `fill.level` is `text.primary` with an alpha from @FILL_MIN@% to @FILL_MAX@%, each level has a contrast of @LEVEL@:1 or more to the level below, and `text.primary` and `text.secondary` have @TEXT@:1 or more on each level.
- **Hairlines.** Lines are 0.5 px to 1 px wide and have low alpha. A dark theme uses a line that is darker than the surface.
- **Typography.** One UI font and one monospace font. The scale is title 13 px medium, row 12 px, metadata 11 px.
- **Spacing.** Checked: each `space.*` and `density.*` length is a multiple of @GRID@ px. There are two densities: comfortable and compact.
- **Motion.** Checked: each transition is from @MOTION_MIN@ ms to @MOTION_MAX@ ms, and one cycle of a continuous indicator is from @INDICATOR_MIN@ ms to @INDICATOR_MAX@ ms. A transition uses ease-out and occurs only on a state change. The only continuous indicator is the working glyph. The client stops all motion when the user asks for reduced motion.
- **Content contrast.** Checked: `terminal.foreground` on `terminal.background`, each `syntax.*` colour on `surface.base`, and `text.primary` on each diff background have a contrast of @TEXT@:1 or more. The terminal cursor and the diff gutters have @MARK@:1 or more.

Contrast is the contrast ratio of WCAG 2.x. A transparent colour is painted over its background before the calculation.

## Value kinds

| Kind | Syntax in a theme file |
|---|---|
";

const GROUPS: [(&str, &str, &str); 17] = [
    (
        "surface",
        "Surfaces",
        "The tones of the layout and the three elevation levels. Each surface is opaque.",
    ),
    (
        "text",
        "Text",
        "The text levels. Each level keeps the text contrast of the design rules on each surface.",
    ),
    (
        "fill",
        "Fills",
        "Transparent tints that are painted over a surface to show hover, selection and the levels of open groups.",
    ),
    ("line", "Lines", "Hairlines and Pane dividers."),
    ("focus", "Focus", "The keyboard focus indicator."),
    (
        "accent",
        "Accent",
        "The one colour of the UI that is not neutral and is not a state.",
    ),
    ("shadow", "Shadow", "Only elevation level 3 has a shadow."),
    (
        "state",
        "State",
        "The five state colours. They are for status marks. Do not use them for decoration, and do not use a second colour for a state.",
    ),
    (
        "terminal",
        "Terminal",
        "The colours of a terminal: the defaults and the 16 ANSI colours. The iTerm import sets these tokens.",
    ),
    (
        "diff",
        "Diff",
        "The backgrounds and the gutter marks of a diff.",
    ),
    (
        "syntax",
        "Syntax",
        "A small set of syntax highlight roles. Text with no role uses `text.primary`.",
    ),
    (
        "font",
        "Typography",
        "The two font families and the type scale. Repositories of clients ship the font files.",
    ),
    ("space", "Spacing", "The steps of the 4 px grid."),
    (
        "density",
        "Density",
        "The metrics of the two densities. The user selects the density in the configuration.",
    ),
    ("size", "Sizes", "Fixed sizes of icons and marks."),
    (
        "radius",
        "Radii",
        "One corner radius for each elevation level.",
    ),
    (
        "motion",
        "Motion",
        "The durations of transitions and of the continuous indicator, and the easing of transitions.",
    ),
];

pub fn render() -> String {
    let mut text = INTRODUCTION
        .replace("@TEXT@", &rules::TEXT_CONTRAST.to_string())
        .replace("@MARK@", &rules::MARK_CONTRAST.to_string())
        .replace("@GRID@", &rules::GRID.to_string())
        .replace("@LEVEL@", &rules::LEVEL_CONTRAST.to_string())
        .replace("@LEVELS@", &rules::LEVELS.to_string())
        .replace("@FILL_MIN@", &rules::FILL_ALPHA_PERCENT.0.to_string())
        .replace("@FILL_MAX@", &rules::FILL_ALPHA_PERCENT.1.to_string())
        .replace(
            "@MOTION_MIN@",
            &rules::MOTION_RANGE.0.as_millis().to_string(),
        )
        .replace(
            "@MOTION_MAX@",
            &rules::MOTION_RANGE.1.as_millis().to_string(),
        )
        .replace(
            "@INDICATOR_MIN@",
            &rules::INDICATOR_RANGE.0.as_millis().to_string(),
        )
        .replace(
            "@INDICATOR_MAX@",
            &rules::INDICATOR_RANGE.1.as_millis().to_string(),
        );
    for kind in [
        ValueKind::Color,
        ValueKind::Length,
        ValueKind::Duration,
        ValueKind::FontFamily,
        ValueKind::FontWeight,
        ValueKind::Easing,
    ] {
        text.push_str(&format!(
            "| {} | {} |\n",
            kind_name(kind),
            kind.syntax().replace('"', "`")
        ));
    }

    text.push_str(&format!(
        "\n## Token list\n\nThe token set has {} tokens.\n",
        token::all().count()
    ));
    for (group, title, introduction) in GROUPS {
        text.push_str(&format!(
            "\n### {title}\n\n{introduction}\n\n| Token | Kind | Role | Use |\n|---|---|---|---|\n"
        ));
        for spec in token::all().filter(|spec| spec.group() == group) {
            text.push_str(&format!(
                "| `{}` | {} | {} | {} |\n",
                spec.name,
                kind_name(spec.kind),
                spec.role,
                spec.usage
            ));
        }
    }
    text
}

pub fn kind_name(kind: ValueKind) -> &'static str {
    match kind {
        ValueKind::Color => "colour",
        ValueKind::Length => "length",
        ValueKind::Duration => "duration",
        ValueKind::FontFamily => "font family",
        ValueKind::FontWeight => "font weight",
        ValueKind::Easing => "easing",
    }
}
