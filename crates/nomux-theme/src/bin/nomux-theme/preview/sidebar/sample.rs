//! The one set of sample data of the sidebar mock-ups, and the frames that
//! show it in one state each. All directions render the same data.

use super::super::glyph::State;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Agent,
    Shell,
    /// A Team: the children are the members, the lead first.
    Team,
}

#[derive(Clone)]
pub struct Session {
    pub name: &'static str,
    pub kind: Kind,
    pub state: State,
    pub doing: &'static str,
    pub age: &'static str,
    pub lead: bool,
    pub children: Vec<Session>,
    pub open: bool,
    pub selected: bool,
    pub hover: bool,
}

impl Session {
    fn new(
        name: &'static str,
        kind: Kind,
        state: State,
        doing: &'static str,
        age: &'static str,
    ) -> Self {
        Self {
            name,
            kind,
            state,
            doing,
            age,
            lead: false,
            children: Vec::new(),
            open: false,
            selected: false,
            hover: false,
        }
    }

    /// What the session does. The lead of a Team says that it leads.
    pub fn described(&self) -> String {
        if self.lead {
            format!("lead · {}", self.doing)
        } else {
            self.doing.to_owned()
        }
    }

    /// The most urgent state of the children.
    pub fn worst_child(&self) -> Option<State> {
        self.children.iter().map(|child| child.state).min()
    }

    fn needs_you(&self) -> usize {
        usize::from(self.kind != Kind::Team && self.state == State::NeedsYou)
            + self.children.iter().map(Session::needs_you).sum::<usize>()
    }
}

#[derive(Clone)]
pub struct Worktree {
    pub name: &'static str,
    pub branch: &'static str,
    pub changes: Option<u32>,
    pub sessions: Vec<Session>,
    pub open: bool,
}

impl Worktree {
    /// The session with the most urgent state. A needs-you session comes
    /// first in its worktree, thus it is first on a tie.
    pub fn worst(&self) -> Option<&Session> {
        self.sessions.iter().min_by_key(|session| session.state)
    }

    /// The sessions in the sequence of the sidebar: the sessions that need
    /// the user first, the others in their own sequence.
    pub fn ordered(&self) -> impl Iterator<Item = &Session> {
        let first = self
            .sessions
            .iter()
            .filter(|session| session.state == State::NeedsYou);
        first.chain(
            self.sessions
                .iter()
                .filter(|session| session.state != State::NeedsYou),
        )
    }

    /// The second line of the worktree: the branch and the count of changes.
    pub fn branch_line(&self) -> String {
        match self.changes {
            Some(changes) => format!("{} · {changes} changes", self.branch),
            None => self.branch.to_owned(),
        }
    }
}

pub struct Project {
    pub name: &'static str,
    pub worktrees: Vec<Worktree>,
}

impl Project {
    pub fn needs_you(&self) -> usize {
        self.worktrees
            .iter()
            .flat_map(|worktree| &worktree.sessions)
            .map(Session::needs_you)
            .sum()
    }

    fn worktree(&mut self, name: &str) -> &mut Worktree {
        self.worktrees
            .iter_mut()
            .find(|worktree| worktree.name == name)
            .expect("the sample has the worktree")
    }

    fn session(&mut self, worktree: &str, name: &str) -> &mut Session {
        self.worktree(worktree)
            .sessions
            .iter_mut()
            .find(|session| session.name == name)
            .expect("the sample has the session")
    }

    fn close_all_but(&mut self, open: &str) {
        for worktree in &mut self.worktrees {
            worktree.open = worktree.name == open;
        }
    }
}

/// The sample of the study: one project, four worktrees.
fn sample() -> Project {
    let mut claude = Session::new(
        "Claude",
        Kind::Agent,
        State::NeedsYou,
        "wants to run npm test",
        "2m",
    );
    claude.selected = true;
    claude.children = vec![Session::new(
        "Explore",
        Kind::Agent,
        State::Working,
        "reads session.rs",
        "1m",
    )];
    let dev = Session::new(
        "dev",
        Kind::Shell,
        State::Working,
        "port 3000 · 12 of 40 tests",
        "9m",
    );
    let worktree = |name, branch, changes, sessions| Worktree {
        name,
        branch,
        changes,
        sessions,
        open: true,
    };
    let mut main = worktree(
        "main",
        "main",
        None,
        vec![Session::new("zsh", Kind::Shell, State::Idle, "idle", "1h")],
    );
    main.open = false;
    Project {
        name: "acme-web",
        worktrees: vec![
            worktree("fix-login", "fix/login", Some(3), vec![claude, dev]),
            worktree(
                "billing-export",
                "feat/billing-export",
                None,
                vec![Session::new(
                    "Codex",
                    Kind::Agent,
                    State::Done,
                    "wrote 4 files",
                    "12m",
                )],
            ),
            worktree(
                "upgrade-deps",
                "chore/deps",
                None,
                vec![Session::new(
                    "Qwen Code",
                    Kind::Agent,
                    State::Error,
                    "build failed",
                    "4m",
                )],
            ),
            main,
        ],
    }
}

/// One state of the sidebar that a reviewer needs to judge a direction.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Frame {
    Rest,
    Hover,
    OpenAndClosed,
    Team,
    LongText,
}

impl Frame {
    pub const ALL: [Self; 5] = [
        Self::Rest,
        Self::Hover,
        Self::OpenAndClosed,
        Self::Team,
        Self::LongText,
    ];

    pub const fn caption(self) -> &'static str {
        match self {
            Self::Rest => "(a) Rest: the sample data, the Claude row selected",
            Self::Hover => {
                "(b) Pointer over the dev row: the items for the pointer appear, the row keeps its height"
            }
            Self::OpenAndClosed => {
                "(c) fix-login open with the child of Claude open, billing-export closed with its aggregate state"
            }
            Self::Team => {
                "(d) A Team: lead Claude with the members Codex, Gemini CLI and Qwen Code"
            }
            Self::LongText => {
                "(e) A long worktree name and a long reason: both truncate, the row keeps its height"
            }
        }
    }

    pub fn project(self) -> Project {
        let mut project = sample();
        match self {
            Self::Rest => {}
            Self::Hover => project.session("fix-login", "dev").hover = true,
            Self::OpenAndClosed => {
                project.close_all_but("fix-login");
                project.session("fix-login", "Claude").open = true;
            }
            Self::Team => {
                project.close_all_but("fix-login");
                let member =
                    |name, state, doing| Session::new(name, Kind::Agent, state, doing, "3m");
                let mut lead = member("Claude", State::NeedsYou, "asks for approval");
                lead.lead = true;
                let mut team = Session::new("Team", Kind::Team, State::NeedsYou, "4 members", "3m");
                team.open = true;
                team.selected = true;
                team.children = vec![
                    lead,
                    member("Codex", State::Working, "writes tests"),
                    member("Gemini CLI", State::Done, "review done"),
                    member("Qwen Code", State::Idle, "in its own Tab"),
                ];
                project.worktree("fix-login").sessions[0] = team;
            }
            Self::LongText => {
                project.close_all_but("fix-login");
                let worktree = project.worktree("fix-login");
                worktree.name = "fix-session-refresh-when-the-access-token-expires";
                worktree.branch = "fix/session-refresh-when-the-access-token-expires";
                worktree.sessions[0].doing =
                    "wants to run npm run test:integration -- --runInBand --coverage";
            }
        }
        project
    }
}
