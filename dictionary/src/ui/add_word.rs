use std::error::Error;
use std::rc::Rc;
use std::time::Duration;

use ratatui::crossterm::event::poll;
use ratatui::layout::Rect;
use ratatui::widgets::List;
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, ListState},
};
use tui_textarea::TextArea;

use super::main_layout::MainLayout;
const ADD_WORD_MENU: [&str; 3] = ["言葉：　", "読み方：　", "定義：　"];

pub enum Menu {
    Menu,
    Cotoba,
    Yomikata,
    Teigi,
    Exit,
}

enum Editing {
    On,
    Off,
}

pub struct AddWord<'a> {
    layout: Layout,
    menu: ListState,
    list: List<'a>,
    menu_state: Menu,
    input: [String; 3],
    editing: Editing,
    word_area: TextArea<'a>,
}

impl<'a> AddWord<'a> {
    pub fn new() -> Self {
        Self {
            layout: Layout::new(Direction::Vertical, [Constraint::Percentage(20)]),
            menu: ListState::default().with_selected(Some(0)),
            list: List::new(ADD_WORD_MENU)
                .block(Block::bordered().title("言葉追加"))
                .highlight_style(Style::new().reversed()),
            menu_state: Menu::Menu,
            input: [String::new(), String::new(), String::new()],
            editing: Editing::Off,
            word_area: TextArea::default(),
        }
    }

    fn clear_highlight(&mut self) {
        self.list = self.list.to_owned().highlight_style(Style::new());
    }

    fn show_highlight(&mut self) {
        self.list = self
            .list
            .to_owned()
            .highlight_style(Style::new().reversed());
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
            [Constraint::Max(25), Constraint::Percentage(60)],
        )
        .split(area[0]);

        let input_block = Block::bordered().title("入力");

        frame.render_stateful_widget(&self.list, layer2[0], &mut self.menu);
        frame.render_widget(&input_block, layer2[1]);
        self.render_line_state(frame, &input_block, &layer2);
    }

    fn render_line_state(&mut self, frame: &mut Frame, input_block: &Block, layer2: &Rc<[Rect]>) {
        let input_area = Layout::new(
            Direction::Vertical,
            [Constraint::Max(1), Constraint::Max(1), Constraint::Min(1)],
        )
        .split(input_block.inner(layer2[1]));

        match &self.menu_state {
            Menu::Cotoba => {
                frame.render_widget(&self.word_area, input_area[0]);
            }
            Menu::Yomikata => {
                frame.render_widget(&self.word_area, input_area[1]);
            }

            Menu::Teigi => {
                frame.render_widget(&self.word_area, input_area[2]);
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

    pub fn handle_events(&mut self) -> Result<(), Box<dyn Error>> {
        if poll(Duration::from_millis(100))? {
            match event::read() {
                Ok(Event::Key(key_code)) => {
                    if key_code.kind == KeyEventKind::Press {
                        match self.menu_state {
                            /*
                            handle events for the add word menu
                             */
                            Menu::Menu => match key_code.code {
                                KeyCode::Char('q') => self.menu_state = Menu::Exit,
                                KeyCode::Down | KeyCode::Tab => {
                                    self.menu.select_next();
                                }

                                KeyCode::Up => {
                                    self.menu.select_previous();
                                }

                                KeyCode::Enter => {
                                    self.clear_highlight();
                                    self.word_area.set_cursor_line_style(Style::new());

                                    match self.menu.selected().unwrap() {
                                        0 => {
                                            self.menu_state = Menu::Cotoba;
                                        }

                                        1 => {
                                            self.menu_state = Menu::Yomikata;
                                        }

                                        2 => {
                                            self.menu_state = Menu::Teigi;
                                        }

                                        _ => {}
                                    }
                                }

                                _ => {}
                            },

                            Menu::Cotoba | Menu::Yomikata | Menu::Teigi => match key_code.code {
                                KeyCode::Char('q') => {
                                    self.menu_state = Menu::Menu;
                                    self.show_highlight();
                                }

                                KeyCode::Char(ch) => {
                                    self.word_area.insert_char(ch);
                                }

                                KeyCode::Backspace => {
                                    self.word_area.delete_char();
                                }

                                KeyCode::Left => {
                                    self.word_area.move_cursor(tui_textarea::CursorMove::Back);
                                }

                                KeyCode::Right => {
                                    self.word_area
                                        .move_cursor(tui_textarea::CursorMove::Forward);
                                }
                                _ => {}
                            },

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
