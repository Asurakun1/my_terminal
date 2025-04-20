use std::rc::Rc;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::Block,
};

pub struct MainLayout<'a> {
    pub left: Block<'a>,
    pub right: Block<'a>,
    pub layout: Layout,
    pub area: Rc<[Rect]>,
}

impl<'a> MainLayout<'a> {
    pub fn new() -> Self {
        Self {
            left: Block::bordered().style(Style::new().fg(Color::Green)).title("Dictionary (辞書)"),
            right: Block::bordered().style(Style::new().fg(Color::Green)),
            layout: Layout::default()
                .direction(ratatui::layout::Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Fill(50)]),
            area: Rc::new([Rect::new(0, 0, 0, 0)]),
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        self.area = self.layout.split(frame.area());
        frame.render_widget(&self.left, self.area[0]);
        frame.render_widget(&self.right, self.area[1]);
    }
}
