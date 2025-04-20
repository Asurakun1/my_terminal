use std::{error::Error, io::stdout, time::Duration, vec};

use main_layout::MainLayout;
use ratatui::{
    DefaultTerminal, Frame, Terminal, backend,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, poll},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders, List, ListState, Paragraph, Wrap},
};

mod cotoba;
mod main_layout;
mod tests;

enum Menu {
    Menu,
    Exit,
}

pub struct Dictionary {
    terminal: DefaultTerminal,
    menu_state: Menu,
    list_state: ListState,
    items: Vec<String>,
}

impl Dictionary {
    pub fn new() -> Self {
        let terminal = Terminal::new(backend::CrosstermBackend::new(stdout())).unwrap();
        Self {
            terminal,
            menu_state: Menu::Menu,
            list_state: ListState::default(),
            items: vec![
                "Add Word（言葉を追加する）".to_string(),
                "Edit Word（変更）".to_string(),
                "Word Search（検索）".to_string(),
            ],
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.terminal.clear()?;
        self.list_state.select_first();
        let mut main_layout = MainLayout::new();

        loop {
            match self.menu_state {
                Menu::Menu => {
                    self.terminal.draw(|frame| {
                        render_callback(
                            frame,
                            self.items.clone(),
                            &mut self.list_state,
                            &mut main_layout,
                        );
                    })?;

                    handle_events(self)?;
                }
                Menu::Exit => {
                    self.terminal.clear()?;
                    break;
                }
            }
        }
        Ok(())
    }
}

fn render_callback(
    frame: &mut Frame,
    items: Vec<String>,
    state: &mut ListState,
    main_layout: &mut MainLayout,
) {
    //Left and right main window
    main_layout.render(frame);
    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Max(3)])
        .split(main_layout.left.inner(main_layout.area[0]));

    let list = List::new(items)
        .block(Block::new().borders(Borders::BOTTOM))
        .highlight_style(Style::new().reversed());

    let p1_placeholder = Paragraph::new(
        "すみません今からは何もい動いていないです。\n「Ｑ」のボッタンを押すならプログラムを終了します。",
    ).wrap(Wrap { trim: true });

    frame.render_stateful_widget(list, left[0], state);
    frame.render_widget(p1_placeholder, left[1]);
}

fn handle_events(dictionary: &mut Dictionary) -> Result<(), Box<dyn Error>> {
    if poll(Duration::from_millis(100))? {
        match event::read() {
            Ok(Event::Key(key_event)) => {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char('q') => {
                            panic!("This is a temperory force close");
                            //dictionary.menu_state = Menu::Exit;
                        }

                        KeyCode::Down => {
                            dictionary.list_state.select_next();
                        }

                        KeyCode::Up => {
                            dictionary.list_state.select_previous();
                        }
                        _ => (),
                    }
                }
            }

            _ => (),
        }
    }
    Ok(())
}
