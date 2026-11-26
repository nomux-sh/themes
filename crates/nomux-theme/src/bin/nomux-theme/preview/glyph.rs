//! The state glyphs: one shape for each state of a Session, so that a state
//! does not depend on colour only. Each glyph is an inline SVG. The size
//! comes from `size.mark` and the colour from the `state.*` token.

/// The state of a Session, in the sequence of the priority: the first state
/// is the most urgent.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum State {
    NeedsYou,
    Error,
    Working,
    Done,
    Idle,
}

impl State {
    pub const ALL: [Self; 5] = [
        Self::NeedsYou,
        Self::Error,
        Self::Working,
        Self::Done,
        Self::Idle,
    ];

    /// The class of the glyph and the suffix of the `state.*` token.
    pub const fn name(self) -> &'static str {
        match self {
            Self::NeedsYou => "needs-you",
            Self::Error => "error",
            Self::Working => "working",
            Self::Done => "done",
            Self::Idle => "idle",
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::NeedsYou => "needs you",
            Self::Error => "error",
            Self::Working => "working",
            Self::Done => "done, not seen",
            Self::Idle => "idle",
        }
    }

    /// The shape in a 16 by 16 view box: an arc, a disc, a check, a cross
    /// and a ring.
    const fn shape(self) -> &'static str {
        match self {
            Self::Working => "<path d=\"M8 2a6 6 0 1 1-6 6\"/>",
            Self::NeedsYou => "<circle cx=\"8\" cy=\"8\" r=\"6.5\" class=\"filled\"/>",
            Self::Done => "<path d=\"M2.5 8.5l3.5 3.5 7.5-8\"/>",
            Self::Error => "<path d=\"M3.5 3.5l9 9m0-9-9 9\"/>",
            Self::Idle => "<circle cx=\"8\" cy=\"8\" r=\"5.5\"/>",
        }
    }
}

/// The stroke of the open shapes, in units of the view box. The shapes
/// scale with the size of the glyph.
const STROKE: &str = "fill=\"none\" stroke=\"currentColor\" stroke-width=\"3\" stroke-linecap=\"round\" stroke-linejoin=\"round\"";

pub fn glyph(state: State) -> String {
    format!(
        "<svg class=\"glyph {name}\" viewBox=\"0 0 16 16\" {STROKE} role=\"img\" aria-label=\"{label}\">{shape}</svg>",
        name = state.name(),
        label = state.label(),
        shape = state.shape()
    )
}

/// Replaces each `{glyph-<state>}` in a template with the glyph of the state.
pub fn fill_template(template: &str) -> String {
    State::ALL.iter().fold(template.to_owned(), |text, state| {
        text.replace(&format!("{{glyph-{}}}", state.name()), &glyph(*state))
    })
}
