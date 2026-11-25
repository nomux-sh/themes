//! The token set: the one list of semantic design tokens that the nomux UI
//! reads. The schema, the token reference and the validator come from it.

use crate::value::ValueKind;

/// The description of one token.
#[derive(Debug, PartialEq, Eq)]
pub struct TokenSpec {
    /// The dotted name, for example `surface.base`.
    pub name: &'static str,
    /// The kind of value.
    pub kind: ValueKind,
    /// What the token is.
    pub role: &'static str,
    /// Where the UI can use the token.
    pub usage: &'static str,
}

impl TokenSpec {
    /// The first part of the name. Tokens with the same group are one
    /// section of the token reference.
    pub fn group(&self) -> &'static str {
        self.name.split('.').next().unwrap_or(self.name)
    }
}

macro_rules! token_kind {
    (
        $(#[$meta:meta])*
        $token:ident, $kind:expr;
        $( $variant:ident = $name:literal, $role:literal, $usage:literal; )+
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum $token {
            $( #[doc = concat!("`", $name, "`: ", $role)] $variant, )+
        }

        impl $token {
            /// All tokens of this kind, in the sequence of the token reference.
            pub const ALL: &[Self] = &[ $( Self::$variant, )+ ];

            const SPECS: &[TokenSpec] = &[
                $( TokenSpec { name: $name, kind: $kind, role: $role, usage: $usage }, )+
            ];

            /// The description of the token.
            pub fn spec(self) -> &'static TokenSpec {
                &Self::SPECS[self as usize]
            }

            /// The dotted name of the token.
            pub fn name(self) -> &'static str {
                self.spec().name
            }
        }
    };
}

