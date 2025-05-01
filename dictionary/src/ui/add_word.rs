use std::error::Error;
use std::rc::Rc;
use std::time::Duration;

use ratatui::crossterm::event::{KeyEvent, poll};
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

use crate::cotoba::Cotoba;

const ADD_WORD_MENU: [&str; 3] = ["言葉：　", "読み方：　", "定義：　"];

pub enum Menu {
    Menu,
    Cotoba,
    Yomikata,
    Teigi,
    Exit,
}

pub struct AddWord<'a> {
    menu: ListState,
    list: List<'a>,
    menu_state: Menu,
    input: [String; 3],
    word_area: TextArea<'a>,
    cotoba: Cotoba,
}

impl<'a> AddWord<'a> {
    pub fn new() -> Self {
        Self {
            menu: ListState::default().with_selected(Some(0)),
            list: List::new(ADD_WORD_MENU)
                .block(Block::bordered().title("言葉追加"))
                .highlight_style(Style::new().reversed()),
            menu_state: Menu::Menu,
            input: [String::new(), String::new(), String::new()],
            word_area: TextArea::default(),
            cotoba: Cotoba::default(),
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

    pub fn cotoba(&self) -> &Cotoba {
        &self.cotoba
    }

    /*
    Render call section
     */

    pub fn render(&mut self, main_layout: Rc<[Rect]>, frame: &mut Frame) {
        /*
        splits the layout into 2 parts
         */
        let layer2 = Layout::new(
            Direction::Horizontal,
            [Constraint::Max(25), Constraint::Percentage(60)],
        )
        .split(main_layout[0]);

        let input_block = Block::bordered().title("入力");

        frame.render_stateful_widget(&self.list, layer2[0], &mut self.menu);
        frame.render_widget(&input_block, layer2[1]);
        self.render_line_state(frame, &input_block, &layer2);
    }

    fn render_line_state(&mut self, frame: &mut Frame, input_block: &Block, layer2: &Rc<[Rect]>) {
        let input_area = Layout::new(Direction::Vertical, [Constraint::Max(1)])
            .split(input_block.inner(layer2[1]));

        match &self.menu_state {
            Menu::Cotoba | Menu::Yomikata | Menu::Teigi => {
                frame.render_widget(&self.word_area, input_area[0]);
            }

            _ => {}
        }
    }

    /*
    Event Handling Section
     */

    fn handle_inputs(&mut self, key_code: KeyEvent) {
        match key_code.code {
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

            KeyCode::Enter => {
                match self.menu_state {
                    Menu::Cotoba => {
                        self.cotoba.set_word(&self.word_area.lines()[0]);
                    }

                    Menu::Yomikata => {
                        self.cotoba.set_reading(&self.word_area.lines()[0]);
                    }

                    _ => {}
                }

                self.word_area = TextArea::from(vec![""]);
            }
            _ => {}
        }
    }

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

                            Menu::Cotoba | Menu::Yomikata | Menu::Teigi => {
                                self.handle_inputs(key_code);
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
