use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListState, Paragraph, Wrap},
};

#[derive(Default)]
pub struct MainSelection<'a> {
    layout: Layout,
    list: List<'a>,
    state: ListState,
    p1_placeholder: Paragraph<'a>,
}

impl<'a> MainSelection<'a> {
    pub fn new(items: [&'static str; 4]) -> Self {
        MainSelection {
            layout: Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(30), Constraint::Max(3)]),
            list: List::new(items).block(Block::new().borders(Borders::BOTTOM)).highlight_style(Style::new().reversed()),
            state: ListState::default().with_selected(Some(0)),
            p1_placeholder: Paragraph::new("すみません今からは何も動いていないです。\n「Ｑ」のボッタンを押すならプログラムを終了します。").wrap(Wrap { trim: true }),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let layout = self.layout.split(area);
        frame.render_stateful_widget(&self.list, layout[0], &mut self.state);
        frame.render_widget(&self.p1_placeholder, layout[1]);
    }

    pub fn clear_highlight(&mut self) {
        self.list = self.list.clone().highlight_style(Style::new())
    }

    pub fn show_highlight(&mut self) {
        self.list = self.list.clone().highlight_style(Style::new().reversed())
    }

    pub fn get_state(&mut self) -> &mut ListState {
        &mut self.state
    }
}
