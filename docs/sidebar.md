# Sidebar

This document specifies the sidebar of the nomux workspace. It gives three directions. The owner selects one direction. Then the desktop client implements the selected direction from this document.

The preview page shows each direction in the section "Sidebar directions", for `nomux-dark` and `nomux-light`, in the two densities and in five states. The command `nomux-theme preview` writes the page. The workspace mock-up of each theme uses D1.

All values in this document come from the tokens in [tokens.md](tokens.md). The x positions are measured from the left edge of the sidebar. The sidebar is 272 px wide. The project rail is at the left of the sidebar.

## The problem

The old sidebar had these problems:

- Each row was a chain of words: `fix-login · Worktree · 3 changes`. The type words (`Project`, `Worktree`) repeated what the indent already said.
- Five kinds of row had six different text edges. A child row started to the left of its parent.
- Each row had a status mark at the far right edge. An open parent showed the same mark as its child, thus three yellow marks were in one column.
- A card, a selected row and a hover row gave three tints in ten rows: a box in a box.
- A line of detail was not a row. It looked like a caption for the whole card.
- The project name was in the rail, in the title row and in a project row.

## Shared rules

These rules apply to all three directions.

1. **No type words.** The shape, the position and one icon give the type of a row. A worktree shows a branch icon on its second line. A shell has a monospace name.
2. **The project name is only in the header.** There is no project row. The header shows the project name and, if a session needs the user, the needs-you glyph and the count, for example "1 needs you".
3. **Aggregate state only on a closed parent.** A closed worktree, or a closed session with children, shows the most urgent state of its children. An open parent shows no aggregate. The priority is: needs you, error, working, done and not seen, idle.
4. **One glyph set.** Each state has its own shape, thus the state does not depend on colour only:

   | State | Shape | Colour token |
   |---|---|---|
   | Working | arc of three quarters | `state.working` |
   | Needs you | filled disc | `state.needs-you` |
   | Done, not seen | check | `state.done` |
   | Error | cross | `state.error` |
   | Idle | hollow ring | `state.idle` |

   The glyph is `size.mark` (8 px) in a slot of `size.icon` (16 px). A state never changes the size of the slot or of the row.
5. **One container.** The selected row gets `fill.selected`. The row below the pointer gets `fill.hover`, which is a smaller step. There is no card around a group of rows and there are no divider lines.
6. **Needs you does not change the size.** A session that needs the user shows the disc, its reason in `text.primary` instead of `text.muted`, and the count in the header. It sorts first in its worktree. It does not add a line, a pill or a weight.
7. **A detail belongs to its row.** A detail such as "12 of 40 tests" is the secondary text of the row that it describes, for example the shell `dev`.
8. **The secondary text truncates first.** The name truncates only when the secondary text has no space left. The data tail at the right never shrinks.
9. **Colour only for state.** The glyphs have the state colours. All text and icons are neutral.

## Geometry

The level of a row is 0 for a worktree, 1 for a session and 2 for a child. One level is `space.4` (16 px).

| Item | Value |
|---|---|
| Row height | `density.*.row-height`: 32 px comfortable, 24 px compact |
| Two-line row height | `density.*.two-line-row-height`: 44 px comfortable, 36 px compact |
| Header | the row height, then a gap of `space.2` (8 px) |
| Header text x | `density.*.padding`: 12 px comfortable, 8 px compact |
| Slot x | padding + 16 × level |
| Gap between name and secondary text | `space.2` (8 px) |
| Gap between the slot and the text (D1, D3) | `space.1` (4 px) |

## D1 Orca Order (recommended)

The state glyph is on the left of each row. A data tail is on the right. A worktree has two lines. The children of a session close to a count.

| Row | Height comfortable / compact | Text x comfortable / compact | Left slot | Primary | Secondary | Right | Below the pointer only |
|---|---|---|---|---|---|---|---|
| Header | 32 / 24 | 12 / 8 | none | project name, title size and weight | none | needs-you glyph and count, meta size, `text.secondary` | new worktree, close all |
| Worktree | 44 (two lines) / 24 (one line) | 32 / 28 | empty when open; aggregate glyph when closed; caret below the pointer | name, title size and weight | branch icon, branch and the count of changes, meta size, muted. In compact density it is a muted tail on the same line | nothing | menu, new session |
| Agent session | 32 / 24 | 48 / 44 | state glyph | name, row size, title weight | reason, meta size, muted | child count (glyph of the most urgent child and `+N`) when the children are closed, then the age | stop, menu (replace the tail); model, cost and the full reason in a tooltip |
| Shell session | 32 / 24 | 48 / 44 | state glyph | name, row size, monospace | detail (port, progress), muted | age | stop, menu |
| Child | 32 / 24 | 64 / 60 | state glyph | name, row size, muted (`text.primary` if it needs the user) | what it does, muted | age | open |

- The data tail ends `space.1` + `space.1` (8 px) from the right edge of the sidebar. The header count ends at the same edge.
- A Team is one session row with the name "Team" and the count of members as secondary text. Open, its members are child rows. The lead is first and its secondary text starts with "lead".

