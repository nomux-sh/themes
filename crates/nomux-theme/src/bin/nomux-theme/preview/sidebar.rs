//! The sidebar as static mock-ups: the section "Sidebar" of each theme, and
//! the sidebar of the workspace mock-up. The specification is
//! `docs/sidebar.md`.

mod icon;
mod row;
mod sample;
mod tree;

use super::escape;
use super::glyph::{State, glyph};
use row::Row;
pub use sample::Frame;

/// The sidebar in one frame. The density comes from the class of an
/// ancestor.
pub fn sidebar(frame: Frame) -> String {
    let project = frame.project();
    let body = if project.worktrees.is_empty() {
        empty_state()
    } else {
        tree::rows(&project).iter().map(Row::render).collect()
    };
    format!(
        "<div class=\"sidebar\">\n{header}{body}</div>\n",
        header = row::header(&project),
    )
}

/// The text of a project without worktrees, and the action that adds one.
fn empty_state() -> String {
    "<div class=\"empty-state\"><span class=\"primary\">No worktrees</span>\
     <span class=\"secondary\">A worktree holds the Sessions of one branch.</span>\
     <span class=\"button\">New worktree</span></div>\n"
        .to_owned()
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

/// The section "Sidebar" of a theme: each frame in both densities.
pub fn section(theme_name: &str) -> String {
    let frames: String = Frame::ALL.iter().map(|frame| figure(*frame)).collect();
    format!(
        "<div class=\"sidebar-section\">\n<header>\n<h3>Sidebar</h3>\n<p class=\"meta\">The sidebar of {name} \
         in each state, comfortable density on the left and compact density on the right. \
         The specification is docs/sidebar.md.</p>\n</header>\n<div class=\"sidebar-frames\">\n{frames}</div>\n</div>\n",
        name = escape(theme_name),
    )
}

fn figure(frame: Frame) -> String {
    let rail = if frame == Frame::Rest {
        rail()
    } else {
        String::new()
    };
    let windows: String = ["comfortable", "compact"]
        .iter()
        .map(|density| {
            format!(
                "<div class=\"window sidebar-window {density}\">\n{rail}{}</div>\n",
                sidebar(frame)
            )
        })
        .collect();
    format!(
        "<figure>\n<div class=\"density-pair\">\n{windows}</div>\n<figcaption class=\"caption\">{}</figcaption>\n</figure>\n",
        frame.caption()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count(frame: Frame) -> usize {
        sidebar(frame).matches("class=\"tree-row").count()
    }

    #[test]
    fn each_frame_shows_the_rows_of_the_specification() {
        let expected = [
            // Four worktrees; three open with their four sessions; the child closed.
            (Frame::Rest, 8),
            // fix-login open with Claude, its child and dev; three worktrees closed.
            (Frame::OpenAndClosed, 7),
            // The Team row, its four members and dev; three worktrees closed.
            (Frame::Team, 10),
            // upgrade-deps closed; billing-export with Codex and Gemini CLI.
            (Frame::WithoutProcess, 8),
            (Frame::Empty, 0),
        ];
        for (frame, rows) in expected {
            assert_eq!(count(frame), rows, "{frame:?}");
        }
    }

    #[test]
    fn the_pointer_does_not_add_or_remove_a_row() {
        assert_eq!(count(Frame::Hover), count(Frame::Rest));
    }

    #[test]
    fn an_open_parent_shows_no_aggregate_state() {
        // At rest fix-login is open: its slot is empty, the needs-you glyph
        // is on the Claude row and in the header only.
        let sidebar = sidebar(Frame::Rest);
        assert_eq!(sidebar.matches("glyph needs-you").count(), 2);
    }

    #[test]
    fn each_state_has_its_own_shape() {
        let mut shapes: Vec<String> = State::ALL
            .iter()
            .map(|state| {
                let svg = glyph(*state);
                svg[svg.find('>').expect("an svg element")..].to_owned()
            })
            .collect();
        shapes.sort();
        shapes.dedup();
        assert_eq!(shapes.len(), State::ALL.len());
    }
}
