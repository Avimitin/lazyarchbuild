use std::collections::HashMap;

use tui::{
    style::{Modifier, Style},
    widgets,
};
use typed_builder::TypedBuilder;

#[derive(Debug)]
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

#[derive(Debug, TypedBuilder)]
pub struct PkgInfo {
    name: Box<str>,
    #[builder(default_code = "\"\".into()")]
    assignee: Box<str>,
    #[builder(default)]
    marks: Vec<Box<str>>,
    #[builder(default_code = "false")]
    rotten: bool,
    #[builder(default_code = "\"\".into()")]
    process: Box<str>,
}

impl PkgInfo {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn assignee(&self) -> &str {
        &self.assignee
    }

    pub fn marks(&self) -> &[Box<str>] {
        &self.marks
    }

    pub fn is_rotten(&self) -> bool {
        self.rotten
    }

    pub fn current_process(&self) -> &str {
        &self.process
    }
}

#[derive(Debug)]
pub struct PkgInfoTable {
    title: &'static str,
    pub cursor: widgets::TableState,
    pub data: HashMap<Box<str>, PkgInfo>,
    pub style: PkgInfoTableStyle,
}

impl std::default::Default for PkgInfoTable {
    fn default() -> Self {
        Self {
            title: "Arch Linux RISC-V Packages Status",
            cursor: widgets::TableState::default(),
            data: HashMap::new(),
            style: PkgInfoTableStyle::default(),
        }
    }
}

impl PkgInfoTable {
    pub fn title<'a>(&self) -> &'a str {
        &*self.title
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
}
