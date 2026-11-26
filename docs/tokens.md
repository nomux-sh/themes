<!-- This file comes from the token set in crates/nomux-theme/src/token.rs. Do not edit it. Run scripts/generate.sh. -->

# Tokens

The nomux UI reads one set of semantic design tokens. A client does not have a colour, a font, a length or a duration that is not a token.

A token has a dotted name that says its role, for example `surface.base` or `state.needs-you`. The name does not say a value: there is no `blue` and no `grey-700`.

A theme file gives values to tokens. The [README](../README.md#add-a-theme) shows how to write one. The crate [`nomux-theme`](../crates/nomux-theme) loads and checks theme files.

## Design rules

The token set applies the visual rules of nomux. The `check` command of `nomux-theme` examines the rules that have "checked" in this list.

- **Three elevation levels.** Controls rise: `surface.button` (level 1), `surface.raised` (level 2), `surface.floating` (level 3). The layout stays flat: regions differ by a tone step (`surface.sunken`, `surface.base`), not by a border. Do not put a box in a box.
- **Opaque surfaces and opaque text.** Checked: each surface, each text level and `terminal.background` have no transparency. Blur is permitted only on level 3.
- **Tone steps.** Checked: a surface is not equal to a surface that the UI puts below it or next to it. Thus a button stays visible on the base and on a card.
- **Text contrast.** Checked: `text.primary`, `text.secondary` and `text.muted` have a contrast of 4.5:1 or more on each surface, also below `fill.hover` and `fill.selected`. Muted text gets its level from its colour, not from opacity.
- **Colour carries only state.** There are five state colours and one accent. All other UI colours are neutral. Checked: each state colour, the accent and the focus ring have a contrast of 3:1 or more on each surface. The colours of terminals, diffs and syntax are colours of the content, not of the UI.
- **Neutral hover and selection.** Checked: `fill.hover` and `fill.selected` are `text.primary` with an alpha from 4% to 8%. The text colour does not change on a selected row.
- **Levels of open groups.** The strip of an open group in the project rail has one more layer of `fill.level` over `surface.sunken` for each level, to 5 levels. Checked: `fill.level` is `text.primary` with an alpha from 4% to 8%, each level has a contrast of 1.1:1 or more to the level below, and `text.primary` and `text.secondary` have 4.5:1 or more on each level.
- **Hairlines.** Lines are 0.5 px to 1 px wide and have low alpha. A dark theme uses a line that is darker than the surface.
- **Typography.** One UI font and one monospace font. The scale is title 13 px medium, row 12 px, metadata 11 px.
- **Spacing.** Checked: each `space.*` and `density.*` length is a multiple of 4 px. There are two densities: comfortable and compact.
- **Motion.** Checked: each transition is from 120 ms to 200 ms, and one cycle of a continuous indicator is from 800 ms to 1600 ms. A transition uses ease-out and occurs only on a state change. The only continuous indicator is the working glyph. The client stops all motion when the user asks for reduced motion.
- **Content contrast.** Checked: `terminal.foreground` on `terminal.background`, each `syntax.*` colour on `surface.base`, and `text.primary` on each diff background have a contrast of 4.5:1 or more. The terminal cursor and the diff gutters have 3:1 or more.

Contrast is the contrast ratio of WCAG 2.x. A transparent colour is painted over its background before the calculation.

## Value kinds

| Kind | Syntax in a theme file |
|---|---|
| colour | a string such as `#16181c` or `#16181c80` |
| length | a string such as `4px` or `0.5px` |
| duration | a string such as `120ms` |
| font family | an array of font family names |
| font weight | an integer from 1 to 1000 |
| easing | a string such as `cubic-bezier(0, 0, 0.2, 1)` |

## Token list

The token set has 88 tokens.

### Surfaces

The tones of the layout and the three elevation levels. Each surface is opaque.

| Token | Kind | Role | Use |
|---|---|---|---|
| `surface.sunken` | colour | The layout tone one step below the base. | Project rail and the track of a tab bar. |
| `surface.base` | colour | The base tone of the window. | Sidebar, Pane content that is not a terminal (diff, file, timeline) and each region that has no other surface. |
| `surface.button` | colour | Elevation level 1 (button): the flat fill of a control. | Buttons, shell chips of a tab bar, member chips of a team roster and input fields. |
| `surface.raised` | colour | Elevation level 2 (raised): the tone of a container that holds controls. | The active Tab, the expanded card of the active sidebar row and the approval card. |
| `surface.floating` | colour | Elevation level 3 (floating): the tone of a transient element above the layout. | Menus, the command palette, tooltips and hover previews. Only this level has a shadow. |

### Text

The text levels. Each level keeps the text contrast of the design rules on each surface.

| Token | Kind | Role | Use |
|---|---|---|---|
| `text.primary` | colour | The text level with the most contrast. | Titles, the names of rows, the active Tab and the text of content. |
| `text.secondary` | colour | The text level of items that are not active. | Tabs that are not active, chips and the letters of icons. |
| `text.muted` | colour | The text level with the least contrast. It keeps a contrast of 4.5:1 without opacity. | Metadata after ` · `, Child work rows, hints and placeholders. |
| `text.on-accent` | colour | The text on a fill of the accent colour. | The label of the primary button. |

### Fills

Transparent tints that are painted over a surface to show hover, selection and the levels of open groups.

| Token | Kind | Role | Use |
|---|---|---|---|
| `fill.hover` | colour | A neutral tint of `text.primary` (4% to 8%) that is painted over a surface. | The row, Tab or control below the pointer. |
| `fill.selected` | colour | A neutral tint of `text.primary` (4% to 8%) that is painted over a surface. The text colour does not change. | The selected row and a pressed control. |
| `fill.level` | colour | A neutral tint of `text.primary` that is painted once for each level of open groups, over `surface.sunken`. | The strip of an open group in the project rail. Adjacent levels keep the level contrast of the design rules. |

### Lines

Hairlines and Pane dividers.

| Token | Kind | Role | Use |
|---|---|---|---|
| `line.hairline` | colour | A line colour with low alpha. | Separators between items in one region, where a tone step is not possible. Do not use it to put a box around a region. |
| `line.divider` | colour | The line between two Panes. | Pane dividers. The divider gets the accent colour when the pointer is on it. |
| `line.width` | length | The width of a line. | Hairlines and Pane dividers. |

### Focus

The keyboard focus indicator.

| Token | Kind | Role | Use |
|---|---|---|---|
| `focus.ring` | colour | The colour of the keyboard focus indicator. | The ring around the control or the row that has the keyboard focus. |
| `focus.ring-width` | length | The width of the focus ring. | The ring around the control or the row that has the keyboard focus. |

### Accent

The one colour of the UI that is not neutral and is not a state.

| Token | Kind | Role | Use |
|---|---|---|---|
| `accent` | colour | The one accent colour of the theme. | The fill of the primary button, the indicator of the active Tab, the Pane divider below the pointer and the text cursor of an input field. |

### Shadow

Only elevation level 3 has a shadow.

| Token | Kind | Role | Use |
|---|---|---|---|
| `shadow.floating` | colour | The shadow colour of elevation level 3. | The shadow of menus, the command palette, tooltips and hover previews. The client sets the offset and the blur. |

### State

The five state colours. They are for status marks. Do not use them for decoration, and do not use a second colour for a state.

| Token | Kind | Role | Use |
|---|---|---|---|
| `state.working` | colour | State colour blue: the agent works. | Status marks only: the mark of a row, a Tab, a project tile or a team member. |
| `state.needs-you` | colour | State colour yellow: the agent waits for the user (permission, question or sign-in). | Status marks and the edge of the approval card. It is the only state that can pulse. |
| `state.done` | colour | State colour green: the agent is done and the user did not read the result. | Status marks only. |
| `state.error` | colour | State colour red: the agent or the Session stopped with an error. | Status marks, and the mark of a field that has an input error. |
| `state.idle` | colour | State colour grey: the Session is idle. | Status marks only. |

### Terminal

The colours of a terminal: the defaults and the 16 ANSI colours. The iTerm import sets these tokens.

| Token | Kind | Role | Use |
|---|---|---|---|
| `terminal.background` | colour | The default background of a terminal. | The cells of a terminal and the region around them in a terminal Tab. |
| `terminal.foreground` | colour | The default text colour of a terminal. | Terminal text that has no ANSI colour. |
| `terminal.cursor` | colour | The colour of the terminal cursor. | The cursor block, bar or underline of a terminal. |
| `terminal.selection` | colour | The background of selected terminal text. | It is painted over the cell background. The text colour does not change. |
| `terminal.ansi.black` | colour | ANSI colour 0. | Terminal cells that set colour 0. |
| `terminal.ansi.red` | colour | ANSI colour 1. | Terminal cells that set colour 1. |
| `terminal.ansi.green` | colour | ANSI colour 2. | Terminal cells that set colour 2. |
| `terminal.ansi.yellow` | colour | ANSI colour 3. | Terminal cells that set colour 3. |
| `terminal.ansi.blue` | colour | ANSI colour 4. | Terminal cells that set colour 4. |
| `terminal.ansi.magenta` | colour | ANSI colour 5. | Terminal cells that set colour 5. |
| `terminal.ansi.cyan` | colour | ANSI colour 6. | Terminal cells that set colour 6. |
| `terminal.ansi.white` | colour | ANSI colour 7. | Terminal cells that set colour 7. |
| `terminal.ansi.bright-black` | colour | ANSI colour 8. | Terminal cells that set colour 8. |
| `terminal.ansi.bright-red` | colour | ANSI colour 9. | Terminal cells that set colour 9. |
| `terminal.ansi.bright-green` | colour | ANSI colour 10. | Terminal cells that set colour 10. |
| `terminal.ansi.bright-yellow` | colour | ANSI colour 11. | Terminal cells that set colour 11. |
| `terminal.ansi.bright-blue` | colour | ANSI colour 12. | Terminal cells that set colour 12. |
| `terminal.ansi.bright-magenta` | colour | ANSI colour 13. | Terminal cells that set colour 13. |
| `terminal.ansi.bright-cyan` | colour | ANSI colour 14. | Terminal cells that set colour 14. |
| `terminal.ansi.bright-white` | colour | ANSI colour 15. | Terminal cells that set colour 15. |

### Diff

The backgrounds and the gutter marks of a diff.

| Token | Kind | Role | Use |
|---|---|---|---|
| `diff.added.background` | colour | The background of an added line. It is painted over `surface.base`. | Added lines of a diff. |
| `diff.added.gutter` | colour | The mark of an added line. | The gutter bar and the `+` sign of an added line. |
| `diff.removed.background` | colour | The background of a removed line. It is painted over `surface.base`. | Removed lines of a diff. |
| `diff.removed.gutter` | colour | The mark of a removed line. | The gutter bar and the `-` sign of a removed line. |
| `diff.changed.background` | colour | The background of the changed part of a line. It is painted over `surface.base`. | The words that changed in a line, and a changed line in a file gutter. |
| `diff.changed.gutter` | colour | The mark of a changed line. | The gutter bar of a changed line. |

### Syntax

A small set of syntax highlight roles. Text with no role uses `text.primary`.

| Token | Kind | Role | Use |
|---|---|---|---|
| `syntax.comment` | colour | Syntax role: comments. | Code in a diff, a file or a timeline. |
| `syntax.keyword` | colour | Syntax role: keywords and operators that are words. | Code in a diff, a file or a timeline. |
| `syntax.string` | colour | Syntax role: strings and characters. | Code in a diff, a file or a timeline. |
| `syntax.constant` | colour | Syntax role: numbers, booleans and named constants. | Code in a diff, a file or a timeline. |
| `syntax.function` | colour | Syntax role: names of functions and methods. | Code in a diff, a file or a timeline. |
| `syntax.type` | colour | Syntax role: names of types. | Code in a diff, a file or a timeline. |

### Typography

The two font families and the type scale. Repositories of clients ship the font files.

| Token | Kind | Role | Use |
|---|---|---|---|
| `font.ui` | font family | The one font family of the UI. | All text that is not code. |
| `font.mono` | font family | The one monospace font family. | Terminals, diffs, files and code in a timeline. |
| `font.title.size` | length | Type scale: the size of a title. | The title row of a surface, and the title of a row or a Tab. |
| `font.row.size` | length | Type scale: the size of a row. | The second line of a row, body text and the labels of controls. |
| `font.meta.size` | length | Type scale: the size of metadata. | Metadata, counters and hints. |
| `font.code.size` | length | Type scale: the size of monospace text. | Terminals, diffs, files and code in a timeline. |
| `font.title.weight` | font weight | Type scale: the weight of a title. | The title row of a surface, and the title of a row or a Tab. |
| `font.row.weight` | font weight | Type scale: the weight of a row. | The second line of a row, body text and the labels of controls. |
| `font.meta.weight` | font weight | Type scale: the weight of metadata. | Metadata, counters and hints. |
| `font.code.weight` | font weight | Type scale: the weight of monospace text. | Terminals, diffs, files and code in a timeline. |

### Spacing

The steps of the 4 px grid.

| Token | Kind | Role | Use |
|---|---|---|---|
| `space.1` | length | Spacing step 1 of the 4 px grid. | The gap between an icon or a mark and its label. |
| `space.2` | length | Spacing step 2 of the 4 px grid. | The gap between the items of a row and between controls. |
| `space.3` | length | Spacing step 3 of the 4 px grid. | The padding of a card and of a floating element. |
| `space.4` | length | Spacing step 4 of the 4 px grid. | The gap between the sections of a region, and the indent of one tree level. |

### Density

The metrics of the two densities. The user selects the density in the configuration.

| Token | Kind | Role | Use |
|---|---|---|---|
| `density.comfortable.row-height` | length | Comfortable density: the height of a row. | Sidebar rows and menu rows. |
| `density.comfortable.control-height` | length | Comfortable density: the height of a control. | Buttons, Tabs, chips and input fields. |
| `density.comfortable.padding` | length | Comfortable density: the horizontal padding. | The inner edges of rows, Tabs and controls. |
| `density.comfortable.two-line-row-height` | length | Comfortable density: the height of a row that has a second line. | Sidebar rows that show a name and a line of metadata below it, for example a worktree and its branch. |
| `density.compact.row-height` | length | Compact density: the height of a row. | Sidebar rows and menu rows. |
| `density.compact.control-height` | length | Compact density: the height of a control. | Buttons, Tabs, chips and input fields. |
| `density.compact.padding` | length | Compact density: the horizontal padding. | The inner edges of rows, Tabs and controls. |
| `density.compact.two-line-row-height` | length | Compact density: the height of a row that has a second line. | Sidebar rows that show a name and a line of metadata below it, for example a worktree and its branch. |

### Sizes

Fixed sizes of icons and marks.

| Token | Kind | Role | Use |
|---|---|---|---|
| `size.icon` | length | The one icon size of a row. | Icons in rows, Tabs, chips and buttons. |
| `size.mark` | length | The diameter of a status mark. | Status marks. A state does not change the size of the mark. |
| `size.corner-mark` | length | The diameter of a mark at the corner of a tile. | The state mark and the open mark of a tile in the project rail. |

### Radii

One corner radius for each elevation level.

| Token | Kind | Role | Use |
|---|---|---|---|
| `radius.button` | length | The corner radius of elevation level 1. | Buttons, chips, Tabs, rows and input fields. |
| `radius.raised` | length | The corner radius of elevation level 2. | Cards. |
| `radius.floating` | length | The corner radius of elevation level 3. | Menus, the command palette, tooltips and hover previews. |

### Motion

The durations of transitions and of the continuous indicator, and the easing of transitions.

| Token | Kind | Role | Use |
|---|---|---|---|
| `motion.fast` | duration | The duration of a colour change, from 120 ms to 200 ms. | A state change of a mark, a hover fill and a selection fill. |
| `motion.slow` | duration | The duration of a size change, from 120 ms to 200 ms. | A row that expands, a panel that opens and a Tab that moves. |
| `motion.indicator` | duration | The duration of one cycle of a continuous indicator, from 800 ms to 1600 ms. | One turn of the working glyph. The indicator turns at a constant speed while the agent works. |
| `motion.easing` | easing | The one easing of the UI: ease-out. | Each transition. A transition occurs only on a state change. A continuous indicator does not use an easing. The client stops all motion when the user asks for reduced motion. |
