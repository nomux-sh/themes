# Sidebar

This document specifies the sidebar of the nomux desktop client. An engineer can build the sidebar in a native toolkit from this document alone.

Each value is a token from [tokens.md](tokens.md). The pixel values are the values of the base themes. A theme can change a token, thus the client reads the token and does not use the pixel value.

The preview page shows the sidebar in the section "Sidebar" of each theme, in the two densities and in seven frames. The command `nomux-theme preview` writes the page.

## Structure

The sidebar shows one project. The project rail at its left selects the project.

From top to bottom the sidebar has:

1. The **header**: the project name. The project name is only in the header. There is no project row.
2. The **tree**, with three levels:
   - level 0: a **worktree**,
   - level 1: a **Session** of the worktree: an agent, a shell or a Team,
   - level 2: a **child** of a Session: the child work of an agent, or a member of a Team.

A row has one job: a worktree row identifies a worktree, a Session row reports one Session, a child row reports one child.

The row of a group of projects is not decided. The preview page shows three alternatives in the section "Groups (alternatives)" of nomux Dark and nomux Light. This document will specify the alternative that is chosen.

## Geometry

The x positions are measured from the left edge of the sidebar. A level `L` is 0, 1 or 2.

| Item | Token | Comfortable | Compact |
|---|---|---|---|
| Inner padding of the sidebar, on each side | `space.1` | 4 px | 4 px |
| Header height | `density.*.row-height` | 32 px | 24 px |
| Gap below the header | `space.2` | 8 px | 8 px |
| Header text x | `density.*.padding` | 12 px | 8 px |
| Row height: Session and child | `density.*.row-height` | 32 px | 24 px |
| Row height: worktree (two lines) | `density.*.two-line-row-height` | 44 px | 40 px |
| Indent of one level | `space.4` | 16 px | 16 px |
| Slot x of level `L` | `density.*.padding` + `space.4` × `L` | 12, 28, 44 px | 8, 24, 40 px |
| Slot size | `size.icon` | 16 px | 16 px |
| Glyph size, centred in the slot | `size.mark` | 8 px | 8 px |
| Gap between the slot and the text | `space.1` | 4 px | 4 px |
| Text x of level 0, 1, 2 | slot x + `size.icon` + `space.1` | 32, 48, 64 px | 28, 44, 60 px |
| Gap between the name and the secondary text | `space.2` | 8 px | 8 px |
| Gap before the tail | `space.2` | 8 px | 8 px |
| Right edge of the tail and of the header count | `space.1` + `space.1` from the edge | 8 px | 8 px |
| Corner radius of a row | `radius.button` | | |
| Icons in the slot and in the tail | `size.icon` | 16 px | 16 px |
| Branch icon on the second line | `font.meta.size` | 11 px | 11 px |

The left padding of a row is `density.*.padding` − `space.1` + `space.4` × `L`, because the sidebar padding gives the other `space.1`. The right padding of a row is `space.1`.

A worktree has two lines in both densities. The first line holds the slot, the name and the tail. The second line holds the branch line below the name. The two lines are centred as one block in the row. At 40 px in compact density, 3.5 px are above the name and below the branch line.

## Rows

### Element order

Each row has the same order from left to right: slot, name, secondary text, tail. An element that has no content takes no space, but the slot always takes its 16 px.

| Row | Height | Text x | Slot at rest | Name | Secondary text | Tail at rest | Below the pointer |
|---|---|---|---|---|---|---|---|
| Header | `row-height` | `padding` | none | project name: `font.title.size`, `font.title.weight` | none | needs-you glyph and "N needs you": `font.meta.*`, `text.secondary`; only if N > 0 | the count stays; then the actions: new worktree, menu |
| Worktree | `two-line-row-height` | level 0 | open: empty. Closed: the aggregate glyph | worktree name: `font.title.size`, `font.title.weight`, `text.primary` | line 2: branch icon, branch, " · N changes": `font.meta.*`, `text.muted` | none | slot: the caret (open or closed). Tail: new Session, menu |
| Agent Session | `row-height` | level 1 | state glyph | agent name: `font.row.size`, `font.title.weight`, `text.primary` | reason: `font.meta.*`, `text.muted` | closed children: the glyph of the most urgent child and "+N". Then the age | stop, menu replace the tail |
| Shell Session | `row-height` | level 1 | state glyph | shell name: `font.mono`, `font.row.size`, `font.code.weight` | detail, for example "port 3000 · 12 of 40 tests": `font.meta.*`, `text.muted` | age | stop, menu replace the tail |
| Team | `row-height` | level 1 | closed: state glyph. Open: empty | "Team": like an agent name | "N members" | closed: the glyph of the most urgent member and "+N". Then the age | stop, menu replace the tail |
| Child | `row-height` | level 2 | state glyph | name: `font.row.size`, `font.row.weight`, `text.muted` | what it does: `font.meta.*`, `text.muted` | age | none |

- The tail uses `font.meta.*`, `text.muted` and tabular figures, thus the ages make a column.
- The actions use `text.secondary`. Between two actions the gap is `space.2`.
- A member of a Team is a child row. The leader is the first member. Its secondary text starts with "leader · ".
- A child of an agent is open only when the user opens it. At rest the agent shows the count of its children in the tail.

### Below the pointer only

These items show only when the pointer is on the row. The row does not change its height or its text edges.

- Header: new worktree and menu, after the count. The menu holds "close all".
- Worktree: the caret replaces the aggregate glyph in the slot. New Session and menu appear in the tail.
- Session: stop and menu replace the tail. The full reason, the model and the cost are in the tooltip.
- Each row: the tooltip shows the full name and the full secondary text.

## State glyphs

