use std::error::Error;
use std::rc::Rc;

use ratatui::crossterm::event::{KeyEvent, KeyModifiers};
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::widgets::{List, Paragraph, Wrap};
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, ListState},
};
use tui_textarea::{CursorMove, Key, TextArea};

use crate::cotoba::Cotoba;

const ADD_WORD_MENU: [&str; 4] = ["言葉：　", "読み方：　", "定義：　", "保存"];

pub enum Menu {
    Menu,
    Cotoba,
    Yomikata,
    Teigi,
    Save,
    Exit,
}

pub struct AddWord<'a> {
    menu: ListState,
    list: List<'a>,
    menu_state: Menu,
    word_area: TextArea<'a>,
    definition: TextArea<'a>,
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
            word_area: TextArea::default(),
            definition: TextArea::default(),
            cotoba: Cotoba::new(),
        }
    }

    pub fn init(&mut self) {
        self.cotoba = Cotoba::new();
        self.reset();
    }

    fn reset(&mut self) {
        self.show_highlight();
        self.menu.select_first();
        self.menu_state = Menu::Menu;
        self.word_area = TextArea::default();
        self.definition = TextArea::default();
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
        let vertical_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(20), Constraint::Percentage(100)],
        )
        .split(main_layout[0]);

        /*
        splits the layout into 2 parts
         */
        self.render_upper_layer(frame, vertical_layout.clone());
        self.render_lower_layer(frame, vertical_layout);
    }

    fn render_upper_layer(&mut self, frame: &mut Frame, vertical_layout: Rc<[Rect]>) {
        /*
        Render the upper layer
         */

        let upper_layer = Layout::new(
            Direction::Horizontal,
            [Constraint::Max(25), Constraint::Fill(10)],
        )
        .split(vertical_layout[0]);

        let input_block = Block::bordered().title("入力");

        frame.render_stateful_widget(&self.list, upper_layer[0], &mut self.menu);
        frame.render_widget(&input_block, upper_layer[1]);

        self.render_line_state(frame, &input_block, &upper_layer);
    }

    fn render_lower_layer(&mut self, frame: &mut Frame, vertical_layout: Rc<[Rect]>) {
        /*
        Render the Lower Layer
         */
        let lower_layer = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(20), Constraint::Percentage(90)],
        )
        .split(vertical_layout[1]);

        let output_block = Block::bordered().title("言葉データ");

        let def_layer = Layout::new(Direction::Horizontal, [Constraint::Percentage(100)])
            .split(output_block.inner(lower_layer[1]));

        let cotoba_list = List::new([
            format!("言葉　：{}", self.cotoba().get_word()),
            format!("読み方：{}", self.cotoba().get_reading().join("、"),),
        ]);

        let definition = Block::new().title("定義：");

        match self.menu_state {
            Menu::Teigi => {
                self.definition.set_cursor_style(Style::new().reversed());
            }
            _ => {
                self.definition.set_cursor_style(Style::new());
            }
        }

        frame.render_widget(&output_block, vertical_layout[1]);

        frame.render_widget(cotoba_list, output_block.inner(lower_layer[0]));
        frame.render_widget(&definition, output_block.inner(lower_layer[1]));
        frame.render_widget(&self.definition, definition.inner(def_layer[0]));
    }

    fn render_line_state(
        &mut self,
        frame: &mut Frame,
        input_block: &Block,
        upper_layer: &Rc<[Rect]>,
    ) {
        match &self.menu_state {
            Menu::Cotoba | Menu::Yomikata => {
                frame.render_widget(&self.word_area, input_block.inner(upper_layer[1]));
            }

            Menu::Teigi => {
                let new_block = input_block.clone().dim();
                frame.render_widget(&new_block, upper_layer[1]);
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

            KeyCode::Up => {
                self.word_area.move_cursor(tui_textarea::CursorMove::Up);
            }
            KeyCode::Down => {
                self.word_area.move_cursor(tui_textarea::CursorMove::Down);
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

                self.word_area = TextArea::default();
                self.word_area.set_cursor_line_style(Style::new());
            }
            _ => {}
        }
    }

    pub fn handle_events(&mut self) -> Result<(), Box<dyn Error>> {
        match event::read() {
            Ok(Event::Key(key_code)) => {
                if key_code.kind == KeyEventKind::Press {
                    match self.menu_state {
                        /*
                        Menu state selection
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

                                /*
                                selects the given menu states
                                 */
                                match self.menu.selected().unwrap() {
                                    0 => {
                                        self.menu_state = Menu::Cotoba;
                                        self.word_area =
                                            TextArea::from(vec![self.cotoba().get_word()]);
                                    }

                                    1 => {
                                        self.menu_state = Menu::Yomikata;
                                        self.word_area =
                                            TextArea::from(self.cotoba().get_reading());
                                    }

                                    2 => {
                                        self.menu_state = Menu::Teigi;
                                        self.word_area.set_cursor_style(Style::new());
                                        self.definition =
                                            TextArea::from(self.cotoba().get_definition());
                                        self.definition.move_cursor(CursorMove::End);
                                    }

                                    3 => {
                                        self.menu_state = Menu::Save;
                                    }

                                    _ => {}
                                }
                            }

                            _ => {}
                        },

                        Menu::Cotoba | Menu::Yomikata => {
                            self.handle_inputs(key_code);
                        }

                        Menu::Teigi => {
                            if key_code.code == KeyCode::Char('z')
                                && key_code.modifiers.contains(KeyModifiers::CONTROL)
                            {
                                self.definition.undo();
                            }

                            if key_code.code == KeyCode::Char('s')
                                && key_code.modifiers.contains(KeyModifiers::CONTROL)
                            {
                                self.cotoba.set_definition(self.definition.lines());
                                self.menu_state = Menu::Menu;
                                self.show_highlight();
                            } else {
                                self.definition.input(key_code);
                            }
                        }

                        _ => {}
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}
