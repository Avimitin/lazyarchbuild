use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Style, Modifier},
    widgets,
    Frame,
};

#[derive(Debug)]
pub struct PkgInfoTableStyle {
    title: Style,
    row: Style,
    selected: Style,
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

#[derive(Debug)]
pub struct PkgInfo {
    name: Box<str>,
    assignee: Box<str>,
    marks: Vec<Box<str>>,
}

#[derive(Debug)]
pub struct PkgInfoTable {
    title: &'static str,
    cursor: widgets::TableState,
    data: Vec<PkgInfo>,
    style: PkgInfoTableStyle,
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

    pub fn draw<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let layout = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .margin(1)
            .split(frame.size());

        let title = vec![
            widgets::Cell::from("Pkgname").style(self.style.title),
            widgets::Cell::from("Assignee").style(self.style.title),
            widgets::Cell::from("Marks").style(self.style.title),
        ];
        let header = widgets::Row::new(title)
            .style(self.style.row)
            .height(1);

        let rows = self.data.iter().map(|pkg| {
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

        frame.render_stateful_widget(table, layout[0], &mut self.cursor);
    }
}
