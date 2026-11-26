//! Direction D2 "Even Rows": each row is one line of one height. Carets sit
//! in a left gutter, the state glyph in one slot at the right edge.

use super::icon::Icon;
use super::row::{Face, Row, Slot, actions, age};
use super::sample::{Kind, Project, Session, Worktree};

pub fn rows(project: &Project) -> Vec<Row> {
    project.worktrees.iter().flat_map(worktree).collect()
}

fn caret(open: bool) -> Icon {
    if open { Icon::Open } else { Icon::Closed }
}

fn worktree(worktree: &Worktree) -> Vec<Row> {
    let aggregate = if worktree.open {
        None
    } else {
        worktree.worst().map(|session| session.state)
    };
    let mut rows = vec![Row {
        left: Some(Slot::icon(caret(worktree.open))),
        right: Some(Slot::glyph(aggregate)),
        ..Row::new(0, Face::Worktree, worktree.name)
    }];
    if worktree.open {
        rows.extend(worktree.ordered().flat_map(session));
    }
    rows
}

/// A Team is shown as its lead, with the other members as its children.
fn session(session: &Session) -> Vec<Row> {
    let (shown, children, gutter): (&Session, &[Session], Slot) = match session.kind {
        Kind::Team => (
            &session.children[0],
            &session.children[1..],
            Slot::icon(Icon::Lead).with_hover(caret(session.open)),
        ),
        _ if session.children.is_empty() => (session, &[], Slot::default()),
        _ => (session, &session.children, Slot::icon(caret(session.open))),
    };
    let mut rows = vec![Row {
        left: Some(gutter),
        right: Some(Slot::glyph(Some(shown.state))),
        hover_tail: format!("{}{}", age(shown), actions()),
        selected: session.selected,
        hover: session.hover,
        ..Row::session(1, Face::of(shown), shown, shown.doing)
    }];
    if session.open {
        rows.extend(children.iter().map(child));
    }
    rows
}

fn child(child: &Session) -> Row {
    Row {
        left: Some(Slot::default()),
        right: Some(Slot::glyph(Some(child.state))),
        hover_tail: age(child),
        ..Row::session(2, Face::Child, child, child.doing)
    }
}
