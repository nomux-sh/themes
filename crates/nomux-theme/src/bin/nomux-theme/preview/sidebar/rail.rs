//! The project rail: one tile for each project and each group. A group
//! opens in place: its tile and its children are in a strip, and a group
//! in an open group opens in the same way inside the strip.

use super::super::escape;
use super::super::glyph::{State, glyph};
use super::group::{Entry, Group, GroupFrame, Node};

/// The levels of open groups that make the strip narrower by one step.
/// Deeper strips have the width of the fifth level.
const STEPPED_LEVELS: usize = 5;

/// The rail in one frame. The density comes from the class of an ancestor.
pub fn rail(frame: GroupFrame) -> String {
    let rail = Rail {
        expanded: frame.expanded(),
    };
    let class = if rail.expanded {
        "rail expanded"
    } else {
        "rail"
    };
    format!(
        "<div class=\"{class}\">{}</div>\n",
        rail.children(&frame.tree(), &[])
    )
}

struct Rail {
    /// The rail is wide and shows the name beside each tile.
    expanded: bool,
}

impl Rail {
    /// The tiles of the children of a group. `path` holds the names of the
    /// group and of the groups that hold it.
    fn children(&self, group: &Group, path: &[&str]) -> String {
        let siblings: Vec<&str> = group
            .nodes
            .iter()
            .filter_map(|node| match node {
                Node::Project(entry) => Some(entry.name),
                Node::Group(_) => None,
            })
            .collect();
        let mut taken: Vec<String> = Vec::new();
        group
            .nodes
            .iter()
            .map(|node| match node {
                Node::Project(entry) => {
                    let label = label(entry.name, &siblings, path, &taken);
                    let tile = self.project(entry, &label);
                    taken.push(label);
                    tile
                }
                Node::Group(inner) => self.group(inner, &[path, &[inner.name]].concat()),
            })
            .collect()
    }

    fn project(&self, entry: &Entry, label: &str) -> String {
        self.item(&Tile {
            face: escape(label),
            state: Some(entry.state),
            classes: marks(entry.selected, entry.hover, entry.focus),
            name: entry.name,
            detail: None,
        })
    }

    /// A closed group is one tile. An open group is a strip that holds its
    /// tile and, below it, its children.
    fn group(&self, group: &Group, path: &[&str]) -> String {
        let count = match group.projects() {
            1 => "1 project".to_owned(),
            count => format!("{count} projects"),
        };
        let detail = if group.nodes.is_empty() {
            "empty".to_owned()
        } else {
            count
        };
        if !group.open {
            let mut classes = marks(group.holds_selected(), group.hover, false);
            classes.push_str(" group");
            return self.item(&Tile {
                face: minis(group),
                state: group.worst(),
                classes,
                name: group.name,
                detail: Some(detail),
            });
        }
        let depth = path.len();
        let initial: String = group
            .name
            .chars()
            .take(1)
            .flat_map(char::to_uppercase)
            .collect();
        let tile = self.item(&Tile {
            face: format!("{}{OPEN_MARK}", escape(&initial)),
            state: None,
            classes: format!("{} group open", marks(false, group.hover, false)),
            name: group.name,
            detail: Some(detail),
        });
        let class = if depth > STEPPED_LEVELS {
            "rail-group deep"
        } else {
            "rail-group"
        };
        format!(
            "<div class=\"{class}\" style=\"--depth: {step}; background: {fill}\">{tile}{children}</div>",
            step = depth.min(STEPPED_LEVELS),
            fill = fill(depth),
            children = self.children(group, path),
        )
    }

    /// One tile and, in the expanded rail, its name. Below the pointer the
    /// collapsed rail shows the name in a tooltip.
    fn item(&self, tile: &Tile) -> String {
        let name = escape(tile.name);
        let tooltip = if tile.classes.contains("hover") && !self.expanded {
            let detail = tile
                .detail
                .as_ref()
                .map(|detail| format!("<span class=\"detail\">{detail}</span>"))
                .unwrap_or_default();
            format!("<span class=\"tooltip\">{name}{detail}</span>")
        } else {
            String::new()
        };
        let label = if self.expanded {
            format!("<span class=\"rail-name\">{name}</span>")
        } else {
            String::new()
        };
        format!(
            "<div class=\"rail-item\"><div class=\"tile{classes}\">{face}{glyph}{tooltip}</div>{label}</div>",
            classes = tile.classes,
            face = tile.face,
            glyph = tile.state.map(glyph).unwrap_or_default(),
        )
    }
}

