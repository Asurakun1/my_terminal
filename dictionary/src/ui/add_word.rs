use std::error::Error;
use std::rc::Rc;
use std::time::Duration;

use ratatui::crossterm::event::poll;
use ratatui::layout::{Position, Rect};
use ratatui::text::{Line, Text};
use ratatui::widgets::{List, Paragraph};
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
    menu_state: Menu,
    input: [String; 3],
}

impl<'a> AddWord<'a> {
    pub fn new() -> Self {
        Self {
            layout: Layout::new(Direction::Vertical, [Constraint::Percentage(20)]),
            menu: ListState::default().with_selected(Some(0)),
            list: List::new(ADD_WORD_MENU)
                .block(Block::bordered().title("言葉追加"))
                .highlight_style(Style::new().reversed()),
            menu_state: Menu::Cotoba,
            input: [String::new(), String::new(), String::new()],
        }
    }

    pub fn set_menu(&mut self, menu: Menu) {
        self.menu_state = menu;
    }

    pub fn get_menu(&self) -> &Menu {
        &self.menu_state
    }

    pub fn render(&mut self, main_layout: &MainLayout, frame: &mut Frame) {
        let area = self
            .layout
            .split(main_layout.right.inner(main_layout.area[1]));

        /*
        splits the layout into 2 parts
         */
        let layer2 = Layout::new(
            Direction::Horizontal,
            [Constraint::Max(25), Constraint::Percentage(50)],
        )
        .split(area[0]);

        let input_block = Block::bordered().title("入力");

        self.render_line_state(frame, &input_block, &layer2);
        frame.render_stateful_widget(&self.list, layer2[0], &mut self.menu);
        frame.render_widget(&input_block, layer2[1]);
    }

    fn render_line_state(&self, frame: &mut Frame, input_block: &Block, layer2: &Rc<[Rect]>) {
        let input_area = Layout::new(
            Direction::Vertical,
            [
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(30),
            ],
        )
        .split(input_block.inner(layer2[1]));

        let line1 = Line::raw(&self.input[0]);
        let line2 = Line::raw(&self.input[1]);

        frame.render_widget(line1, input_area[0]);
        frame.render_widget(line2, input_area[0]);
        self.render_cursor_position(frame, &input_area);
    }

    fn render_cursor_position(&self, frame: &mut Frame, input_area: &Rc<[Rect]>) {
        match self.menu_state {
            Menu::Cotoba => {
                frame.set_cursor_position(Position::new(
                    input_area[0].x + self.input[0].chars().count() as u16,
                    input_area[0].y,
                ));
            }
            Menu::Yomikata => {
                frame.set_cursor_position(Position::new(input_area[0].x, input_area[0].y + 1));
            }

            Menu::Teigi => {
                frame.set_cursor_position(Position::new(input_area[0].x, input_area[0].y + 2));
            }
            _ => {}
        }
    }

    /*
    Lists and events handled as lists will most likely be scrapped.
    This is because the add_word is supposed to take inputs of three separate modifiable categories
    The word
    The reading
    The definition

    Although a list may be suitable, it is still worthwhile to play around with the ratatui library to see what could work for
    List-block type inputs
    */

    fn current_selection(&mut self) {
        if let Some(index) = self.menu.selected() {
            match index {
                0 => {
                    self.set_menu(Menu::Cotoba);
                }

                1 => {
                    self.set_menu(Menu::Yomikata);
                }

                2 => {
                    self.set_menu(Menu::Teigi);
                }

                _ => {}
            }
        }
    }

    fn handle_inputs(&mut self, ch: char) {
        match self.menu_state {
            Menu::Cotoba => {
                self.input[0] = self.input[0].to_string() + &ch.to_string();
            }
            _ => {}
        }
    }

    fn delete_inputs(&mut self) -> Option<()> {
        match self.menu_state {
            Menu::Cotoba => match self.input[0].char_indices().next_back() {
                Some((i, _)) => {
                    self.input[0] = self.input[0][..i].to_string();
                }
                None => {}
            },

            _ => {}
        }
        Some(())
    }

    pub fn handle_events(&mut self) -> Result<(), Box<dyn Error>> {
        self.current_selection();
        if poll(Duration::from_millis(100))? {
            match event::read() {
                Ok(Event::Key(key_code)) => {
                    if key_code.kind == KeyEventKind::Press {
                        match key_code.code {
                            KeyCode::Char('q') => self.menu_state = Menu::Exit,
                            KeyCode::Down | KeyCode::Tab => {
                                self.menu.select_next();
                            }

                            KeyCode::Up => {
                                self.menu.select_previous();
                            }

                            KeyCode::Char(ch) => {
                                self.handle_inputs(ch);
                            }

                            KeyCode::Backspace => {
                                self.delete_inputs().unwrap();
                            }

                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
