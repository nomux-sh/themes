//! The three directions of the sidebar as static mock-ups: the section
//! "Sidebar directions" of the preview page, and the sidebar of the
//! workspace mock-up. The specification is `docs/sidebar.md`.

mod even_rows;
mod icon;
mod orca_order;
mod quiet_two_level;
mod row;
mod sample;

use nomux_theme::Theme;

use super::glyph::{State, glyph};
use super::{escape, variables};
use row::Row;
pub use sample::Frame;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    OrcaOrder,
    EvenRows,
    QuietTwoLevel,
}

impl Direction {
    pub const ALL: [Self; 3] = [Self::OrcaOrder, Self::EvenRows, Self::QuietTwoLevel];

    const fn class(self) -> &'static str {
        match self {
            Self::OrcaOrder => "orca-order",
            Self::EvenRows => "even-rows",
            Self::QuietTwoLevel => "quiet-two-level",
        }
    }

    const fn title(self) -> &'static str {
        match self {
            Self::OrcaOrder => "D1 Orca Order (recommended)",
            Self::EvenRows => "D2 Even Rows",
            Self::QuietTwoLevel => "D3 Quiet Two-Level",
        }
    }

    const fn summary(self) -> &'static str {
        match self {
            Self::OrcaOrder => {
                "The state glyph is on the left of each row and the data tail on the right; a worktree has two lines and the children of a session close to a count."
            }
            Self::EvenRows => {
                "Each row has one line and one height; carets are in a left gutter and the state glyph is in one slot at the right edge."
            }
            Self::QuietTwoLevel => {
                "At rest only the worktrees show, each with the sentence of its most urgent session; one worktree opens at a time."
            }
        }
    }

    fn rows(self, frame: Frame) -> Vec<Row> {
        let project = frame.project();
        match self {
            Self::OrcaOrder => orca_order::rows(&project),
            Self::EvenRows => even_rows::rows(&project),
            Self::QuietTwoLevel => quiet_two_level::rows(&project, frame),
        }
    }

    /// The sidebar of this direction in one frame. The density comes from
    /// the class of an ancestor.
    pub fn sidebar(self, frame: Frame) -> String {
        let rows: String = self.rows(frame).iter().map(Row::render).collect();
        format!(
            "<div class=\"sidebar {class}\">\n{header}{rows}</div>\n",
            class = self.class(),
            header = row::header(&frame.project()),
        )
    }
}

/// The project rail: one tile for each project, with the glyph of its most
/// urgent state.
pub fn rail() -> String {
    let tiles: String = [
        ("A", Some(State::NeedsYou), true),
        ("B", Some(State::Working), false),
        ("D", Some(State::Error), false),
        ("N", None, false),
    ]
    .iter()
    .map(|(letter, state, selected)| {
        format!(
            "<div class=\"tile{}\">{letter}{}</div>",
            if *selected { " selected" } else { "" },
            state.map(glyph).unwrap_or_default()
        )
    })
    .collect();
    format!("<div class=\"rail\">{tiles}</div>\n")
}

/// The section "Sidebar directions" for one theme: the three directions in
/// both densities, and one frame for each state that a reviewer needs.
pub fn section(theme: &Theme) -> String {
    let directions: String = Direction::ALL
        .iter()
        .map(|direction| article(*direction))
        .collect();
    format!(
        "<section class=\"theme directions\" id=\"sidebar-{id}\" style=\"{variables}\">\n\
         <header>\n<h2>Sidebar directions · {name}</h2>\n<p class=\"meta\">Three ways to order the sidebar, \
         with the same sample data and only the tokens of {id}. The workspace mock-ups below use D1. \
         The specification is docs/sidebar.md.</p>\n</header>\n\
         <div class=\"direction-grid\">\n{directions}</div>\n</section>\n",
        id = theme.id(),
        name = escape(theme.name()),
        variables = variables(theme),
    )
}

fn article(direction: Direction) -> String {
    let rest = direction.sidebar(Frame::Rest);
    let densities: String = [("comfortable", "Comfortable"), ("compact", "Compact")]
        .iter()
        .map(|(class, word)| {
            format!(
                "<figure class=\"{class}\">\n<div class=\"window sidebar-window\">\n{rail}{rest}</div>\n\
                 <figcaption class=\"caption\">(a) Rest, {word} density, with the project rail</figcaption>\n</figure>\n",
                rail = rail(),
            )
        })
        .collect();
    let frames: String = Frame::ALL[1..]
        .iter()
        .map(|frame| {
            format!(
                "<figure class=\"comfortable\">\n<div class=\"window sidebar-frame\">\n{}</div>\n\
                 <figcaption class=\"caption\">{}</figcaption>\n</figure>\n",
                direction.sidebar(*frame),
                frame.caption()
            )
        })
        .collect();
    format!(
        "<article class=\"direction\">\n<header>\n<h3>{title}</h3>\n<p class=\"meta\">{summary}</p>\n</header>\n\
         {densities}{frames}</article>\n",
        title = direction.title(),
        summary = direction.summary(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count(direction: Direction, frame: Frame) -> usize {
        direction.sidebar(frame).matches("class=\"tree-row").count()
    }

    #[test]
    fn each_direction_shows_the_rows_of_its_specification() {
        let expected = [
            // Four worktrees; three open with their four sessions; the child closed.
            (Direction::OrcaOrder, Frame::Rest, 8),
            (Direction::EvenRows, Frame::Rest, 8),
            // Only the four worktrees.
            (Direction::QuietTwoLevel, Frame::Rest, 4),
            // fix-login open with Claude, its child and dev; three worktrees closed.
            (Direction::OrcaOrder, Frame::OpenAndClosed, 7),
            (Direction::EvenRows, Frame::OpenAndClosed, 7),
            (Direction::QuietTwoLevel, Frame::OpenAndClosed, 7),
            // D1: the Team row and four members. D2: the lead and three members.
            // D3: four members. Each with dev and three closed worktrees.
            (Direction::OrcaOrder, Frame::Team, 10),
            (Direction::EvenRows, Frame::Team, 9),
            (Direction::QuietTwoLevel, Frame::Team, 9),
        ];
        for (direction, frame, rows) in expected {
            assert_eq!(count(direction, frame), rows, "{direction:?} in {frame:?}");
        }
    }

    #[test]
    fn the_pointer_does_not_add_or_remove_a_row() {
        for direction in [Direction::OrcaOrder, Direction::EvenRows] {
            assert_eq!(
                count(direction, Frame::Hover),
                count(direction, Frame::Rest)
            );
        }
    }

    #[test]
    fn an_open_parent_shows_no_aggregate_state() {
        // At rest in D1 fix-login is open: its slot is empty, the needs-you
        // glyph is on the Claude row and in the header only.
        let sidebar = Direction::OrcaOrder.sidebar(Frame::Rest);
        assert_eq!(sidebar.matches("glyph needs-you").count(), 2);
    }
}