token_kind! {
    /// The colour tokens.
    ColorToken, ValueKind::Color;

    SurfaceSunken = "surface.sunken",
        "The layout tone one step below the base.",
        "Project rail and the track of a tab bar.";
    SurfaceBase = "surface.base",
        "The base tone of the window.",
        "Sidebar, Pane content that is not a terminal (diff, file, timeline) and each region that has no other surface.";
    SurfaceButton = "surface.button",
        "Elevation level 1 (button): the flat fill of a control.",
        "Buttons, shell chips of a tab bar, member chips of a team roster and input fields.";
    SurfaceRaised = "surface.raised",
        "Elevation level 2 (raised): the tone of a container that holds controls.",
        "The active Tab, the expanded card of the active sidebar row and the approval card.";
    SurfaceFloating = "surface.floating",
        "Elevation level 3 (floating): the tone of a transient element above the layout.",
        "Menus, the command palette, tooltips and hover previews. Only this level has a shadow.";

    TextPrimary = "text.primary",
        "The text level with the most contrast.",
        "Titles, the names of rows, the active Tab and the text of content.";
    TextSecondary = "text.secondary",
        "The text level of items that are not active.",
        "Tabs that are not active, chips and the letters of icons.";
    TextMuted = "text.muted",
        "The text level with the least contrast. It keeps a contrast of 4.5:1 without opacity.",
        "Metadata after ` · `, Child work rows, hints and placeholders.";
    TextOnAccent = "text.on-accent",
        "The text on a fill of the accent colour.",
        "The label of the primary button.";

    FillHover = "fill.hover",
        "A neutral tint of `text.primary` (4% to 8%) that is painted over a surface.",
        "The row, Tab or control below the pointer.";
    FillSelected = "fill.selected",
        "A neutral tint of `text.primary` (4% to 8%) that is painted over a surface. The text colour does not change.",
        "The selected row and a pressed control.";

    LineHairline = "line.hairline",
        "A line colour with low alpha.",
        "Separators between items in one region, where a tone step is not possible. Do not use it to put a box around a region.";
    LineDivider = "line.divider",
        "The line between two Panes.",
        "Pane dividers. The divider gets the accent colour when the pointer is on it.";

    FocusRing = "focus.ring",
        "The colour of the keyboard focus indicator.",
        "The ring around the control or the row that has the keyboard focus.";

    Accent = "accent",
        "The one accent colour of the theme.",
        "The fill of the primary button, the indicator of the active Tab, the Pane divider below the pointer and the text cursor of an input field.";

    ShadowFloating = "shadow.floating",
        "The shadow colour of elevation level 3.",
        "The shadow of menus, the command palette, tooltips and hover previews. The client sets the offset and the blur.";

    StateWorking = "state.working",
        "State colour blue: the agent works.",
        "Status marks only: the mark of a row, a Tab, a project tile or a team member.";
    StateNeedsYou = "state.needs-you",
        "State colour yellow: the agent waits for the user (permission, question or sign-in).",
        "Status marks and the edge of the approval card. It is the only state that can pulse.";
    StateDone = "state.done",
        "State colour green: the agent is done and the user did not read the result.",
        "Status marks only.";
    StateError = "state.error",
        "State colour red: the agent or the Session stopped with an error.",
        "Status marks, and the mark of a field that has an input error.";
    StateIdle = "state.idle",
        "State colour grey: the Session is idle.",
        "Status marks only.";

    TerminalBackground = "terminal.background",
        "The default background of a terminal.",
        "The cells of a terminal and the region around them in a terminal Tab.";
    TerminalForeground = "terminal.foreground",
        "The default text colour of a terminal.",
        "Terminal text that has no ANSI colour.";
    TerminalCursor = "terminal.cursor",
        "The colour of the terminal cursor.",
        "The cursor block, bar or underline of a terminal.";
    TerminalSelection = "terminal.selection",
        "The background of selected terminal text.",
        "It is painted over the cell background. The text colour does not change.";
    TerminalAnsiBlack = "terminal.ansi.black", "ANSI colour 0.", "Terminal cells that set colour 0.";
    TerminalAnsiRed = "terminal.ansi.red", "ANSI colour 1.", "Terminal cells that set colour 1.";
    TerminalAnsiGreen = "terminal.ansi.green", "ANSI colour 2.", "Terminal cells that set colour 2.";
    TerminalAnsiYellow = "terminal.ansi.yellow", "ANSI colour 3.", "Terminal cells that set colour 3.";
    TerminalAnsiBlue = "terminal.ansi.blue", "ANSI colour 4.", "Terminal cells that set colour 4.";
    TerminalAnsiMagenta = "terminal.ansi.magenta", "ANSI colour 5.", "Terminal cells that set colour 5.";
    TerminalAnsiCyan = "terminal.ansi.cyan", "ANSI colour 6.", "Terminal cells that set colour 6.";
    TerminalAnsiWhite = "terminal.ansi.white", "ANSI colour 7.", "Terminal cells that set colour 7.";
    TerminalAnsiBrightBlack = "terminal.ansi.bright-black", "ANSI colour 8.", "Terminal cells that set colour 8.";
    TerminalAnsiBrightRed = "terminal.ansi.bright-red", "ANSI colour 9.", "Terminal cells that set colour 9.";
    TerminalAnsiBrightGreen = "terminal.ansi.bright-green", "ANSI colour 10.", "Terminal cells that set colour 10.";
    TerminalAnsiBrightYellow = "terminal.ansi.bright-yellow", "ANSI colour 11.", "Terminal cells that set colour 11.";
    TerminalAnsiBrightBlue = "terminal.ansi.bright-blue", "ANSI colour 12.", "Terminal cells that set colour 12.";
    TerminalAnsiBrightMagenta = "terminal.ansi.bright-magenta", "ANSI colour 13.", "Terminal cells that set colour 13.";
    TerminalAnsiBrightCyan = "terminal.ansi.bright-cyan", "ANSI colour 14.", "Terminal cells that set colour 14.";
    TerminalAnsiBrightWhite = "terminal.ansi.bright-white", "ANSI colour 15.", "Terminal cells that set colour 15.";

    DiffAddedBackground = "diff.added.background",
        "The background of an added line. It is painted over `surface.base`.",
        "Added lines of a diff.";
    DiffAddedGutter = "diff.added.gutter",
        "The mark of an added line.",
        "The gutter bar and the `+` sign of an added line.";
    DiffRemovedBackground = "diff.removed.background",
        "The background of a removed line. It is painted over `surface.base`.",
        "Removed lines of a diff.";
    DiffRemovedGutter = "diff.removed.gutter",
        "The mark of a removed line.",
        "The gutter bar and the `-` sign of a removed line.";
    DiffChangedBackground = "diff.changed.background",
        "The background of the changed part of a line. It is painted over `surface.base`.",
        "The words that changed in a line, and a changed line in a file gutter.";
    DiffChangedGutter = "diff.changed.gutter",
        "The mark of a changed line.",
        "The gutter bar of a changed line.";

    SyntaxComment = "syntax.comment", "Syntax role: comments.", "Code in a diff, a file or a timeline.";
    SyntaxKeyword = "syntax.keyword", "Syntax role: keywords and operators that are words.", "Code in a diff, a file or a timeline.";
    SyntaxString = "syntax.string", "Syntax role: strings and characters.", "Code in a diff, a file or a timeline.";
    SyntaxConstant = "syntax.constant", "Syntax role: numbers, booleans and named constants.", "Code in a diff, a file or a timeline.";
    SyntaxFunction = "syntax.function", "Syntax role: names of functions and methods.", "Code in a diff, a file or a timeline.";
    SyntaxType = "syntax.type", "Syntax role: names of types.", "Code in a diff, a file or a timeline.";
}

