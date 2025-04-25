use std::{error::Error, io::stdout, time::Duration};

use ratatui::{
    DefaultTerminal, Frame, Terminal, backend,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, poll},
};
use ui::{
    add_word::{self, AddWord},
    main_layout::MainLayout,
    main_selection::MainSelection,
};

mod cotoba;
mod tests;
mod ui;

const MAIN_MENU: [&str; 4] = [
    "Add Word（言葉を追加する）",
    "Edit Word（変更）",
    "Word Search（検索）",
    "Back（戻る）",
];

#[derive(Clone, Copy)]
enum Menu {
    Menu,
    AddWord,
    Exit,
}

pub struct DictionaryApp<'a> {
    terminal: DefaultTerminal,
    menu_state: Menu,
    main_selection: MainSelection<'a>,
    add_word: AddWord<'a>,
}

impl<'a> DictionaryApp<'a> {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let terminal = Terminal::new(backend::CrosstermBackend::new(stdout()))?;
        Ok(Self {
            terminal,
            menu_state: Menu::Menu,
            main_selection: MainSelection::default(),
            add_word: AddWord::new(),
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.terminal.clear()?;
        let mut main_layout = MainLayout::new();
        self.main_selection = MainSelection::new(MAIN_MENU);

        loop {
            match self.menu_state {
                Menu::Menu | Menu::AddWord => {
                    let main_selection = &mut self.main_selection;
                    let main_layout = &mut main_layout;
                    self.terminal.draw(|frame| {
                        render_main(
                            frame,
                            main_layout,
                            main_selection,
                            &self.menu_state,
                            &mut self.add_word,
                        )
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

fn render_main(
    frame: &mut Frame,
    main_layout: &mut MainLayout,
    main_selection: &mut MainSelection,
    menu_state: &Menu,
    add_word: &mut AddWord,
) {
    // Render the main layout of the terminal interface, including the left and right sections of the UI.

    main_layout.render(frame);
    //left menu selection

    main_selection.render(frame, main_layout.left.inner(main_layout.area[0]));

    match menu_state {
        Menu::AddWord => {
            add_word.render(main_layout, frame);
        }
        _ => {}
    }
}

/*
Handle all events for selections, states, exites
*/

fn handle_events(dictionary: &mut DictionaryApp) -> Result<(), Box<dyn Error>> {
    if poll(Duration::from_millis(100))? {
        match dictionary.menu_state {
            //handle events while the current state is Main Menu
            Menu::Menu => {
                match event::read() {
                    Ok(Event::Key(key_event)) => {
                        if key_event.kind == KeyEventKind::Press {
                            match key_event.code {
                                KeyCode::Down => {
                                    dictionary.main_selection.get_state().select_next();
                                }

                                KeyCode::Up => {
                                    dictionary.main_selection.get_state().select_previous();
                                }

                                KeyCode::Enter => {
                                    match dictionary.main_selection.get_state().selected().unwrap()
                                    {
                                        0 => {
                                            //Blank the selector and change the state
                                            dictionary.main_selection.clear_highlight();

                                            dictionary.menu_state = Menu::AddWord;
                                            dictionary.add_word.set_menu(add_word::Menu::Menu);
                                        }

                                        3 => dictionary.menu_state = Menu::Exit,

                                        _ => {}
                                    }
                                }

                                _ => (),
                            }
                        }
                    }

                    _ => (),
                }
            }
            Menu::AddWord => match dictionary.add_word.get_menu() {
                add_word::Menu::Exit => {
                    dictionary.main_selection.show_highlight();
                    dictionary.menu_state = Menu::Menu
                }
                _ => {
                    dictionary.add_word.handle_events()?;
                }
            },

            _ => {}
        }
    }
    Ok(())
}