struct Tile<'a> {
    face: String,
    /// The state mark at the corner: the state of a project, or the
    /// aggregate state of a closed group.
    state: Option<State>,
    classes: String,
    name: &'a str,
    /// The second part of the tooltip of a group: its count of projects.
    detail: Option<String>,
}

fn marks(selected: bool, hover: bool, focus: bool) -> String {
    [
        (selected, " selected"),
        (hover, " hover"),
        (focus, " focus"),
    ]
    .iter()
    .filter(|(on, _)| *on)
    .map(|(_, class)| *class)
    .collect()
}

/// The mark of an open group at the bottom right corner of its tile: the
/// open caret, drawn like a state glyph.
const OPEN_MARK: &str = "<svg class=\"open-mark\" viewBox=\"0 0 16 16\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"3\" stroke-linecap=\"round\" stroke-linejoin=\"round\" aria-label=\"open\"><path d=\"M3 6l5 5 5-5\"/></svg>";

/// The face of a closed group: a small square for each of its first four
/// children, filled for a project, hollow for a group.
fn minis(group: &Group) -> String {
    if group.nodes.is_empty() {
        return String::new();
    }
    let squares: String = group
        .nodes
        .iter()
        .take(4)
        .map(|node| match node {
            Node::Project(_) => "<span></span>",
            Node::Group(_) => "<span class=\"hollow\"></span>",
        })
        .collect();
    format!("<span class=\"minis\">{squares}</span>")
}

/// The fill of a strip: one layer of `fill.level` for each level of open
/// groups, over the surface of the rail, to five levels.
fn fill(depth: usize) -> String {
    let step = "linear-gradient(var(--fill-level), var(--fill-level)), ";
    format!(
        "{}var(--surface-sunken)",
        step.repeat(depth.min(STEPPED_LEVELS))
    )
}

/// The two letters of a project tile.
///
/// 1. The words of the name are separated by `-`, `_`, `.` and spaces.
/// 2. The leading words that all projects of the group share, and each word
///    that is a word of the name of a group that holds the project, are
///    removed. When no word stays, the first word after the shared words
///    stays.
/// 3. One word gives its first letter in upper case and its second letter in
///    lower case. Two or more words give the first letter of the first two
///    words, in upper case.
/// 4. When an earlier project of the group has the same label, the second
///    letter becomes the next letter of the remaining words that gives a
///    label that is free.
fn label(name: &str, siblings: &[&str], path: &[&str], taken: &[String]) -> String {
    let own = words(name);
    let shared = if siblings.len() < 2 {
        0
    } else {
        (0..own.len())
            .take_while(|index| {
                siblings
                    .iter()
                    .all(|sibling| words(sibling).get(*index) == Some(&own[*index]))
            })
            .count()
    };
    let group_words: Vec<String> = path.iter().flat_map(|group| words(group)).collect();
    let mut kept: Vec<&String> = own
        .iter()
        .skip(shared)
        .filter(|word| !group_words.contains(word))
        .collect();
    if kept.is_empty() {
        kept.extend(own.get(shared).or(own.last()));
    }
    let letters: Vec<char> = kept.iter().flat_map(|word| word.chars()).collect();
    let first = letters[0].to_uppercase().collect::<String>();
    let label = match kept.as_slice() {
        [_, second, ..] => {
            format!("{first}{}", second.chars().next().unwrap_or_default()).to_uppercase()
        }
        _ => format!("{first}{}", letters.get(1).copied().unwrap_or_default()),
    };
    if !taken.contains(&label) {
        return label;
    }
    letters[1..]
        .iter()
        .map(|letter| format!("{first}{letter}"))
        .find(|candidate| !taken.contains(candidate))
        .unwrap_or(label)
}

fn words(name: &str) -> Vec<String> {
    name.split(['-', '_', '.', ' '])
        .filter(|word| !word.is_empty())
        .map(str::to_lowercase)
        .collect()
}

