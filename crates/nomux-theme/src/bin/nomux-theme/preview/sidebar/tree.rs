//! The rows of the sidebar for one project: the state glyph on the left of
//! each row, a data tail on the right, a worktree of two lines, the children
//! of a session closed to a count.

use super::icon::Icon;
use super::row::{Face, Row, Slot, actions, age, branch, child_count};
use super::sample::{Kind, Project, Session, Worktree};

pub fn rows(project: &Project) -> Vec<Row> {
    project.worktrees.iter().flat_map(worktree).collect()
}

fn worktree(worktree: &Worktree) -> Vec<Row> {
    let (aggregate, caret) = if worktree.open {
        (None, Icon::Open)
    } else {
        (worktree.worst().map(|session| session.state), Icon::Closed)
    };
    let mut rows = vec![Row {
        two_line: true,
        secondary: branch(&worktree.branch_line()),
        hover_tail: actions(&[Icon::New, Icon::Menu]),
        ..Row::new(
            0,
            Face::Worktree,
            worktree.name,
            Slot::glyph(aggregate).with_hover(caret),
        )
    }];
    if worktree.open {
        rows.extend(worktree.ordered().flat_map(session));
    }
    rows
}

fn session(session: &Session) -> Vec<Row> {
    let open = session.open && !session.children.is_empty();
    let state = if session.kind == Kind::Team && open {
        None
    } else {
        Some(session.state)
    };
    let mut tail = String::new();
    if !session.children.is_empty() && !open {
        tail.push_str(&child_count(session));
    }
    tail.push_str(&age(session));
    let mut rows = vec![Row {
        tail,
        hover_tail: actions(&[Icon::Stop, Icon::Menu]),
        ..Row::session(
            1,
            Face::of(session),
            session,
            session.doing,
            Slot::glyph(state),
        )
    }];
    if open {
        rows.extend(session.children.iter().map(child));
    }
    rows
}

fn child(child: &Session) -> Row {
    Row {
        tail: age(child),
        ..Row::session(
            2,
            Face::Child,
            child,
            &child.described(),
            Slot::glyph(Some(child.state)),
        )
    }
}
