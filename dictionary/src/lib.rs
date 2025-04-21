use std::{error::Error, io::stdout, time::Duration, vec};

use ratatui::{
    DefaultTerminal, Frame, Terminal, backend,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, poll},
};
use ui::{main_layout::MainLayout, main_selection::MainSelection};

mod cotoba;
mod tests;
mod ui;

enum Menu {
    Menu,
    AddWord,
    Exit,
}

pub struct DictionaryApp<'a> {
    terminal: DefaultTerminal,
    menu_state: Menu,
    main_selection: MainSelection<'a>,
    items: Vec<String>,
}

impl<'a> DictionaryApp<'a> {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let terminal = Terminal::new(backend::CrosstermBackend::new(stdout()))?;
        Ok(Self {
            terminal,
            menu_state: Menu::Menu,
            main_selection: MainSelection::default(),
            items: vec![
                "Add Word（言葉を追加する）".to_string(),
                "Edit Word（変更）".to_string(),
                "Word Search（検索）".to_string(),
            ],
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.terminal.clear()?;
        let mut main_layout = MainLayout::new();
        self.main_selection = MainSelection::new(self.items.clone());
        loop {
            match self.menu_state {
                Menu::Menu => {
                    let main_selection = &mut self.main_selection;
                    let main_layout = &mut main_layout;
                    self.terminal.draw(|frame| {
                        render_main(frame, main_layout, main_selection);
                    })?;

                    handle_events(self)?;
                }

                Menu::AddWord => {
                    self.terminal.draw(|frame| {
                        let main_selection = &mut self.main_selection;
                        let main_layout = &mut main_layout;
                        render_main(frame, main_layout, main_selection);
                    })?;
                    handle_events(self)?;
                }
                Menu::Exit => {
                    self.terminal.clear()?;
                    break;
                }

                _ => {}
            }
        }
        Ok(())
    }
}

fn render_add_word(frame: &mut Frame) {}

fn render_main(
    frame: &mut Frame,
    main_layout: &mut MainLayout,
    main_selection: &mut MainSelection,
) {
    // Render the main layout of the terminal interface, including the left and right sections of the UI.
    main_layout.render(frame);
    //left menu selection
    main_selection.render(frame, main_layout.left.inner(main_layout.area[0]));
}

fn handle_events(dictionary: &mut DictionaryApp) -> Result<(), Box<dyn Error>> {
    if poll(Duration::from_millis(100))? {
        match event::read() {
            Ok(Event::Key(key_event)) => {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char('q') => {
                            dictionary.menu_state = Menu::Exit;
                            panic!("This is a temperory force close");
                        }

                        KeyCode::Down => {
                            dictionary.main_selection.state.select_next();
                        }

                        KeyCode::Up => {
                            dictionary.main_selection.state.select_previous();
                        }

                        KeyCode::Enter => match dictionary.main_selection.state.selected().unwrap()
                        {
                            0 => {
                                dictionary.menu_state = Menu::AddWord;
                            }

                            _ => {}
                        },

                        _ => (),
                    }
                }
            }

            _ => (),
        }
    }
    Ok(())
}
