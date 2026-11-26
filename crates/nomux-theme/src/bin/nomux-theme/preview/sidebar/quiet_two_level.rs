//! Direction D3 "Quiet Two-Level": at rest only the worktrees show, each as
//! two lines with the sentence of its most urgent session. One worktree
//! opens at a time.

use super::icon::Icon;
use super::row::{Face, Row, Slot, actions, age, branch};
use super::sample::{Frame, Kind, Project, Session, Worktree};
use crate::preview::glyph::State;

/// At rest each worktree is closed. In the other frames only the first
/// worktree that the frame opens is open.
pub fn rows(project: &Project, frame: Frame) -> Vec<Row> {
    let open = if frame == Frame::Rest {
        None
    } else {
        project.worktrees.iter().position(|worktree| worktree.open)
    };
    project
        .worktrees
        .iter()
        .enumerate()
        .flat_map(|(index, item)| worktree(item, open == Some(index)))
        .collect()
}

fn worktree(worktree: &Worktree, open: bool) -> Vec<Row> {
    let worst = worktree.worst();
    let caret = if open { Icon::Open } else { Icon::Closed };
    let (state, secondary) = match (open, worst) {
        (false, Some(session)) => (Some(session.state), sentence(session)),
        _ => (None, branch(&worktree.branch_line())),
    };
    let mut rows = vec![Row {
        two_line: true,
        left: Some(Slot::glyph(state).with_hover(caret)),
        secondary,
        attention: !open && state == Some(State::NeedsYou),
        ..Row::new(0, Face::Worktree, worktree.name)
    }];
    if open {
        rows.extend(worktree.ordered().flat_map(session));
    }
    rows
}

/// The second line of a closed worktree: who and what, for example
/// "Claude wants to run npm test".
fn sentence(session: &Session) -> String {
    let text = match (session.kind, session.children.first()) {
        (Kind::Team, Some(lead)) => format!(
            "Team · {} members · {} {}",
            session.children.len(),
            lead.name,
            lead.doing
        ),
        _ => format!("{} {}", session.name, session.doing),
    };
    super::super::escape(&text)
}

/// A Team shows its members as sessions, the lead first.
fn session(session: &Session) -> Vec<Row> {
    if session.kind == Kind::Team {
        return session
            .children
            .iter()
            .map(|member| row(1, member))
            .collect();
    }
    let mut rows = vec![row(1, session)];
    if session.open {
        rows.extend(session.children.iter().map(|child| row(2, child)));
    }
    rows
}

fn row(level: u8, session: &Session) -> Row {
    let face = if level == 2 {
        Face::Child
    } else {
        Face::of(session)
    };
    Row {
        left: Some(Slot::glyph(Some(session.state))),
        hover_tail: format!("{}{}", age(session), actions()),
        ..Row::session(level, face, session, &session.described())
    }
}
