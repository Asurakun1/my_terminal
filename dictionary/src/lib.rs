use std::{error::Error, io::stdout, time::Duration};

use cotoba::Cotoba;
use ratatui::{
    DefaultTerminal, Frame, Terminal, backend,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, poll},
    layout::{Constraint, Direction, Layout},
    widgets::List,
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
    list: Vec<Cotoba>,
}

impl<'a> DictionaryApp<'a> {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let terminal = Terminal::new(backend::CrosstermBackend::new(stdout()))?;
        Ok(Self {
            terminal,
            menu_state: Menu::Menu,
            main_selection: MainSelection::default(),
            add_word: AddWord::new(),
            list: vec![],
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
                            &mut self.list,
                        )
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

fn render_main(
    frame: &mut Frame,
    main_layout: &mut MainLayout,
    main_selection: &mut MainSelection,
    menu_state: &Menu,
    add_word: &mut AddWord,
    list: &mut Vec<Cotoba>,
) {
    // Render the main layout of the terminal interface, including the left and right sections of the UI.

    main_layout.render(frame);
    //left menu selection

    main_selection.render(frame, main_layout.left.inner(main_layout.area[0]));

    /*
    Right window is split into two parts vertically
     */
    let mut right_window = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(0), Constraint::Fill(10)])
        .split(main_layout.right.inner(main_layout.area[1]));

    match menu_state {
        Menu::AddWord => {
            //Right window draws the top of the stack only when called
            right_window = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(30), Constraint::Fill(10)])
                .split(main_layout.right.inner(main_layout.area[1]));
            add_word.render(right_window.clone(), frame);
        }
        _ => {}
    }

    let items = list
        .iter()
        .map(|cotoba| cotoba.get_word())
        .collect::<Vec<&str>>();

    let list = List::new(items);

    frame.render_widget(list, right_window[1]);
}

/*
Handle all events for selections, states, exites
*/

fn handle_events<'a>(dictionary: &'a mut DictionaryApp) -> Result<(), Box<dyn Error>> {
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
                                            dictionary.terminal.clear()?;
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

            /*
            handle events inside the AddWord
             */
            Menu::AddWord => match dictionary.add_word.get_menu() {
                add_word::Menu::Exit => {
                    dictionary.main_selection.show_highlight();
                    dictionary.menu_state = Menu::Menu;
                }

                _ => {
                    dictionary.add_word.handle_events()?;

                    /*
                    list needs to push a new word once when a new cotoba has been created
                    this needs to be fixed
                     */
                    if !dictionary.add_word.cotoba().get_word().is_empty() {
                        dictionary.list.push(dictionary.add_word.cotoba().clone());
                        dictionary.add_word.init();
                    }
                }
            },

            _ => {}
        }
    }
    Ok(())
}
