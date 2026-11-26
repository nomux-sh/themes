//! The section "Groups (alternatives)": three ways to show the tree of
//! groups and projects. Each alternative shows the same sample tree in the
//! same frames, in both densities.

use super::super::escape;
use super::super::glyph::{State, glyph};
use super::group::{DEEPEST, Entry, Group, GroupFrame, Node, WEB};
use super::icon::{Icon, icon};
use super::row::{Face, Row, Slot};
use super::sample::Project;
use super::{render, tree};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Alternative {
    /// The rail goes away. Groups, projects and the worktrees of the open
    /// project are rows of one tree.
    Tree,
    /// The tree of groups and projects is a column of its own, next to the
    /// sidebar of the selected project.
    TwoPanes,
    /// The rail stays. A group is a tile that opens in place; a nested group
    /// steps in.
    Rail,
}

impl Alternative {
    pub const ALL: [Self; 3] = [Self::Tree, Self::TwoPanes, Self::Rail];

    const fn number(self) -> u8 {
        match self {
            Self::Tree => 1,
            Self::TwoPanes => 2,
            Self::Rail => 3,
        }
    }

    const fn title(self) -> &'static str {
        match self {
            Self::Tree => "Alternative 1: tree in the sidebar",
            Self::TwoPanes => "Alternative 2: two panes",
            Self::Rail => "Alternative 3: rail with group tiles",
        }
    }

    const fn description(self) -> &'static str {
        match self {
            Self::Tree => {
                "The rail goes away. Groups and projects are rows of one tree; the selected project opens \
                 in place, and its worktrees are the rows below it. A group row has the secondary text face; \
                 a closed group shows its aggregate state in the slot and the count of its projects in the tail."
            }
            Self::TwoPanes => {
                "The tree of groups and projects is a column of its own on surface.sunken. The sidebar of \
                 the selected project stays as specified. The column collapses to the rail of frame (f)."
            }
            Self::Rail => {
                "The rail stays. A group tile shows up to four small tiles of what it holds. A group at the \
                 top opens in place; a nested group steps in, and the back tile shows the most urgent state \
                 outside it. The names show below the pointer, or in the wide rail of frames (c) and (e)."
            }
        }
    }

    pub fn window(self, frame: GroupFrame) -> String {
        let tree = frame.tree();
        let project = frame.project();
        match self {
            Self::Tree => {
                let navigator = Navigator {
                    project: Some(&project),
                    select_project: frame != GroupFrame::Selected,
                };
                let rows: String = navigator.rows(&tree, 0).iter().map(Row::render).collect();
                format!("<div class=\"sidebar\">\n{rows}</div>\n")
            }
            Self::TwoPanes => {
                let left = if frame == GroupFrame::Selected {
                    rail(&tree, frame)
                } else {
                    let navigator = Navigator {
                        project: None,
                        select_project: true,
                    };
                    let rows: String = navigator.rows(&tree, 0).iter().map(Row::render).collect();
                    format!("<div class=\"navigator\">\n{rows}</div>\n")
                };
                format!("{left}{}", render(&project))
            }
            Self::Rail => format!("{}{}", rail(&tree, frame), render(&project)),
        }
    }
}

/// The rows of a tree of groups and projects. With a project, the selected
/// project is open and its worktrees follow it.
struct Navigator<'a> {
    project: Option<&'a Project>,
    select_project: bool,
}

