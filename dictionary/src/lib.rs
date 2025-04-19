use std::{error::Error, io::stdout, time::Duration};

use ratatui::{
    DefaultTerminal, Frame, Terminal, backend,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, poll},
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::Block,
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
    pub fn new() -> Self {
        let terminal = Terminal::new(backend::CrosstermBackend::new(stdout())).unwrap();
        Self {
            terminal,
            menu_state: Menu::Menu,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.terminal.clear()?;

        loop {
            match self.menu_state {
                Menu::Menu => {
                    self.terminal.draw(|frame| {
                        render_callback(frame);
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

fn render_callback(frame: &mut Frame) {
    let main_block = Block::bordered().style(Style::new().fg(Color::Green));

    let base_area = Layout::new(
        ratatui::layout::Direction::Horizontal,
        [Constraint::Fill(50), Constraint::Fill(50)],
    )
    .split(main_block.inner(frame.area()));

    let place_holder = Block::bordered();

    frame.render_widget(&main_block, base_area[0]);
    frame.render_widget(&place_holder, base_area[1]);
}

fn handle_events(dictionary: &mut Dictionary) -> Result<(), Box<dyn Error>> {
    if poll(Duration::from_millis(100))? {
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
    }
    Ok(())
}