/// The part "Groups" of the section "Sidebar": the rail in each frame, in
/// both densities.
pub fn groups() -> String {
    let densities: String = ["comfortable", "compact"]
        .iter()
        .map(|density| {
            let frames: String = GroupFrame::ALL
                .iter()
                .map(|frame| {
                    format!(
                        "<figure class=\"{class}\">\n<div class=\"window rail-window\">\n{rail}</div>\n<figcaption class=\"caption\">{caption}</figcaption>\n</figure>\n",
                        class = frame.class(),
                        rail = rail(*frame),
                        caption = frame.caption(),
                    )
                })
                .collect();
            format!("<div class=\"rail-frames {density}\">\n<p class=\"meta\">{density} density</p>\n{frames}</div>\n")
        })
        .collect();
    format!(
        "<div class=\"groups\">\n<h4>Groups</h4>\n<p class=\"meta\">The project rail with the same sample: nine \
         projects in four groups, one group in a group in a group. A group opens in place: its tile is the first \
         tile of a strip, its children follow below it. The selected project acme-web is the project of the \
         sidebar.</p>\n{densities}</div>\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tiles(frame: GroupFrame) -> Vec<String> {
        rail(frame)
            .split("<div class=\"tile")
            .skip(1)
            .map(str::to_owned)
            .collect()
    }

    fn tile(frame: GroupFrame, name: &str) -> String {
        let rail = Rail { expanded: true };
        let html = rail.children(&frame.tree(), &[]);
        html.split("<div class=\"rail-item\">")
            .find(|item| item.contains(&format!("rail-name\">{name}<")))
            .expect("the tile")
            .to_owned()
    }

    #[test]
    fn a_closed_group_shows_the_most_urgent_state_inside_it() {
        // acme-web needs the user three levels below Client work.
        let client = tile(GroupFrame::Closed, "Client work");
        assert!(client.contains("glyph needs-you"));
        assert!(client.contains(" selected"));
    }

    #[test]
    fn an_open_group_shows_no_aggregate_state() {
        let client = tile(GroupFrame::Rest, "Client work");
        assert!(client.contains(" open") && !client.contains("class=\"glyph"));
    }

    #[test]
    fn each_open_group_is_a_strip_that_starts_with_its_tile() {
        for (frame, strips) in [
            (GroupFrame::Closed, 0),
            (GroupFrame::OneOpen, 1),
            (GroupFrame::TwoLevels, 2),
            (GroupFrame::Rest, 3),
            (GroupFrame::FiveLevels, 5),
            (GroupFrame::Scroll, 6),
        ] {
            let html = rail(frame);
            let groups: Vec<&str> = html.split("<div class=\"rail-group").skip(1).collect();
            assert_eq!(groups.len(), strips, "{frame:?}");
            for group in groups {
                let first = group.find("<div class=\"tile").expect("a tile");
                assert!(group[first..].starts_with("<div class=\"tile group open"));
            }
        }
    }

    #[test]
    fn an_empty_open_group_is_a_strip_with_its_tile_only() {
        let html = rail(GroupFrame::Empty);
        let later = &html[html.rfind("<div class=\"rail-group").expect("a strip")..];
        let item = "</div></div>";
        let end = later.find(item).expect("the tile of Later") + item.len();
        assert!(
            later[end..].starts_with("</div>"),
            "the strip ends after its tile"
        );
    }

    #[test]
    fn the_selected_project_in_an_open_group_keeps_its_mark() {
        let selected: Vec<String> = tiles(GroupFrame::Rest)
            .into_iter()
            .filter(|tile| tile.contains(" selected"))
            .collect();
        assert_eq!(selected.len(), 1);
        assert!(selected[0].contains(">We<"));
    }

    #[test]
    fn a_long_group_name_does_not_change_a_label() {
        let short = ["Client work", "Acme", "Web", "Legacy", "2019"];
        let long = [
            "Client work",
            "Acme Corporation customer platform",
            "Storefront and checkout for the web",
            "Legacy",
            "2019",
        ];
        let siblings = ["acme-web", "acme-admin"];
        for (name, siblings, depth) in [
            ("acme-web", &siblings[..], 3),
            ("acme-admin", &siblings[..], 3),
            ("acme-mobile", &["acme-mobile"][..], 2),
            ("acme-shop-2019", &["acme-shop-2019"][..], 5),
        ] {
            let narrow = label(name, siblings, &short[..depth], &[]);
            assert_eq!(narrow, label(name, siblings, &long[..depth], &[]), "{name}");
        }
        assert_eq!(
            label("acme-shop-2019", &["acme-shop-2019"], &long, &[]),
            "Sh"
        );
    }

    #[test]
    fn the_labels_of_siblings_differ() {
        let path = ["Client work", "Acme", "Web"];
        let siblings = ["acme-web", "acme-admin"];
        assert_eq!(label("acme-web", &siblings, &path, &[]), "We");
        assert_eq!(label("acme-admin", &siblings, &path, &[]), "Ad");
        assert_eq!(
            label("acme-mobile", &["acme-mobile"], &path[..2], &[]),
            "Mo"
        );
        assert_eq!(label("globex-api", &["globex-api"], &path[..1], &[]), "GA");
        let siblings = ["dotfiles", "docs"];
        let taken = vec![label("dotfiles", &siblings, &[], &[])];
        assert_eq!(taken[0], "Do");
        assert_eq!(label("docs", &siblings, &[], &taken), "Dc");
    }
}
