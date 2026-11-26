//! The one sample tree of groups and projects of the project rail, and the
//! frames that show it in one state each.

use super::super::glyph::State;
use super::sample::Frame;

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
    pub hover: bool,
    pub focus: bool,
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

impl Entry {
    fn new(name: &'static str, state: State) -> Self {
        Self {
            name,
            state,
            selected: false,
            hover: false,
            focus: false,
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

    /// The group holds the selected project, at any depth.
    pub fn holds_selected(&self) -> bool {
        self.nodes.iter().any(|node| match node {
            Node::Group(group) => group.holds_selected(),
            Node::Project(entry) => entry.selected,
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

    fn entry_mut(&mut self, name: &str) -> &mut Entry {
        fn find<'a>(group: &'a mut Group, name: &str) -> Option<&'a mut Entry> {
            group.nodes.iter_mut().find_map(|node| match node {
                Node::Group(inner) => find(inner, name),
                Node::Project(entry) => (entry.name == name).then_some(entry),
            })
        }
        find(self, name).expect("the sample has the project")
    }

    fn close_all(&mut self) {
        for node in &mut self.nodes {
            if let Node::Group(group) = node {
                group.open = false;
                group.close_all();
            }
        }
    }

    /// Opens each group on the path.
    fn open(&mut self, path: &[&str]) {
        for depth in 1..=path.len() {
            self.find_mut(&path[..depth]).open = true;
        }
    }
}

/// The path of the group that holds the selected project.
const WEB: [&str; 3] = ["Client work", "Acme", "Web"];

/// The sample: nine projects in four groups, one group in a group in a
/// group. The selected project is the project of the sidebar sample. At
/// rest the groups on the path of the selected project are open.
fn sample() -> Group {
    let project = |name, state| Node::Project(Entry::new(name, state));
    let selected = Frame::Rest.project();
    let web = Group::new(
        "Web",
        true,
        vec![
            Node::Project(Entry {
                selected: true,
                ..Entry::new(
                    selected.name,
                    selected.worst().expect("the sample has Sessions"),
                )
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

/// Web holds Legacy, Legacy holds 2019, 2019 holds acme-shop-2019: five
/// levels of groups.
fn add_five_levels(tree: &mut Group) {
    let year = Group::new(
        "2019",
        true,
        vec![Node::Project(Entry::new("acme-shop-2019", State::Idle))],
    );
    let legacy = Group::new("Legacy", true, vec![Node::Group(year)]);
    tree.find_mut(&WEB).nodes.push(Node::Group(legacy));
}

/// One state of the project rail that the page shows.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GroupFrame {
    Closed,
    OneOpen,
    TwoLevels,
    Rest,
    FiveLevels,
    Empty,
    PointerOnProject,
    PointerOnGroup,
    Scroll,
    Expanded,
}

impl GroupFrame {
    pub const ALL: [Self; 10] = [
        Self::Closed,
        Self::OneOpen,
        Self::TwoLevels,
        Self::Rest,
        Self::FiveLevels,
        Self::Empty,
        Self::PointerOnProject,
        Self::PointerOnGroup,
        Self::Scroll,
        Self::Expanded,
    ];

    pub const fn caption(self) -> &'static str {
        match self {
            Self::Closed => {
                "(a) All groups closed. Client work holds the selected project and a project that needs you"
            }
            Self::OneOpen => "(b) Personal open. blog has the keyboard focus",
            Self::TwoLevels => "(c) Two levels open: Client work and Acme. Web is closed",
            Self::Rest => "(d) Three levels open: the selected project acme-web in Web",
            Self::FiveLevels => "(e) Five levels open: Web, Legacy, 2019",
            Self::Empty => "(f) Personal open with the empty group Later, open",
            Self::PointerOnProject => "(g) Pointer on acme-admin",
            Self::PointerOnGroup => "(h) Pointer on Personal",
            Self::Scroll => "(i) Taller than the window: the rail scrolls",
            Self::Expanded => {
                "(j) The expanded rail with long names: the faces and the labels stay"
            }
        }
    }

    /// The class of the figure: room for a tooltip, a window of fixed
    /// height, or the width of the expanded rail.
    pub const fn class(self) -> &'static str {
        match self {
            Self::PointerOnProject | Self::PointerOnGroup => "pointer",
            Self::Scroll => "scroll",
            Self::Expanded => "expanded",
            _ => "",
        }
    }

    /// The rail shows the names of its tiles.
    pub fn expanded(self) -> bool {
        self == Self::Expanded
    }

    /// The tree of groups in this frame.
    pub fn tree(self) -> Group {
        let mut tree = sample();
        match self {
            Self::Rest => {}
            Self::Closed => tree.close_all(),
            Self::OneOpen => {
                tree.close_all();
                tree.open(&["Personal"]);
                tree.entry_mut("blog").focus = true;
            }
            Self::TwoLevels => {
                tree.close_all();
                tree.open(&WEB[..2]);
            }
            Self::FiveLevels => add_five_levels(&mut tree),
            Self::Empty => {
                tree.close_all();
                let personal = tree.find_mut(&["Personal"]);
                personal.open = true;
                personal
                    .nodes
                    .push(Node::Group(Group::new("Later", true, vec![])));
            }
            Self::PointerOnProject => tree.entry_mut("acme-admin").hover = true,
            Self::PointerOnGroup => {
                tree.close_all();
                tree.find_mut(&["Personal"]).hover = true;
            }
            Self::Scroll => {
                add_five_levels(&mut tree);
                tree.open(&["Personal"]);
            }
            Self::Expanded => {
                add_five_levels(&mut tree);
                tree.find_mut(&WEB[..2]).name = "Acme Corporation customer platform";
                tree.find_mut(&["Client work", "Acme Corporation customer platform", "Web"])
                    .name = "Web storefront and checkout";
                tree.entry_mut("globex-api").name = "globex-api-gateway-and-billing";
            }
        }
        tree
    }
}