## D2 Even Rows

Each row has one line and one height. A caret is in a left gutter. The state glyph is in one slot at the right edge.

| Row | Height comfortable / compact | Text x comfortable / compact | Left gutter | Primary | Secondary | Right slot | Below the pointer only |
|---|---|---|---|---|---|---|---|
| Header | 32 / 24 | 12 / 8 | none | project name | none | needs-you glyph and count | new worktree, close all |
| Worktree | 32 / 24 | 28 / 24 | caret, always | name, title size and weight | none (branch and changes in a tooltip) | aggregate glyph, only when closed | menu, new session |
| Agent session | 32 / 24 | 44 / 40 | caret if it has children; for a Team the lead icon, caret below the pointer | name, row size, title weight | reason, muted | state glyph | age, stop, menu |
| Shell session | 32 / 24 | 44 / 40 | none | name, monospace | detail, muted | state glyph | age, stop, menu |
| Child | 32 / 24 | 60 / 56 | none | name, muted | what it does, muted | state glyph | age |

- The right slot starts at x = 244 px and ends 12 px from the right edge.
- A Team is shown as its lead. The other members are child rows of the lead.

## D3 Quiet Two-Level

At rest only the worktrees show. Each worktree shows the sentence of its most urgent session. One worktree opens at a time.

| Row | Height comfortable / compact | Text x comfortable / compact | Left slot | Primary | Secondary | Right | Below the pointer only |
|---|---|---|---|---|---|---|---|
| Header | 32 / 24 | 12 / 8 | none | project name | none | needs-you glyph and count | new worktree |
| Worktree | 44 / 36 (two lines in both densities) | 32 / 28 | aggregate glyph when closed; empty when open; caret below the pointer | name, title size and weight | closed: the sentence of the most urgent session, for example "Claude wants to run npm test" (`text.primary` if it needs the user); open: branch icon, branch and the count of changes | none | menu |
| Agent session | 32 / 24, only when the worktree is open | 48 / 44 | state glyph | name, row size, title weight | reason, muted | none | age, stop, menu |
| Shell session | 32 / 24, only when open | 48 / 44 | state glyph | name, monospace | detail, muted | none | age, stop, menu |
| Child | 32 / 24, only when its parent is open | 64 / 60 | state glyph | name, muted | what it does, muted | none | age, open |

- The second line of a worktree always has its space. When the worktree opens, the line changes its content, not its height.
- A Team in a closed worktree gives the sentence "Team · 4 members · Claude asks for approval". Open, the members are session rows. The lead is first and its secondary text starts with "lead".

## Measured values

Measured in Chromium on the preview page, sidebar 272 px, for `nomux-dark` and `nomux-light` (the values are the same in the two themes). The fonts Inter and JetBrains Mono were not installed on the machine of the measurement, thus the text used Noto Sans and Noto Sans Mono. The page shows frames (b) to (e) in comfortable density only. For the compact values, a copy of the page showed these frames in compact density too.

| | D1 | D2 | D3 |
|---|---|---|---|
| Text edges, comfortable | 32, 48, 64 | 28, 44, 60 | 32, 48, 64 |
| Text edges, compact | 28, 44, 60 | 24, 40, 56 | 28, 44, 60 |
| Header text x | 12 / 8 | 12 / 8 | 12 / 8 |
| Row heights, comfortable | worktree 44, session 32, child 32 | 32 for each row | worktree 44, session 32, child 32 |
| Row heights, compact | 24 for each row | 24 for each row | worktree 36, session 24 |
| Height change between rest, pointer and needs you | none | none | none |
| Rows at rest | 8 | 8 | 4 |
| Height at rest with the header, comfortable | 344 px | 296 px | 216 px |
| Glyphs at rest, header included | 7 | 6 | 5 |
| Hues at rest | 5 | 5 | 4 |
| Reason text truncated at rest, comfortable | none | none | none |

Below the pointer, D2 shows the age and the actions before the right slot. Then the detail of `dev` truncates. The full text is in the tooltip.

## Decisions that differ from the first sketches

- **Text edges 32, 48 and 64 for D1 and D3, not 34, 50 and 66.** The gap between the slot and the text is `space.1` (4 px), the token for the gap between a mark and its label. A gap of 6 px is not on the 4 px grid.
- **The header uses the title weight (500), not 600.** The type scale has no third weight. The header differs from a worktree by its position.
- **The header count is a needs-you glyph and `text.secondary` text, not yellow text.** The state colours are for marks. The check examines them at 3:1, not at 4.5:1 for text.
- **The data tail of D1 and D3 ends 8 px from the right edge, not 12 px.** With 12 px, the reason of the Claude row truncated at comfortable density. The right slot of D2 stays at 244 px.
- **The working glyph does not turn.** The motion tokens are for a state change from 120 ms to 200 ms. A continuous turn needs a decision on a new duration token.
- **The rows show no model names.** The sample data has none, and the brief keeps the model out of the default view.