impl Navigator<'_> {
    fn rows(&self, group: &Group, level: u8) -> Vec<Row> {
        group
            .nodes
            .iter()
            .flat_map(|node| match node {
                Node::Group(inner) => {
                    let mut rows = vec![group_row(inner, level)];
                    if inner.open {
                        rows.extend(self.rows(inner, level + 1));
                    }
                    rows
                }
                Node::Project(entry) => self.project_rows(entry, level),
            })
            .collect()
    }

    fn project_rows(&self, entry: &Entry, level: u8) -> Vec<Row> {
        let open = self.project.filter(|_| entry.selected);
        let slot = match open {
            Some(_) => Slot::glyph(None).with_hover(Icon::Open),
            None => Slot::glyph(Some(entry.state)),
        };
        let mut rows = vec![Row {
            selected: entry.selected && self.select_project,
            ..Row::new(level, Face::Project, entry.name, slot)
        }];
        if let Some(project) = open {
            rows.extend(tree::rows(project).into_iter().map(|mut row| {
                row.level += level + 1;
                row
            }));
        }
        rows
    }
}

/// A group row: a closed group shows its aggregate state and the count of
/// its projects, an open group shows neither, an empty group says so.
fn group_row(group: &Group, level: u8) -> Row {
    let (slot, secondary, tail) = if group.nodes.is_empty() {
        (Slot::glyph(None), "empty", String::new())
    } else if group.open {
        (Slot::glyph(None).with_hover(Icon::Open), "", String::new())
    } else {
        (
            Slot::glyph(group.worst()).with_hover(Icon::Closed),
            "",
            format!("<span class=\"age\">{}</span>", group.projects()),
        )
    };
    Row {
        secondary: secondary.to_owned(),
        tail,
        hover: group.hover,
        ..Row::new(level, Face::Group, group.name, slot)
    }
}

/// The rail of group tiles. At the top a group opens in place. In the
/// frames "Depth" and "Selected" the rail has stepped into a nested group.
fn rail(tree: &Group, frame: GroupFrame) -> String {
    let named = matches!(frame, GroupFrame::Depth | GroupFrame::LongNames);
    let path: &[&str] = match frame {
        GroupFrame::Depth => &DEEPEST,
        GroupFrame::Selected => &WEB,
        _ => &[],
    };
    let items: String = if path.is_empty() {
        tree.nodes
            .iter()
            .map(|node| tile(node, named, true))
            .collect()
    } else {
        let inner = tree.find(path);
        let back = item(
            &icon(Icon::Back),
            tree.worst_outside(inner),
            "",
            &path.join(" / "),
            named,
        );
        let tiles: String = inner
            .nodes
            .iter()
            .map(|node| tile(node, named, false))
            .collect();
        format!("{back}{tiles}")
    };
    let class = if named { "rail named" } else { "rail" };
    format!("<div class=\"{class}\">{items}</div>\n")
}

fn tile(node: &Node, named: bool, in_place: bool) -> String {
    match node {
        Node::Project(entry) => item(
            &initials(entry.name),
            Some(entry.state),
            if entry.selected { " selected" } else { "" },
            entry.name,
            named,
        ),
        Node::Group(group) if in_place && group.open && !group.nodes.is_empty() => {
            let children: String = group
                .nodes
                .iter()
                .map(|node| tile(node, named, false))
                .collect();
            format!(
                "{}<div class=\"rail-children\">{children}</div>",
                item(&icon(Icon::Open), None, " group", group.name, named)
            )
        }
        Node::Group(group) => {
            let minis = if group.nodes.is_empty() {
                String::new()
            } else {
                format!("<span class=\"minis\">{}</span>", "<span></span>".repeat(4))
            };
            let mut classes = String::from(" group");
            for (on, class) in [
                (group.find_selected().is_some(), " selected"),
                (group.hover, " hover"),
            ] {
                if on {
                    classes.push_str(class);
                }
            }
            item(&minis, group.worst(), &classes, group.name, named)
        }
    }
}

/// One tile of the rail and, in the wide rail, its name. Below the pointer
/// the name shows in a tooltip.
fn item(face: &str, state: Option<State>, classes: &str, name: &str, named: bool) -> String {
    let name = escape(name);
    let tooltip = if classes.contains("hover") && !named {
        format!("<span class=\"tooltip\">{name}</span>")
    } else {
        String::new()
    };
    let label = if named {
        format!("<span class=\"rail-name\">{name}</span>")
    } else {
        String::new()
    };
    format!(
        "<div class=\"rail-item{classes}\"><div class=\"tile{classes}\">{face}{}{tooltip}</div>{label}</div>",
        state.map(glyph).unwrap_or_default()
    )
}

