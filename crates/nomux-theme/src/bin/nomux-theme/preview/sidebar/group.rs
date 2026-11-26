//! The one sample tree of groups and projects of the group alternatives,
//! and the frames that show it in one state each.

use super::super::glyph::State;
use super::sample::{Frame, Project};

pub enum Node {
    Group(Group),
    Project(Entry),
}

/// A group holds projects and other groups, to any depth.
pub struct Group {
    pub name: &'static str,
    pub open: bool,
    pub hover: bool,
    pub nodes: Vec<Node>,
}

/// A project in the tree of groups.
pub struct Entry {
    pub name: &'static str,
    /// The most urgent state of the project.
    pub state: State,
    pub selected: bool,
}

impl Node {
    fn worst(&self) -> Option<State> {
        match self {
            Self::Group(group) => group.worst(),
            Self::Project(entry) => Some(entry.state),
        }
    }

    fn projects(&self) -> usize {
        match self {
            Self::Group(group) => group.projects(),
            Self::Project(_) => 1,
        }
    }
}

impl Group {
    fn new(name: &'static str, open: bool, nodes: Vec<Node>) -> Self {
        Self {
            name,
            open,
            hover: false,
            nodes,
        }
    }

    /// The aggregate state: the most urgent state of each project in the
    /// group and in its groups.
    pub fn worst(&self) -> Option<State> {
        self.nodes.iter().filter_map(Node::worst).min()
    }

    /// The count of the projects in the group and in its groups.
    pub fn projects(&self) -> usize {
        self.nodes.iter().map(Node::projects).sum()
    }

    /// The most urgent state of the projects that are not in `inner`.
    pub fn worst_outside(&self, inner: &Self) -> Option<State> {
        self.nodes
            .iter()
            .filter_map(|node| match node {
                Node::Group(group) if std::ptr::eq(group, inner) => None,
                Node::Group(group) => group.worst_outside(inner),
                Node::Project(entry) => Some(entry.state),
            })
            .min()
    }

    /// The group at the end of a path of group names.
    pub fn find(&self, path: &[&str]) -> &Self {
        path.iter().fold(self, |group, name| {
            group
                .nodes
                .iter()
                .find_map(|node| match node {
                    Node::Group(inner) if inner.name == *name => Some(inner),
                    _ => None,
                })
                .expect("the sample has the group")
        })
    }

    fn find_mut(&mut self, path: &[&str]) -> &mut Self {
        path.iter().fold(self, |group, name| {
            group
                .nodes
                .iter_mut()
                .find_map(|node| match node {
                    Node::Group(inner) if inner.name == *name => Some(inner),
                    _ => None,
                })
                .expect("the sample has the group")
        })
    }

    /// The selected project.
    pub fn selected(&self) -> &Entry {
        self.find_selected()
            .expect("the sample has a selected project")
    }

    pub fn find_selected(&self) -> Option<&Entry> {
        self.nodes.iter().find_map(|node| match node {
            Node::Group(group) => group.find_selected(),
            Node::Project(entry) => entry.selected.then_some(entry),
        })
    }

    fn selected_mut(&mut self) -> &mut Entry {
        fn find(group: &mut Group) -> Option<&mut Entry> {
            group.nodes.iter_mut().find_map(|node| match node {
                Node::Group(inner) => find(inner),
                Node::Project(entry) => entry.selected.then_some(entry),
            })
        }
        find(self).expect("the sample has a selected project")
    }
}

/// The path of the group that holds the selected project at rest.
pub const WEB: [&str; 3] = ["Client work", "Acme", "Web"];
/// The path of the deepest group of the frame "Depth".
pub const DEEPEST: [&str; 5] = ["Client work", "Acme", "Web", "Legacy", "2019"];

/// The sample: nine projects in four groups, one group in a group in a
/// group. The selected project is the project of the sidebar sample.
fn sample() -> Group {
    let project = |name, state| {
        Node::Project(Entry {
            name,
            state,
            selected: false,
        })
    };
    let selected = Frame::Rest.project();
    let web = Group::new(
        "Web",
        true,
        vec![
            Node::Project(Entry {
                name: selected.name,
                state: selected.worst().expect("the sample has Sessions"),
                selected: true,
            }),
            project("acme-admin", State::Working),
        ],
    );
    let acme = Group::new(
        "Acme",
        true,
        vec![Node::Group(web), project("acme-mobile", State::Idle)],
    );
    Group::new(
        "",
        true,
        vec![
            Node::Group(Group::new(
                "Client work",
                true,
                vec![Node::Group(acme), project("globex-api", State::Error)],
            )),
            Node::Group(Group::new(
                "Personal",
                false,
                vec![
                    project("dotfiles", State::Idle),
                    project("blog", State::Done),
                    project("recipes", State::NoProcess),
                ],
            )),
            project("nomux", State::Working),
            project("themes", State::Idle),
        ],
    )
}

/// One state of the tree of groups that the page shows.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GroupFrame {
    Rest,
    Aggregate,
    Depth,
    Empty,
    LongNames,
    Selected,
}

impl GroupFrame {
    pub const ALL: [Self; 6] = [
        Self::Rest,
        Self::Aggregate,
        Self::Depth,
        Self::Empty,
        Self::LongNames,
        Self::Selected,
    ];

    pub const fn caption(self) -> &'static str {
        match self {
            Self::Rest => {
                "(a) Rest: Client work, Acme and Web open, Personal closed. acme-web is selected, its worktrees are closed"
            }
            Self::Aggregate => {
                "(b) Client work closed: it shows the needs-you state of acme-web, three levels down. The pointer is on Personal"
            }
            Self::Depth => {
                "(c) Five levels of groups: Web holds Legacy, Legacy holds 2019, 2019 holds acme-shop-2019"
            }
            Self::Empty => "(d) Personal open with the empty group Later",
            Self::LongNames => "(e) Long names of groups and of the selected project",
            Self::Selected => {
                "(f) The selected project acme-web with fix-login open: its Sessions and the selected Claude row"
            }
        }
    }

    /// The tree of groups in this frame.
    pub fn tree(self) -> Group {
        let mut tree = sample();
        match self {
            Self::Rest | Self::Selected => {}
            Self::Aggregate => {
                tree.find_mut(&WEB[..1]).open = false;
                tree.find_mut(&["Personal"]).hover = true;
            }
            Self::Depth => {
                let leaf = Node::Project(Entry {
                    name: "acme-shop-2019",
                    state: State::Idle,
                    selected: false,
                });
                let year = Group::new("2019", true, vec![leaf]);
                let legacy = Group::new("Legacy", true, vec![Node::Group(year)]);
                tree.find_mut(&WEB).nodes.push(Node::Group(legacy));
            }
            Self::Empty => {
                let personal = tree.find_mut(&["Personal"]);
                personal.open = true;
                personal
                    .nodes
                    .push(Node::Group(Group::new("Later", false, vec![])));
            }
            Self::LongNames => {
                tree.find_mut(&WEB[..2]).name = "Acme Corporation customer platform";
                tree.find_mut(&["Client work", "Acme Corporation customer platform", "Web"])
                    .name = "Storefront and checkout for the web";
                tree.selected_mut().name = "acme-web-storefront-and-checkout-2026";
            }
        }
        tree
    }

    /// The selected project with its worktrees: open only in the frame
    /// "Selected".
    pub fn project(self) -> Project {
        let mut project = Frame::Rest.project();
        project.name = self.tree().selected().name;
        if self != Self::Selected {
            project.close_all_but("");
        }
        project
    }
}