token_kind! {
    /// The font family tokens.
    FontFamilyToken, ValueKind::FontFamily;

    Ui = "font.ui",
        "The one font family of the UI.",
        "All text that is not code.";
    Mono = "font.mono",
        "The one monospace font family.",
        "Terminals, diffs, files and code in a timeline.";
}

token_kind! {
    /// The length tokens.
    LengthToken, ValueKind::Length;

    FontTitleSize = "font.title.size", "Type scale: the size of a title.", "The title row of a surface, and the title of a row or a Tab.";
    FontRowSize = "font.row.size", "Type scale: the size of a row.", "The second line of a row, body text and the labels of controls.";
    FontMetaSize = "font.meta.size", "Type scale: the size of metadata.", "Metadata, counters and hints.";
    FontCodeSize = "font.code.size", "Type scale: the size of monospace text.", "Terminals, diffs, files and code in a timeline.";

    LineWidth = "line.width", "The width of a line.", "Hairlines and Pane dividers.";
    FocusRingWidth = "focus.ring-width", "The width of the focus ring.", "The ring around the control or the row that has the keyboard focus.";

    Space1 = "space.1", "Spacing step 1 of the 4 px grid.", "The gap between an icon or a mark and its label.";
    Space2 = "space.2", "Spacing step 2 of the 4 px grid.", "The gap between the items of a row and between controls.";
    Space3 = "space.3", "Spacing step 3 of the 4 px grid.", "The padding of a card and of a floating element.";
    Space4 = "space.4", "Spacing step 4 of the 4 px grid.", "The gap between the sections of a region, and the indent of one tree level.";

    DensityComfortableRowHeight = "density.comfortable.row-height", "Comfortable density: the height of a row.", "Sidebar rows and menu rows.";
    DensityComfortableControlHeight = "density.comfortable.control-height", "Comfortable density: the height of a control.", "Buttons, Tabs, chips and input fields.";
    DensityComfortablePadding = "density.comfortable.padding", "Comfortable density: the horizontal padding.", "The inner edges of rows, Tabs and controls.";
    DensityCompactRowHeight = "density.compact.row-height", "Compact density: the height of a row.", "Sidebar rows and menu rows.";
    DensityCompactControlHeight = "density.compact.control-height", "Compact density: the height of a control.", "Buttons, Tabs, chips and input fields.";
    DensityCompactPadding = "density.compact.padding", "Compact density: the horizontal padding.", "The inner edges of rows, Tabs and controls.";

    SizeIcon = "size.icon", "The one icon size of a row.", "Icons in rows, Tabs, chips and buttons.";
    SizeMark = "size.mark", "The diameter of a status mark.", "Status marks. A state does not change the size of the mark.";

    RadiusButton = "radius.button", "The corner radius of elevation level 1.", "Buttons, chips, Tabs, rows and input fields.";
    RadiusRaised = "radius.raised", "The corner radius of elevation level 2.", "Cards.";
    RadiusFloating = "radius.floating", "The corner radius of elevation level 3.", "Menus, the command palette, tooltips and hover previews.";
}

token_kind! {
    /// The font weight tokens.
    FontWeightToken, ValueKind::FontWeight;

    FontTitleWeight = "font.title.weight", "Type scale: the weight of a title.", "The title row of a surface, and the title of a row or a Tab.";
    FontRowWeight = "font.row.weight", "Type scale: the weight of a row.", "The second line of a row, body text and the labels of controls.";
    FontMetaWeight = "font.meta.weight", "Type scale: the weight of metadata.", "Metadata, counters and hints.";
    FontCodeWeight = "font.code.weight", "Type scale: the weight of monospace text.", "Terminals, diffs, files and code in a timeline.";
}

token_kind! {
    /// The duration tokens.
    DurationToken, ValueKind::Duration;

    MotionFast = "motion.fast",
        "The duration of a colour change, from 120 ms to 200 ms.",
        "A state change of a mark, a hover fill and a selection fill.";
    MotionSlow = "motion.slow",
        "The duration of a size change, from 120 ms to 200 ms.",
        "A row that expands, a panel that opens and a Tab that moves.";
}

token_kind! {
    /// The easing tokens.
    EasingToken, ValueKind::Easing;

    MotionEasing = "motion.easing",
        "The one easing of the UI: ease-out.",
        "Each transition. Motion occurs only on a state change, and the client stops it when the user asks for reduced motion.";
}

/// All tokens of the token set, in the sequence of the token reference.
pub fn all() -> impl Iterator<Item = &'static TokenSpec> {
    ColorToken::SPECS
        .iter()
        .chain(FontFamilyToken::SPECS)
        .chain(LengthToken::SPECS)
        .chain(FontWeightToken::SPECS)
        .chain(DurationToken::SPECS)
        .chain(EasingToken::SPECS)
}

/// Finds a token by its dotted name.
pub fn find(name: &str) -> Option<&'static TokenSpec> {
    all().find(|spec| spec.name == name)
}