Each state has its own shape, thus a state does not depend on colour only. The glyph is drawn in a view box of 16 × 16 units at `size.mark` (8 px). The open shapes have a stroke of 3 units (1.5 px) with round caps and joins.

| State | Shape | Path in the 16 × 16 view box | Colour |
|---|---|---|---|
| Needs you | filled disc | circle at (8, 8), radius 6.5, filled | `state.needs-you` |
| Error | cross | `M3.5 3.5l9 9m0-9-9 9` | `state.error` |
| Working | arc of three quarters, turning | `M8 2a6 6 0 1 1-6 6` | `state.working` |
| Done, not seen | check | `M2.5 8.5l3.5 3.5 7.5-8` | `state.done` |
| Idle | hollow ring | circle at (8, 8), radius 5.5 | `state.idle` |
| No process | horizontal dash | `M3 8h10` | `state.idle` |

The sequence of the table is the priority: needs you is the most urgent, no process is the least urgent.

### The working glyph turns

- The working glyph turns clockwise around its centre. One turn takes `motion.indicator` (1000 ms) at a constant speed. It does not use `motion.easing`, because it is not a transition.
- When the user asks for reduced motion, the glyph does not turn. It shows the static arc: the opening is in the top-left quarter.
- The turn does not change the size of the glyph or of the slot.

### Rules for the states

- **Needs you.** The row shows the disc, and its reason in `text.primary` instead of `text.muted`. The header shows the count. The row sorts first in its worktree. The row does not get a line, a pill, a badge or a weight.
- **Done, not seen, and done and seen.** The check means that the agent is done and the user did not see the result. When the user opens the Session, the state becomes idle: the hollow ring. The reason text stays. Thus a green mark always asks for the attention of the user, and colour carries only state.
- **No process: dormant.** A dormant Session has no process, for example after a restart of the Host. It shows the dash in `state.idle` and the secondary text "dormant · restores on open". The dash makes it different from idle without a new colour.
- **No process: hibernated.** A hibernated agent has no process until the user opens it. It shows the same dash and the secondary text "hibernated". The secondary text gives the cause. The glyph only says that no process runs.
- **Selected and needs you.** The row gets `fill.selected` and keeps the disc and the `text.primary` reason. A selection never hides a state.
- **Error.** The cross, and the reason, for example "build failed", in `text.muted`.

## Aggregate state

- Only a closed parent shows an aggregate state.
- A closed worktree shows the glyph of the most urgent state of its Sessions in its slot.
- A closed agent with children, or a closed Team, shows the glyph of its most urgent child and "+N" in its tail. Its own state stays in its slot.
- An open parent shows no aggregate. An open worktree has an empty slot. An open Team has an empty slot: its members show their states.
- The project tile in the rail shows the most urgent state of the project.

## Sort

- In a worktree, the Sessions that need the user come first. The other Sessions keep their sequence of creation.
- The worktrees keep their sequence. A state change does not move a worktree, thus a row does not jump below the pointer.
- The children keep their sequence. The leader of a Team is first.

## Selection, pointer and keyboard focus

- The selected row gets `fill.selected`. The row below the pointer gets `fill.hover`, a smaller step. Both are tints of `text.primary` painted over `surface.base`. The text colour does not change.
- A fill changes in `motion.fast` with `motion.easing`.
- There is no card around a group of rows. Only the selected row and the row below the pointer get a fill.
- The row that has the keyboard focus gets a ring of `focus.ring-width` (2 px) in `focus.ring`, inside the row and with `radius.button`. The ring is inside the row, thus it does not cover a neighbour. Focus and selection are independent: one row can have the focus while another row is selected.

## Long text

- The secondary text truncates first, with an ellipsis. The name truncates only when no space is left for the secondary text.
- The tail never shrinks and never truncates.
- The name and the branch line of a worktree truncate independently.
- The project name in the header truncates. The count never truncates.
- The tooltip shows the full text.
- With the sample data at rest, no text truncates at 272 px in either density.

## Empty project

A project without worktrees shows the header and then:

1. "No worktrees", in `font.row.*` and `text.secondary`.
2. "A worktree holds the Sessions of one branch.", in `font.meta.*` and `text.muted`.
3. The button "New worktree": `surface.button`, `density.*.control-height`, `radius.button`.

The text starts at the header text x. The gap between the lines is `space.1`. The button is `space.2` below the text, and the block ends `space.3` above the bottom of the sidebar.

## Do not

- Do not write type words such as "Project", "Worktree" or "Session" in a row. The shape and the position give the type.
- Do not put a card or a border around a group of rows. Do not use divider lines.
- Do not show a state mark on an open parent.
- Do not change the weight, the size or the height of a row for a state.
- Do not use a state colour for text, for an icon or for decoration. Only the glyphs have state colours.
- Do not add a second accent.
- Do not show the project name in a row.

## Measured values

Measured in Chromium on the preview page, for `nomux-dark` and `nomux-light`, sidebar 272 px. The values are the same in the two themes. The fonts Inter and JetBrains Mono were not installed on the machine of the measurement, thus the text used Noto Sans and Noto Sans Mono.

| | Comfortable | Compact |
|---|---|---|
| Header text x | 12 | 8 |
| Text x of level 0, 1, 2 | 32, 48, 64 | 28, 44, 60 |
| Header height | 32 | 24 |
| Worktree height | 44 | 40 |
| Session, Team and child height | 32 | 24 |
| Right edge of each tail | 8 from the edge | 8 from the edge |
| Height change between the frames for one row kind | none | none |
| Sidebar height at rest (a) and below the pointer (b) | 352 and 352 | 296 and 296 |
| Text truncated at rest | none | none |
| Text truncated in each frame | only frame (e), the long text | only frame (e), the long text |
