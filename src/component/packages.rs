use crate::types::Mark;
use derive_builder::Builder;
use tui::{
    style::{Modifier, Style},
    widgets,
};

#[derive(Debug, Builder)]
pub struct PkgInfoTableStyle {
    pub title: Style,
    pub row: Style,
    pub selected: Style,
}

impl std::default::Default for PkgInfoTableStyle {
    fn default() -> Self {
        Self {
            title: Style::default().add_modifier(Modifier::BOLD),
            row: Style::default(),
            selected: Style::default().add_modifier(Modifier::REVERSED),
        }
    }
}

#[derive(Debug, Builder)]
#[builder(pattern = "owned")]
pub struct PkgInfo {
    #[builder(default = "\"\".into()")]
    pub name: Box<str>,
    #[builder(setter(strip_option), default = "None")]
    pub assignee: Option<Box<str>>,
    #[builder(default = "Vec::new()")]
    pub marks: Vec<Mark>,
    #[builder(default = "false")]
    pub rotten: bool,
    #[builder(setter(strip_option), default = "None")]
    pub process: Option<Box<str>>,
}

impl PkgInfo {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn has_assignee(&self) -> bool {
        self.assignee.is_some()
    }

    pub fn has_process(&self) -> bool {
        self.process.is_some()
    }

    pub fn has_marks(&self) -> bool {
        !self.marks.is_empty()
    }

    pub fn assignee(&self) -> &str {
        if let Some(ref ae) = self.assignee {
            ae
        } else {
            ""
        }
    }

    pub fn marks(&self) -> Box<[&str]> {
        self.marks
            .iter()
            .map(|elem| elem.name.as_ref())
            .collect::<Box<_>>()
    }

    pub fn is_rotten(&self) -> bool {
        self.rotten
    }

    pub fn current_process(&self) -> &str {
        if let Some(ref pc) = self.process {
            pc
        } else {
            ""
        }
    }
}

#[derive(Debug)]
pub struct PkgInfoTable {
    title: &'static str,
    pub cursor: widgets::TableState,
    pub data: Vec<PkgInfo>,
    pub style: PkgInfoTableStyle,
}

impl std::default::Default for PkgInfoTable {
    fn default() -> Self {
        Self {
            title: "Arch Linux RISC-V Packages Status",
            cursor: widgets::TableState::default(),
            data: Vec::new(),
            style: PkgInfoTableStyle::default(),
        }
    }
}

impl PkgInfoTable {
    pub fn title<'a>(&self) -> &'a str {
        self.title
    }

    pub fn next(&mut self) {
        let idx = self.cursor.selected();
        if idx.is_none() {
            self.cursor.select(Some(0));
            return;
        }

        let idx = idx.unwrap();

        if idx >= self.data.len() - 1 {
            self.cursor.select(Some(0))
        } else {
            self.cursor.select(Some(idx + 1))
        }
    }

    pub fn previous(&mut self) {
        let idx = self.cursor.selected();
        if idx.is_none() {
            self.cursor.select(Some(0));
            return;
        }

        let idx = idx.unwrap();

        if idx == 0 {
            self.cursor.select(Some(self.data.len() - 1))
        } else {
            self.cursor.select(Some(idx - 1))
        }
    }

    pub fn beginning(&mut self) {
        self.cursor.select(Some(0))
    }

    pub fn end(&mut self) {
        self.cursor.select(Some(self.data.len() - 1))
    }
}
