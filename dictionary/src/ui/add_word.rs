use ratatui::widgets::List;
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, ListState},
};

use super::main_layout::MainLayout;
const ADD_WORD_MENU: [&str; 3] = ["言葉：　", "読み方：　", "定義：　"];

pub enum Menu {
    Cotoba,
    Yomikata,
    Teigi,
    Exit,
}

pub struct AddWord<'a> {
    layout: Layout,
    menu: ListState,
    list: List<'a>,
    pub menu_state: Menu,
}

impl<'a> AddWord<'a> {
    pub fn new() -> Self {
        Self {
            layout: Layout::new(Direction::Vertical, [Constraint::Percentage(20)]),
            menu: ListState::default(),
            list: List::new(ADD_WORD_MENU)
                .block(Block::bordered().title("言葉追加"))
                .highlight_style(Style::new().reversed()),
            menu_state: Menu::Cotoba,
        }
    }

    pub fn render(&mut self, main_layout: &MainLayout, frame: &mut Frame) {
        let area = self
            .layout
            .split(main_layout.right.inner(main_layout.area[1]));

        frame.render_stateful_widget(&self.list, area[0], &mut self.menu);
    }
    pub fn handle_events(&mut self) {
        match event::read() {
            Ok(Event::Key(key_code)) => {
                if key_code.kind == KeyEventKind::Press {
                    match key_code.code {
                        KeyCode::Char('q') => self.menu_state = Menu::Exit,
                        KeyCode::Down => {
                            self.menu.select_next();
                        }

                        KeyCode::Up => {
                            self.menu.select_previous();
                        }

                        _ => {
                            if let Some(index) = self.menu.selected() {
                                match index {
                                    0 => {}

                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
