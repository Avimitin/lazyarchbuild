use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    terminal,
    text::{Span, Spans},
    widgets::{self, Block, BorderType, Borders, Paragraph},
};

use crate::component;

pub fn draw_welcome_page<B: Backend>(terminal: &mut terminal::Terminal<B>) -> anyhow::Result<()> {
    terminal.draw(|frame| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(40),
                    Constraint::Percentage(20),
                    Constraint::Percentage(40),
                ]
                .as_ref(),
            )
            .split(frame.size());

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(40),
                    Constraint::Percentage(20),
                    Constraint::Percentage(40),
                ]
                .as_ref(),
            )
            .split(chunks[1]);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let welcome_text = vec![
            Spans::from(Span::styled(
                "Fetching Package Info",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Spans::from(""),
            Spans::from(""),
            Spans::from(""),
            Spans::from("Please wait a secs, pulling data..."),
            Spans::from(""),
            Spans::from(""),
            Spans::from(""),
        ];

        let paragraph = Paragraph::new(welcome_text)
            .style(
                Style::default()
                    .bg(tui::style::Color::White)
                    .fg(tui::style::Color::Blue),
            )
            .block(block)
            .alignment(tui::layout::Alignment::Center);

        frame.render_widget(paragraph, chunks[1])
    })?;

    Ok(())
}

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
                Constraint::Percentage(35),
                Constraint::Min(20),
                Constraint::Percentage(45),
            ]);

        frame.render_stateful_widget(table, layout[0], &mut data.cursor);
    })?;
    Ok(())
}
