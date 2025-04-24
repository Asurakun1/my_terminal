use dictionary::DictionaryApp;
use ratatui::style::Stylize;
use std::rc::Rc;
use std::{error::Error, time::Duration};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, poll},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Styled},
    widgets::{Block, Paragraph, RatatuiLogo},
};

const MENU_SELECT: [&str; 4] = [
    "Dictionary（辞書）",
    "すべてのリストが選択可能です。",
    "でも、選択だとしても。今からは何も動いていない。",
    "Exit（終了）",
];

#[derive(PartialEq)]
enum Menu {
    Menu,
    Dictionary,
    Exit,
}

pub struct App<'a> {
    app_state: Menu,
    terminal: DefaultTerminal,
    cycle: u8,
    menu_select: Box<[Paragraph<'a>]>,
}

impl<'a> App<'a> {
    pub fn new(terminal: DefaultTerminal) -> Self {
        let list: Box<[Paragraph]> = MENU_SELECT
            .iter()
            .map(|item| Paragraph::new(*item).block(Block::bordered()))
            .collect();

        Self {
            cycle: 0,
            terminal,
            menu_select: list,
            //Temperory state change
            app_state: Menu::Menu,
        }
    }

    /*
    Runs the state machine
     */

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            match self.app_state {
                Menu::Menu => {
                    self.terminal.draw(|frame| {
                        render_callback(frame, self.cycle, &mut self.menu_select);
                    })?;
                    handle_events(self)?;
                }
                // Enter Dictionary Module
                Menu::Dictionary => {
                    let mut dictionary = DictionaryApp::new()?;
                    dictionary.run()?;

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
fn render_callback(frame: &mut Frame, cycle: u8, items: &mut Box<[Paragraph]>) {
    //outer layer main screen block
    let layer_0 = chunks(frame.area());
    let (menu_block, divider) = main_screen_block(Rc::clone(&layer_0));
    frame.render_widget(&menu_block, layer_0[0]);

    //list of modules (implemented or not)
    menu_selection(frame, &menu_block, &divider, &cycle, items);
    menu_right_chunk(frame, Rc::clone(&divider), &menu_block);
}

/*
Split the first layer into 2 halves
*/
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

/*
Right main menu
Icons are just for display
*/

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

/*
Menu selection function
Item 1
Item 2
Item 3
Item 4
*/

fn menu_selection(
    frame: &mut Frame,
    menu: &Block,
    divider: &Rc<[Rect]>,
    cycle: &u8,
    items: &mut Box<[Paragraph]>,
) {
    let inner_left_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Max(3),
            Constraint::Max(3),
            Constraint::Max(3),
            Constraint::Max(3),
        ])
        .split(menu.inner(divider[0]));

    //iterate the menu items and render. Highlight only the selected, the rest will not be highlighted.
    (0..items.iter().count()).into_iter().for_each(|index| {
        match *cycle == index as u8 {
            true => {
                items[index] = items[index].to_owned().set_style(Style::new().reversed());
            }
            false => {
                items[index] = items[index].to_owned().set_style(Style::new());
            }
        }
        frame.render_widget(&items[index], inner_left_chunk[index]);
    });
}

fn chunks(chunk: Rect) -> Rc<[Rect]> {
    Layout::default()
        .constraints([Constraint::Fill(10)])
        .split(chunk)
}

/*
handle all events
input
states
data flow
*/

fn handle_events(app: &mut App) -> Result<(), Box<dyn Error>> {
    if poll(Duration::from_millis(100))? {
        match event::read() {
            Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Down => {
                    if app.cycle < 3 {
                        app.cycle += 1;
                    } else {
                        app.cycle = 0;
                    }
                }
                KeyCode::Up => {
                    if app.cycle > 0 {
                        app.cycle -= 1;
                    } else {
                        app.cycle = 3;
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
