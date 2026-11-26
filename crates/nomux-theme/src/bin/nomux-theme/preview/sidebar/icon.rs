//! The neutral icons of the sidebar rows. They have the colour of the text
//! around them, never a state colour.

#[derive(Clone, Copy)]
pub enum Icon {
    /// The caret of a row that is open.
    Open,
    /// The caret of a row that is closed.
    Closed,
    /// The branch of a worktree.
    Branch,
    /// Add a worktree or a Session.
    New,
    /// Stop the Session.
    Stop,
    /// The menu of the row.
    Menu,
}

impl Icon {
    const fn parts(self) -> (&'static str, &'static str) {
        match self {
            Self::Open => ("caret", "<path d=\"M3 6l5 5 5-5\"/>"),
            Self::Closed => ("caret", "<path d=\"M6 3l5 5-5 5\"/>"),
            Self::Branch => (
                "branch",
                "<circle cx=\"4.5\" cy=\"3.5\" r=\"1.75\"/><circle cx=\"4.5\" cy=\"12.5\" r=\"1.75\"/>\
                 <circle cx=\"11.5\" cy=\"5.5\" r=\"1.75\"/><path d=\"M4.5 5.25v5.5M11.5 7.25c0 2.5-3 2.5-7 3.5\"/>",
            ),
            Self::New => ("action", "<path d=\"M8 3v10M3 8h10\"/>"),
            Self::Stop => (
                "action",
                "<rect x=\"3.5\" y=\"3.5\" width=\"9\" height=\"9\" rx=\"1.5\" class=\"filled\"/>",
            ),
            Self::Menu => (
                "action",
                "<circle cx=\"3\" cy=\"8\" r=\"1.5\" class=\"filled\"/><circle cx=\"8\" cy=\"8\" r=\"1.5\" class=\"filled\"/>\
                 <circle cx=\"13\" cy=\"8\" r=\"1.5\" class=\"filled\"/>",
            ),
        }
    }
}

pub fn icon(icon: Icon) -> String {
    let (class, shape) = icon.parts();
    format!(
        "<svg class=\"icon-svg {class}\" viewBox=\"0 0 16 16\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.5\" stroke-linecap=\"round\" stroke-linejoin=\"round\" aria-hidden=\"true\">{shape}</svg>"
    )
}
