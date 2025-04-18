use std::{
    error::Error,
    io::{self, stdout},
    sync::Arc,
    thread::{self, Thread},
    time::{Duration, Instant},
};

use ratatui::{
    DefaultTerminal, Frame, Terminal, backend,
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind, poll},
        execute,
        terminal::LeaveAlternateScreen,
    },
    layout::{Constraint, Layout},
    prelude::CrosstermBackend,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph, RatatuiLogo},
};

mod tests;

enum Menu {
    Menu,
    Exit,
}

pub struct Dictionary {
    terminal: DefaultTerminal,
    menu_state: Menu,
    text: [String; 3],
}

impl Dictionary {
    pub fn new() -> Self {
        let terminal = Terminal::new(backend::CrosstermBackend::new(stdout())).unwrap();
        let text = [
            "asdfzfxdasd".to_string(),
            "123213asdasd".to_string(),
            "asdzcaadl123123".to_string(),
        ];

        Self {
            terminal,
            menu_state: Menu::Menu,
            text,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let interval = Duration::from_secs(1);

        let mut last_time = Instant::now();
        self.terminal.clear()?;
        loop {
            match self.menu_state {
                Menu::Menu => {
                    self.terminal.draw(|frame| {
                        render_callback(frame, &self.text, interval, &mut last_time);
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
    text: &[String; 3],
    interval: Duration,
    last_time: &mut Instant,
) {
    let main_block = Block::bordered().style(Style::new().fg(Color::Green));

    let base_area = Layout::new(
        ratatui::layout::Direction::Horizontal,
        [Constraint::Fill(50), Constraint::Fill(50)],
    )
    .split(main_block.inner(frame.area()));

    let logo = RatatuiLogo::small();

    // text.into_iter().for_each(|char| {
    //     thread::sleep(Duration::from_millis(10));
    //     print!("{char}");
    //     std::io::stdout().flush().unwrap();
    // });

    let current_time = Instant::now();

    let elapsed = current_time.duration_since(*last_time);

    let mut para = Paragraph::new(text[0].clone()).block(Block::bordered());

    if elapsed >= interval {
        para = Paragraph::new(text[1].clone()).block(Block::bordered());
        frame.render_widget(&para, base_area[1]);

        *last_time = current_time;
    }

    frame.render_widget(&main_block, base_area[0]);
    frame.render_widget(&para, base_area[1]);
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
