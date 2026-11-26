# Sidebar

This document specifies the sidebar of the nomux desktop client. An engineer can build the sidebar in a native toolkit from this document alone.

Each value is a token from [tokens.md](tokens.md). The pixel values are the values of the base themes. A theme can change a token, thus the client reads the token and does not use the pixel value.

The preview page shows the sidebar in the section "Sidebar" of each theme, in the two densities and in seven frames, and the project rail in the part "Groups" of the section, in ten frames. The command `nomux-theme preview` writes the page.

## Structure

The sidebar shows one project. The project rail at its left selects the project.

From top to bottom the sidebar has:

1. The **header**: the project name. The project name is only in the header. There is no project row.
2. The **tree**, with three levels:
   - level 0: a **worktree**,
   - level 1: a **Session** of the worktree: an agent, a shell or a Team,
   - level 2: a **child** of a Session: the child work of an agent, or a member of a Team.

A row has one job: a worktree row identifies a worktree, a Session row reports one Session, a child row reports one child.

The project rail holds the projects and the groups of projects. The section [Project rail](#project-rail) specifies it.

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
- The project tile in the rail shows the most urgent state of the project. A closed group tile shows the most urgent state of each project in the group and in its groups. An open group tile shows the open mark and no state.

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

## Project rail

The rail is the column at the left of the sidebar. It holds one tile for each project and each group of projects, in the sequence of the user. A group holds projects and groups, to any depth. The rail selects the project that the sidebar shows.

### Groups open in place

- A click on a group tile opens the group in place. The group tile becomes the first tile of a **strip**: a rounded area behind the tile. The children of the group follow below it, in the strip.
- A group in an open group opens in the same way, inside the strip of its parent. Thus the strips nest.
- The strip grows down as the content grows. The tiles below it move down.
- There is no step into a group, no back tile and no path. The rail stays one column.
- A click on the tile of an open group closes it. The strip and its children go away. The groups in it keep their state: when the user opens the group again, they show as before.

### Geometry

The x positions are measured from the left edge of the rail. `T` is the tile size: `density.*.control-height`. `D` is the depth of an open group: 1 for a group at the top of the rail.

| Item | Token | Comfortable | Compact |
|---|---|---|---|
| Tile size, for each tile and each depth | `density.*.control-height` | 28 px | 24 px |
| Tile x, for each tile and each depth | `space.3` | 12 px | 12 px |
| Tile centre x | `space.3` + `T` / 2 | 26 px | 24 px |
| Rail width, closed | `T` + 2 × `space.3` | 52 px | 48 px |
| Rail width, expanded (default) | 12 × `space.4` | 192 px | 192 px |
| Padding of the rail, top and bottom | `space.2` | 8 px | 8 px |
| Gap between two tiles | `space.2` | 8 px | 8 px |
| Strip of depth `D`: x | `space.1` / 2 × `D` | 2, 4, 6, 8, 10 px | 2, 4, 6, 8, 10 px |
| Strip of depth `D`: width (closed rail) | rail width − 2 × strip x | 48, 44, 40, 36, 32 px | 44, 40, 36, 32, 28 px |
| Strip: space above its first tile and below its last tile | `space.1` | 4 px | 4 px |
| Strip corner radius | `radius.raised` | 8 px | 8 px |
| Tile corner radius | `radius.button` | 6 px | 6 px |
| Label of a project tile, initial of an open group | `font.title.size`, `font.title.weight` | 13 px | 13 px |
| Corner mark: state mark and open mark | `size.corner-mark` | 10 px | 10 px |
| Ring that cuts a corner mark out of the tile | `focus.ring-width` | 2 px | 2 px |

- **Inset rule.** Each level of strips is `space.1` / 2 narrower on each side than its parent strip. The tiles do not move: each tile has the same x and the same size at each depth, thus all tiles have one centre line. From depth 6, a strip has the width of depth 5.
- A strip takes the `space.1` above its first tile and below its last tile from the gap between two tiles. Thus a group tile does not move when the group opens, and between a strip and the next tile 4 px stay.
- **Fill.** A strip of depth `D` is `D` layers of `fill.level` over `surface.sunken`. Thus each level is one step lighter in a dark theme and one step darker in a light theme. From depth 6, a strip has the fill of depth 5. The strip has no border and no shadow: the levels differ by the fill only.
- **Level contrast.** The theme check makes sure that each level of the fill, to depth 5, has a contrast of 1.1:1 or more to the level below it, and that `text.primary` and `text.secondary` have 4.5:1 or more on each level. Thus the names of the expanded rail stay readable at depth 5.
- The rail is `surface.sunken`.

### Tiles

| Tile | Face | Background | Corner mark |
|---|---|---|---|
| Project | the label, `text.secondary` | `surface.button` | the most urgent state of the project |
| Closed group | the **mini grid**: one square for each of the first four children, filled for a project, hollow for a group | `surface.button` | the aggregate state |
| Empty closed group | no face | `surface.button` | none |
| Open group | the **initial** of the group name, `text.secondary` | `surface.button` | the open mark, at the bottom right |

- **Mini grid.** 2 × 2 squares of (`size.icon` − `space.1`) / 2 (6 px), with a gap of `space.1`: 16 px in all. A filled square is `text.muted` with the radius `radius.button` / 4. A hollow square has a line of `line.width` in `text.muted`. The squares fill from the top left, by rows. The grid shows what the group holds and how much (one to four), and its shape is different from a label, thus a group tile never looks like a project tile.
- **Initial.** The first character of the group name, in upper case, with the font of a project label. It has one character and a project label has two, thus an open group never looks like a project tile. The initial tells which group is open without the pointer. Two groups can have the same initial, for example Web and Work. Then the depth of the strip, the projects in it and the tooltip tell them apart. The initial does not change to a second letter, because it must stay the first letter of the name that the user reads in the expanded rail.
- **Open and closed.** A closed group is a tile with the mini grid. An open group is a tile with its initial and the open mark, and the strip behind it and its children. The open mark is in the tile and takes no row.
- **Corner mark.** A mark of `size.corner-mark` (10 px) at a corner of the tile: its box extends `space.1` / 2 (2 px) past the edge of the tile on each side of the corner. A ring of `focus.ring-width` (2 px) in `surface.sunken` cuts the mark out of the tile, at each depth. Thus each mark keeps the contrast that the theme check gives on `surface.sunken`.
  - The **state mark** is at the top right corner: the state glyph of [State glyphs](#state-glyphs), drawn at `size.corner-mark`. The rules of [Aggregate state](#aggregate-state) apply: an open group shows no state mark.
  - The **open mark** is at the bottom right corner of an open group tile: the open caret, the path `M3 6l5 5 5-5` in the 16 × 16 view box with a stroke of 3 units and round caps and joins, in `text.muted`. It is neutral: it is not a state.

### Label of a project tile

The label has two characters. The rule:

1. The words of the name are separated by `-`, `_`, `.` and spaces.
2. Remove the leading words that all projects of the same group share, and each word that is a word of the name of a group that holds the project. The words of a group name are separated in the same way. If no word stays, the first word after the shared words stays.
3. One word left: its first letter in upper case and its second letter in lower case. Two words or more: the first letter of the first two words, in upper case.
4. If an earlier project in the same group has the same label, the second character becomes the next letter of the remaining words that gives a free label.

With the sample: acme-web, acme-admin and acme-mobile become "We", "Ad" and "Mo"; acme-shop-2019 in the group 2019 becomes "Sh"; globex-api becomes "GA"; dotfiles and docs in one group become "Do" and "Dc". A group name that is longer, for example "Acme Corporation customer platform" for Acme, gives the same labels, because the rule compares words. The label does not depend on the width of the rail: the narrow and the expanded rail show the same label. The label has no colour of its own: colour carries only state. The tooltip and the expanded rail show the full name.

### Selection, pointer and keyboard focus

- **Selection.** The tile of the selected project gets `fill.selected` and a label in `text.primary`. A bar marks it at the left edge of the rail: `focus.ring-width` (2 px) wide, `size.icon` (16 px) high, centred on the tile, in `accent`, with the corners at the right rounded by `focus.ring-width`. This is the mark of the active Tab, turned on its side.
- When the selected project is in a closed group, the group tile gets the selection: the fill and the bar. Thus the bar always shows where the selected project is.
- **Pointer.** The tile below the pointer gets `fill.hover`. After the tooltip delay of the client, a tooltip shows the full name of a project, or the name of a group and its count of projects, for example "Personal 3 projects", or "empty". The tooltip is `surface.floating`, `radius.floating`, the shadow `shadow.floating`, `font.row.*` in `text.primary`; the count is `font.meta.*` in `text.muted`. It starts `space.1` right of the rail edge and is centred on the tile. The expanded rail shows no tooltip.
- **Focus.** The tile with the keyboard focus gets a ring of `focus.ring-width` in `focus.ring`, `line.width` outside the tile. Focus and selection are independent.
- A fill changes in `motion.fast` with `motion.easing`.

### Motion

- A group opens and closes with a change of height: the strip and the tiles below it move in `motion.slow` (200 ms) with `motion.easing`.
- When the user asks for reduced motion, the change is immediate.
- The preview page has no script, thus it shows the end states: frames (a) and (b) show Personal closed and open.

### Expanded rail

- The user drags the right edge of the rail to make it wide. Then each tile has its name at its right; the default width is 12 × `space.4` (192 px). A drag below that width closes the rail to its closed width.
- The tiles, their faces, their labels, the strips and the inset rule do not change when the rail becomes wide. The strips are as wide as the rail minus their x on each side.
- The name starts `space.2` right of the tile and ends `space.2` before the right edge of its strip. A project name is `font.row.*`; a group name is `font.row.size` and `font.title.weight`. Both are `text.secondary`; the name of the selected project is `text.primary`.
- A long name truncates with an ellipsis. The full name is in the tooltip of the name.

### Scroll

- When the tiles are taller than the window, the rail scrolls vertically. The tiles are cut at the bottom edge of the rail: there is no fade and no shadow.
- The rail has no scroll bar at rest. A thin overlay scroll bar shows while the rail scrolls.
- When the selected project changes from outside the rail, the rail scrolls the least that makes its tile visible.

### Drag targets (not built)

- A project dropped on a group tile goes into the group, after its last child.
- A project or a group dropped between two tiles moves there. A drop between two tiles of a strip puts it in that group.
- The preview page does not show a drag.

## Do not

- Do not write type words such as "Project", "Worktree" or "Session" in a row. The shape and the position give the type.
- Do not put a card or a border around a group of rows of the tree. Do not use divider lines. The strip of an open group in the rail is not a row: it is specified in [Project rail](#project-rail).
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

The project rail, measured in the same way in each of the seven themes, in the ten frames of the part "Groups":

| | Comfortable | Compact |
|---|---|---|
| Tile size in each frame | 28 × 28 | 24 × 24 |
| Tile centre x in each frame, at each depth, closed and expanded | 26 | 24 |
| Rail width closed | 52 | 48 |
| Rail width expanded | 192 | 192 |
| Strip x and width at depth 1 to 5, closed rail | 2 / 48, 4 / 44, 6 / 40, 8 / 36, 10 / 32 | 2 / 44, 4 / 40, 6 / 36, 8 / 32, 10 / 28 |
| State mark and open mark, drawn size | 10 × 10 | 10 × 10 |

The contrast between adjacent levels of the strip fill, level 0 (`surface.sunken`) to level 5, from the theme values with the formula of WCAG 2.x, and the contrast of `text.secondary` on level 5:

| Theme | `fill.level` | Level 1, 2, 3, 4, 5 to the level below | `text.secondary` on level 5 |
|---|---|---|---|
| base-dark | `#e8e8e810` | 1.14, 1.17, 1.19, 1.19, 1.18 | 4.93 |
| base-light | `#1c1c1c10` | 1.13, 1.13, 1.12, 1.12, 1.12 | 4.80 |
| catppuccin-mocha | `#cdd6f410` | 1.13, 1.16, 1.17, 1.17, 1.16 | 5.14 |
| gruvbox-dark | `#ebdbb210` | 1.15, 1.17, 1.17, 1.17, 1.16 | 4.86 |
| nomux-dark | `#e6e8eb10` | 1.14, 1.18, 1.19, 1.19, 1.18 | 4.97 |
| nomux-light | `#1a1d2110` | 1.13, 1.13, 1.12, 1.12, 1.12 | 4.66 |
| tokyo-night | `#c0caf50e` | 1.11, 1.13, 1.13, 1.13, 1.13 | 4.68 |
