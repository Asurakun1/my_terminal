use std::error::Error;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    widgets::RatatuiLogo,
};

mod tests;

enum Menu {
    Menu,
    Exit,
}

pub struct Dictionary {
    terminal: DefaultTerminal,
    menu_state: Menu,
}

impl Dictionary {
    pub fn new(terminal: DefaultTerminal) -> Self {
        Self {
            terminal,
            menu_state: Menu::Menu,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            match self.menu_state {
                Menu::Menu => {
                    self.terminal.draw(render_callback)?;
                    handle_events(self)?;
                }
                Menu::Exit => {
                    break;
                }
            }
        }
        Ok(())
    }
}

fn render_callback(frame: &mut Frame) {
    let logo = RatatuiLogo::small();

    frame.render_widget(logo, frame.area());
}

fn handle_events(dictionary: &mut Dictionary) -> Result<(), Box<dyn Error>> {
    match event::read() {
        Ok(Event::Key(key_event)) => {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Char('q') => {
                        dictionary.menu_state = Menu::Exit;
                    }
                    _ => (),
                }
            }
        }

        _ => (),
    }
    Ok(())
}