/// The letters of a project tile: the first letter of the first two words.
fn initials(name: &str) -> String {
    name.split('-')
        .take(2)
        .filter_map(|word| word.chars().next())
        .collect::<String>()
        .to_uppercase()
}

/// The section "Groups (alternatives)" of a theme.
pub fn section(theme_name: &str) -> String {
    let alternatives: String = Alternative::ALL
        .iter()
        .map(|alternative| block(*alternative))
        .collect();
    format!(
        "<div class=\"sidebar-section groups\">\n<header>\n<h3>Groups (alternatives)</h3>\n<p class=\"meta\">Three \
         ways to show groups, nested groups and projects in {name}. The group row is not decided. Each \
         alternative shows the same sample: nine projects in four groups, one group in a group in a group. \
         The selected project acme-web is the project of the section \"Sidebar\".</p>\n</header>\n{alternatives}</div>\n",
        name = escape(theme_name),
    )
}

fn block(alternative: Alternative) -> String {
    let densities: String = ["comfortable", "compact"]
        .iter()
        .map(|density| {
            let frames: String = GroupFrame::ALL
                .iter()
                .map(|frame| {
                    format!(
                        "<figure>\n<div class=\"window sidebar-window\">\n{}</div>\n<figcaption class=\"caption\">{}</figcaption>\n</figure>\n",
                        alternative.window(*frame),
                        frame.caption()
                    )
                })
                .collect();
            format!("<div class=\"group-frames {density}\">\n<p class=\"meta\">{density} density</p>\n{frames}</div>\n")
        })
        .collect();
    format!(
        "<div class=\"alternative\" data-alternative=\"{number}\">\n<h4>{title}</h4>\n<p class=\"meta\">{description}</p>\n{densities}</div>\n",
        number = alternative.number(),
        title = alternative.title(),
        description = alternative.description(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rows(alternative: Alternative, frame: GroupFrame) -> Vec<String> {
        alternative
            .window(frame)
            .lines()
            .filter(|line| line.contains("class=\"tree-row"))
            .map(str::to_owned)
            .collect()
    }

    #[test]
    fn a_closed_group_shows_the_most_urgent_state_inside_it() {
        // acme-web needs the user three levels below Client work.
        for alternative in [Alternative::Tree, Alternative::TwoPanes] {
            let rows = rows(alternative, GroupFrame::Aggregate);
            assert!(rows[0].contains("Client work") && rows[0].contains("glyph needs-you"));
        }
    }

    #[test]
    fn an_open_group_shows_no_aggregate_state() {
        for alternative in [Alternative::Tree, Alternative::TwoPanes] {
            let rows = rows(alternative, GroupFrame::Rest);
            assert!(rows[..3].iter().all(|row| !row.contains("class=\"glyph")));
        }
    }

    #[test]
    fn the_rows_of_the_open_project_follow_its_depth() {
        // acme-web is at level 3, its worktrees at level 4, their Sessions at 5.
        let rows = rows(Alternative::Tree, GroupFrame::Selected);
        let claude = rows
            .iter()
            .find(|row| row.contains(">Claude<"))
            .expect("the Claude row");
        assert!(claude.contains("--level: 5"));
    }

    #[test]
    fn the_back_tile_shows_the_most_urgent_state_outside_the_group() {
        // In 2019, acme-web is outside and needs the user.
        let window = Alternative::Rail.window(GroupFrame::Depth);
        let rail = &window[..window.find("class=\"sidebar\"").expect("a sidebar")];
        let back = &rail[..rail.find("rail-name").expect("a wide rail")];
        assert!(back.contains("glyph needs-you"));
    }
}
