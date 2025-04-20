use dictionary::Dictionary;
use std::rc::Rc;
use std::{error::Error, time::Duration};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, poll},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Styled},
    widgets::{Block, Paragraph, RatatuiLogo, Wrap},
};

#[derive(PartialEq)]
enum Menu {
    Menu,
    Dictionary,
    Exit,
}

pub struct App {
    app_state: Menu,
    terminal: DefaultTerminal,
    cycle: u8,
}

impl App {
    pub fn new(terminal: DefaultTerminal) -> Self {
        Self {
            cycle: 0,
            terminal,
            //Temperory state change
            app_state: Menu::Menu,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            match self.app_state {
                Menu::Menu => {
                    self.terminal.draw(|frame| {
                        render_callback(frame, self.cycle);
                    })?;
                    handle_events(self)?;
                }
                // Enter Dictionary Module
                Menu::Dictionary => {
                    {
                        let mut dictionary = Dictionary::new();
                        dictionary.run()?;
                    }

                    //re enter the alternate screen
                    self.app_state = Menu::Menu;
                    self.terminal.clear()?;
                }
                Menu::Exit => {
                    break;
                }
            }
        }

        Ok(())
    }
}

//Draws the main screen
fn render_callback(frame: &mut Frame, cycle: u8) {
    //outer layer main screen block
    let layer_0 = chunks(frame.area());
    let (menu_block, divider) = main_screen_block(Rc::clone(&layer_0));
    frame.render_widget(&menu_block, layer_0[0]);

    //list of modules (implemented or not)
    menu_selection(frame, &menu_block, &divider, &cycle);
    menu_right_chunk(frame, Rc::clone(&divider), &menu_block);
}

fn main_screen_block<'a>(layer_0: Rc<[Rect]>) -> (Block<'a>, Rc<[Rect]>) {
    let menu_block = Block::bordered()
        .style(Style::new().fg(Color::Green))
        .title("Main Menu");

    let divider = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(50), Constraint::Fill(50)])
        .split(layer_0[0]);
    (menu_block, divider)
}

fn menu_right_chunk(frame: &mut Frame, divider: Rc<[Rect]>, menu: &Block) {
    let inner_right_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(10),
            Constraint::Fill(40),
            Constraint::Fill(10),
        ])
        .split(menu.inner(divider[1]));

    let ctrl =
        Paragraph::new("Commands: \u{2191} / \u{2193} / Enter（選択）").block(Block::bordered());
    let logo = RatatuiLogo::default().size(ratatui::widgets::RatatuiLogoSize::Small);
    let block = Paragraph::new("Ver. 0.0.1");

    frame.render_widget(ctrl, inner_right_chunk[0]);
    frame.render_widget(logo, inner_right_chunk[1]);
    frame.render_widget(block, inner_right_chunk[2]);
}

fn menu_selection(frame: &mut Frame, menu: &Block, divider: &Rc<[Rect]>, cycle: &u8) {
    let inner_left_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Max(3),
            Constraint::Max(3),
            Constraint::Max(3),
            Constraint::Max(3),
        ])
        .split(menu.inner(divider[0]));

    let mut item_1 = Paragraph::new("Dictionary（辞書）")
        .block(Block::bordered())
        .wrap(Wrap { trim: false });
    let mut item_2 = Paragraph::new("すべてのリストが選択可能です。")
        .block(Block::bordered())
        .wrap(Wrap { trim: true });
    let mut item_3 = Paragraph::new("でも、選択だとしても。今からは何も動いていない。")
        .block(Block::bordered())
        .wrap(Wrap { trim: true });
    let mut item_4 = Paragraph::new("Exit（終了）")
        .block(Block::bordered())
        .wrap(Wrap { trim: true });

    match cycle {
        0 => {
            item_1 = item_1.set_style(Style::new().bg(Color::Green).fg(Color::Black));
        }
        1 => {
            item_2 = item_2.set_style(Style::new().bg(Color::Green).fg(Color::Black));
        }
        2 => item_3 = item_3.set_style(Style::new().bg(Color::Green).fg(Color::Black)),
        3 => item_4 = item_4.set_style(Style::new().bg(Color::Green).fg(Color::Black)),
        _ => {}
    }

    frame.render_widget(item_1, inner_left_chunk[0]);
    frame.render_widget(item_2, inner_left_chunk[1]);
    frame.render_widget(item_3, inner_left_chunk[2]);
    frame.render_widget(item_4, inner_left_chunk[3]);
}

fn chunks(chunk: Rect) -> Rc<[Rect]> {
    Layout::default()
        .constraints([Constraint::Fill(10)])
        .split(chunk)
}

fn handle_events(app: &mut App) -> Result<(), Box<dyn Error>> {
    if poll(Duration::from_millis(100))? {
        match event::read() {
            Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Down => {
                    if app.cycle < 3 {
                        app.cycle += 1;
                    } else {
                        app.cycle = 3;
                    }
                }
                KeyCode::Up => {
                    if app.cycle > 0 {
                        app.cycle -= 1;
                    } else {
                        app.cycle = 0;
                    }
                }
                KeyCode::Enter => match app.cycle {
                    0 => app.app_state = Menu::Dictionary,
                    3 => app.app_state = Menu::Exit,
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
    }
    Ok(())
}
