//! The row primitives of the sidebar mock-ups: the header and one tree row.
//! The tree composes these; it writes no row markup itself.

use super::super::escape;
use super::super::glyph::{State, glyph};
use super::icon::{Icon, icon};
use super::sample::{Kind, Project, Session};

/// The face of the name of a row.
#[derive(Clone, Copy)]
pub enum Face {
    /// 12 px, title weight, secondary.
    Group,
    /// 13 px, title weight.
    Project,
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
            Self::Group => "group",
            Self::Project => "project",
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

/// The 16 px slot at the left of a row: a glyph or nothing at rest and,
/// optionally, an icon below the pointer. The slot keeps its size in each
/// state.
pub struct Slot {
    rest: String,
    hover: String,
}

impl Slot {
    pub fn glyph(state: Option<State>) -> Self {
        Self {
            rest: state.map(glyph).unwrap_or_default(),
            hover: String::new(),
        }
    }

    pub fn with_hover(mut self, hover: Icon) -> Self {
        self.hover = icon(hover);
        self
    }

    fn render(&self) -> String {
        format!(
            "<span class=\"slot\">{}</span>",
            swap(&self.rest, &self.hover)
        )
    }
}

/// Content at rest that other content replaces below the pointer.
fn swap(rest: &str, hover: &str) -> String {
    if hover.is_empty() {
        return rest.to_owned();
    }
    format!("<span class=\"at-rest\">{rest}</span><span class=\"on-hover\">{hover}</span>")
}

/// One row of the tree.
pub struct Row {
    /// 0 for a worktree, 1 for a session, 2 for a child. A tree of groups
    /// adds the depth of the project.
    pub level: u8,
    pub two_line: bool,
    pub slot: Slot,
    pub name: String,
    pub face: Face,
    /// The secondary text, already escaped: it can hold an icon.
    pub secondary: String,
    /// The secondary text has the text level of the name, because it asks
    /// for the user.
    pub attention: bool,
    pub tail: String,
    pub hover_tail: String,
    pub selected: bool,
    pub hover: bool,
    pub focus: bool,
}

impl Row {
    pub fn new(level: u8, face: Face, name: &str, slot: Slot) -> Self {
        Self {
            level,
            two_line: false,
            slot,
            name: escape(name),
            face,
            secondary: String::new(),
            attention: false,
            tail: String::new(),
            hover_tail: String::new(),
            selected: false,
            hover: false,
            focus: false,
        }
    }

    /// A row that shows a session or a child, with its marks of selection,
    /// pointer and focus.
    pub fn session(level: u8, face: Face, session: &Session, secondary: &str, slot: Slot) -> Self {
        Self {
            secondary: escape(secondary),
            attention: session.state == State::NeedsYou && session.kind != Kind::Team,
            selected: session.selected,
            hover: session.hover,
            focus: session.focus,
            ..Self::new(level, face, session.name, slot)
        }
    }

    pub fn render(&self) -> String {
        let mut classes = format!("tree-row {}", self.face.class());
        for (on, class) in [
            (self.two_line, "two-line"),
            (self.selected, "selected"),
            (self.hover, "hover"),
            (self.focus, "focus"),
            (self.attention, "attention"),
        ] {
            if on {
                classes.push(' ');
                classes.push_str(class);
            }
        }
        let secondary = if self.secondary.is_empty() {
            String::new()
        } else {
            format!("<span class=\"secondary\">{}</span>", self.secondary)
        };
        let tail = if self.tail.is_empty() && self.hover_tail.is_empty() {
            String::new()
        } else {
            format!(
                "<span class=\"tail\">{}</span>",
                swap(&self.tail, &self.hover_tail)
            )
        };
        format!(
            "<div class=\"{classes}\" style=\"--level: {level}\">{slot}<span class=\"text\"><span class=\"primary\">{name}</span>{secondary}</span>{tail}</div>\n",
            slot = self.slot.render(),
            name = self.name,
            level = self.level,
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

/// The actions of a row that appear below the pointer.
pub fn actions(icons: &[Icon]) -> String {
    let icons: String = icons.iter().map(|item| icon(*item)).collect();
    format!("<span class=\"actions\">{icons}</span>")
}

/// The branch line of a worktree, with the branch icon.
pub fn branch(line: &str) -> String {
    format!("{}{}", icon(Icon::Branch), escape(line))
}

/// The header of the sidebar: the one place of the project name, the count
/// of the sessions that need the user and, below the pointer, the actions
/// of the project.
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
        "<div class=\"sidebar-header\"><span class=\"primary\">{}</span><span class=\"tail\">{count}<span class=\"on-hover\">{}</span></span></div>\n",
        escape(project.name),
        actions(&[Icon::New, Icon::Menu])
    )
}
