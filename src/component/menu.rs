use tui::{
    style::{Color, Modifier, Style},
    widgets::{Block, List, ListItem, ListState},
};

pub struct MenuStyle {
    pub selected: Style,
    pub row: Style,
    /// Prefix for selected item
    pub symbol: &'static str,
    pub menu_title: &'static str,
}

pub struct PopUpMenu {
    state: ListState,
    pub items: Vec<tui::text::Text<'static>>,
    pub style: MenuStyle,
}

impl PopUpMenu {
    pub fn from<T>(items: &[T]) -> Self
    where
        T: ToString,
    {
        let items = items
            .iter()
            .map(|s| {
                let s = s.to_string();
                tui::text::Text::from(s)
            })
            .collect::<Vec<_>>();

        Self {
            items,
            state: ListState::default(),
            style: MenuStyle {
                selected: Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
                row: Style::default().fg(Color::White),
                symbol: ">> ",
                menu_title: "Menu",
            },
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn to_renderable(&self) -> List {
        let items: Vec<_> = self
            .items
            .iter()
            .map(|em| ListItem::new(em.clone()).style(self.style.row))
            .collect();
        let block = Block::default()
            .title(self.style.menu_title)
            .borders(tui::widgets::Borders::ALL);

        List::new(items)
            .block(block)
            .highlight_style(self.style.selected)
            .highlight_symbol(self.style.symbol)
    }
}
