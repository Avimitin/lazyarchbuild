use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    terminal, widgets,
};

use crate::component;

pub fn draw_package_table<B: Backend>(
    terminal: &mut terminal::Terminal<B>,
    data: &mut component::packages::PkgInfoTable,
) -> anyhow::Result<()> {
    terminal.draw(|frame| {
        let layout = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .margin(1)
            .split(frame.size());

        let title = vec![
            widgets::Cell::from("Pkgname").style(data.style.title),
            widgets::Cell::from("Assignee").style(data.style.title),
            widgets::Cell::from("Marks").style(data.style.title),
        ];
        let header = widgets::Row::new(title).style(data.style.row).height(1);

        let rows = data.data.iter().map(|pkg| {
            let pkg = vec![
                widgets::Cell::from(pkg.name()),
                widgets::Cell::from(pkg.assignee()),
                widgets::Cell::from(pkg.marks().join(" ")),
            ];
            widgets::Row::new(pkg).height(1)
        });

        let table = widgets::Table::new(rows)
            .header(header)
            .block(
                widgets::Block::default()
                    .borders(widgets::Borders::ALL)
                    .title(data.title()),
            )
            .highlight_style(data.style.selected)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Percentage(15),
                Constraint::Min(20),
                Constraint::Percentage(70),
            ]);

        frame.render_stateful_widget(table, layout[0], &mut data.cursor);
    })?;
    Ok(())
}
