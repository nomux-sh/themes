//! The row primitives of the sidebar mock-ups: the header and one tree row.
//! Each direction composes these; no direction writes row markup itself.

use super::super::escape;
use super::super::glyph::{State, glyph};
use super::icon::{Icon, icon};
use super::sample::{Kind, Project, Session};

/// The face of the name of a row.
#[derive(Clone, Copy)]
pub enum Face {
    /// 13 px, title weight.
    Worktree,
    /// 12 px, title weight.
    Agent,
    /// 12 px, monospace.
    Shell,
    /// 12 px, row weight, muted.
    Child,
}

impl Face {
    const fn class(self) -> &'static str {
        match self {
            Self::Worktree => "worktree",
            Self::Agent => "agent",
            Self::Shell => "shell",
            Self::Child => "child",
        }
    }

    pub fn of(session: &Session) -> Self {
        match session.kind {
            Kind::Shell => Self::Shell,
            Kind::Agent | Kind::Team => Self::Agent,
        }
    }
}

/// The content of a 16 px slot: one item at rest and, optionally, another
/// item below the pointer. The slot keeps its size in each state.
#[derive(Default)]
pub struct Slot {
    pub rest: String,
    pub hover: String,
}

impl Slot {
    pub fn glyph(state: Option<State>) -> Self {
        Self {
            rest: state.map(glyph).unwrap_or_default(),
            hover: String::new(),
        }
    }

    pub fn icon(rest: Icon) -> Self {
        Self {
            rest: icon(rest),
            hover: String::new(),
        }
    }

    pub fn with_hover(mut self, hover: Icon) -> Self {
        self.hover = icon(hover);
        self
    }

    fn render(&self, side: &str) -> String {
        if self.hover.is_empty() {
            return format!("<span class=\"slot {side}\">{}</span>", self.rest);
        }
        format!(
            "<span class=\"slot {side}\"><span class=\"at-rest\">{}</span><span class=\"on-hover\">{}</span></span>",
            self.rest, self.hover
        )
    }
}

/// One row of the tree.
pub struct Row {
    /// 0 for a worktree, 1 for a session, 2 for a child.
    pub level: u8,
    pub two_line: bool,
    pub left: Option<Slot>,
    pub name: String,
    pub face: Face,
    /// The secondary text, already escaped: it can hold an icon.
    pub secondary: String,
    /// The secondary text has the text level of the name, because it asks
    /// for the user.
    pub attention: bool,
    pub tail: String,
    pub hover_tail: String,
    pub right: Option<Slot>,
    pub selected: bool,
    pub hover: bool,
}

impl Row {
    pub fn new(level: u8, face: Face, name: &str) -> Self {
        Self {
            level,
            two_line: false,
            left: None,
            name: escape(name),
            face,
            secondary: String::new(),
            attention: false,
            tail: String::new(),
            hover_tail: String::new(),
            right: None,
            selected: false,
            hover: false,
        }
    }

    /// A row that shows a session or a child, with its marks of selection
    /// and pointer.
    pub fn session(level: u8, face: Face, session: &Session, secondary: &str) -> Self {
        Self {
            secondary: escape(secondary),
            attention: session.state == State::NeedsYou && session.kind != Kind::Team,
            selected: session.selected,
            hover: session.hover,
            ..Self::new(level, face, session.name)
        }
    }

    pub fn render(&self) -> String {
        let mut classes = format!("tree-row level-{} {}", self.level, self.face.class());
        for (on, class) in [
            (self.two_line, "two-line"),
            (self.selected, "selected"),
            (self.hover, "hover"),
            (self.attention, "attention"),
        ] {
            if on {
                classes.push(' ');
                classes.push_str(class);
            }
        }
        let left = self
            .left
            .as_ref()
            .map(|slot| slot.render("left"))
            .unwrap_or_default();
        let right = self
            .right
            .as_ref()
            .map(|slot| slot.render("right"))
            .unwrap_or_default();
        let secondary = if self.secondary.is_empty() {
            String::new()
        } else {
            format!("<span class=\"secondary\">{}</span>", self.secondary)
        };
        let tail = match (self.tail.is_empty(), self.hover_tail.is_empty()) {
            (true, true) => String::new(),
            (false, true) => format!("<span class=\"tail\">{}</span>", self.tail),
            _ => format!(
                "<span class=\"tail\"><span class=\"at-rest\">{}</span><span class=\"on-hover\">{}</span></span>",
                self.tail, self.hover_tail
            ),
        };
        format!(
            "<div class=\"{classes}\">{left}<span class=\"text\"><span class=\"primary\">{name}</span>{secondary}</span>{tail}{right}</div>\n",
            name = self.name
        )
    }
}

/// The age of a session, in the tail of a row.
pub fn age(session: &Session) -> String {
    format!("<span class=\"age\">{}</span>", session.age)
}

/// The count of the closed children of a session: the glyph of the most
/// urgent child and `+N`.
pub fn child_count(session: &Session) -> String {
    format!(
        "<span class=\"count\">{}+{}</span>",
        session.worst_child().map(glyph).unwrap_or_default(),
        session.children.len()
    )
}

/// The actions of a session that appear below the pointer.
pub fn actions() -> String {
    format!(
        "<span class=\"actions\">{}{}</span>",
        icon(Icon::Stop),
        icon(Icon::Menu)
    )
}

/// The branch line of a worktree, with the branch icon.
pub fn branch(line: &str) -> String {
    format!("{}{}", icon(Icon::Branch), escape(line))
}

/// The header of the sidebar: the one place of the project name, and the
/// count of the sessions that need the user.
pub fn header(project: &Project) -> String {
    let needs_you = project.needs_you();
    let count = if needs_you == 0 {
        String::new()
    } else {
        format!(
            "<span class=\"needs-you-count\">{}{needs_you} needs you</span>",
            glyph(State::NeedsYou)
        )
    };
    format!(
        "<div class=\"sidebar-header\"><span class=\"primary\">{}</span>{count}</div>\n",
        escape(project.name)
    )
}
