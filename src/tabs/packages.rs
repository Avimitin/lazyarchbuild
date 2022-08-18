use crate::component::package;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::Modifier,
    widgets, Frame,
};

#[derive(Clone)]
pub struct InformationTableStyle {
    normal: tui::style::Style,
    selected: tui::style::Style,
    header: tui::style::Style,
}

impl std::default::Default for InformationTableStyle {
    fn default() -> Self {
        Self {
            selected: tui::style::Style::default().add_modifier(Modifier::REVERSED),
            normal: tui::style::Style::default().bg(tui::style::Color::Black),
            header: tui::style::Style::default()
                .add_modifier(Modifier::UNDERLINED)
                .add_modifier(Modifier::BOLD),
        }
    }
}

pub struct InformationTable {
    title: &'static str,
    cursor_position: widgets::TableState,
    pkg_list: Vec<package::PkgInfo>,
    style: InformationTableStyle,
}

impl InformationTable {
    pub fn from(list: Vec<package::PkgInfo>, user_config: Option<InformationTableStyle>) -> Self {
        Self {
            title: "Package Information",
            pkg_list: list,
            cursor_position: widgets::TableState::default(),
            style: user_config.unwrap_or_default(),
        }
    }

    pub fn next(&mut self) {
        let idx = match self.cursor_position.selected() {
            Some(i) => {
                if i >= self.pkg_list.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.cursor_position.select(Some(idx))
    }

    pub fn previous(&mut self) {
        let idx = match self.cursor_position.selected() {
            Some(i) => {
                if i == 0 {
                    self.pkg_list.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.cursor_position.select(Some(idx))
    }

    pub fn draw_on<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let rects = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .margin(5)
            .split(frame.size());
        let header = vec![
            widgets::Cell::from("Pkgname").style(self.style.header),
            widgets::Cell::from("Assignee").style(self.style.header),
            widgets::Cell::from("Marks").style(self.style.header),
        ];
        let header = widgets::Row::new(header)
            .style(self.style.normal)
            .height(1);

        let rows = self.pkg_list.iter().map(|pkg| {
            let pkg = vec![
                widgets::Cell::from(pkg.name.as_ref()),
                widgets::Cell::from(pkg.assignee.as_ref()),
                widgets::Cell::from(pkg.marks.join(" ")),
            ];
            widgets::Row::new(pkg).height(1)
        });

        let table = widgets::Table::new(rows)
            .header(header)
            .block(
                widgets::Block::default()
                    .borders(widgets::Borders::ALL)
                    .title(self.title),
            )
            .highlight_style(self.style.selected)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Percentage(15),
                Constraint::Min(20),
                Constraint::Percentage(70),
            ]);

        frame.render_stateful_widget(table, rects[0], &mut self.cursor_position);
    }
}

pub fn draw_packages() -> InformationTable {
    InformationTable::from(
        vec![
            package::PkgInfo {
                name: "rust".into(),
                assignee: "rvalue".into(),
                marks: vec!["failing".to_string()],
            },
            package::PkgInfo {
                name: "libaio".into(),
                assignee: "sterprim".into(),
                marks: vec![
                    "failing".to_string(),
                    "noqemu".to_string(),
                    "outdated".to_string(),
                    "stuck".to_string(),
                ],
            },
            package::PkgInfo {
                name: "exiv2".into(),
                assignee: "asuna".into(),
                marks: vec![
                    "failing".to_string(),
                    "stuck".to_string(),
                ],
            },
            package::PkgInfo {
                name: "firefox".into(),
                assignee: "东东".into(),
                marks: vec!["failing".to_string(), "rotten".to_string()],
            },
            package::PkgInfo {
                name: "lldb".into(),
                assignee: "melon".into(),
                marks: vec!["failing".to_string()],
            },
        ],
        None,
    )
}
