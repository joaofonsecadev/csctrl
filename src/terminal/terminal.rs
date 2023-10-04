use std::io::{stdout, Stdout};
use crossterm::ExecutableCommand;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::{Backend, CrosstermBackend};
use crate::csctrl::types::CsServerConfig;
use std::cell::OnceCell;
use std::ops::Deref;
use std::time::Duration;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Alignment, Direction};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph};

struct TerminalUiState {
    selected_server: CsServerConfig,
    input_box: String,
}

pub struct Terminal {
    terminal_ui: OnceCell<ratatui::Terminal<CrosstermBackend<Stdout>>>,
    is_terminal_active: bool,
    terminal_ui_state: TerminalUiState,
}

impl Terminal {
    pub fn terminal() -> Terminal {
        Terminal {
            terminal_ui_state: TerminalUiState {
                selected_server: CsServerConfig {
                    name: "".to_string(),
                    address: "".to_string(),
                    rcon_address: "".to_string(),
                    rcon_password: "".to_string(),
                    csctrl_token: "".to_string(),
                },
                input_box: "".to_string(),
            },
            terminal_ui: OnceCell::new(),
            is_terminal_active: false,
        }
    }
    
    pub fn init(&mut self) {
        enable_raw_mode().unwrap();
        crossterm::execute!(stdout(), EnterAlternateScreen).unwrap();

        self.terminal_ui.get_or_init(|| {
            return ratatui::Terminal::new(CrosstermBackend::new(stdout())).unwrap();
        });

        self.is_terminal_active = true;
    }
    
    pub fn tick(&mut self) {
        self.handle_events();

        self.terminal_ui.get_mut().unwrap().draw(|frame| {
            ui(&mut self.terminal_ui_state, frame);
        }).unwrap();
    }

    fn handle_events(&mut self) {
        if !crossterm::event::poll(Duration::from_millis(50)).unwrap() { return; }

        if let Event::Key(key) = crossterm::event::read().unwrap() {
            match key.code {
                KeyCode::Char(value) => {
                    if !validate_input_char(&value) { return; }
                    if key.kind != KeyEventKind::Press { return; }
                    &self.terminal_ui_state.input_box.push(value);
                }
                KeyCode::Backspace => {
                    if key.kind != KeyEventKind::Press { return; }
                    &self.terminal_ui_state.input_box.pop();
                }
                KeyCode::Enter => {
                    if key.kind != KeyEventKind::Press { return; }
                    let input = self.terminal_ui_state.input_box.to_string();
                    if input.is_empty() { return; }
                    self.terminal_ui_state.input_box.clear();
                    crate::csctrl::csctrl::command_messenger().write().unwrap().push(input);
                }
                KeyCode::Esc => { self.close_terminal(); }
                _ => {}
            }
        }
    }

    fn close_terminal(&mut self) {
        disable_raw_mode().unwrap();
        crossterm::execute!(self.terminal_ui.get_mut().unwrap().backend_mut(), LeaveAlternateScreen);
        self.terminal_ui.get_mut().unwrap().show_cursor().unwrap();
        self.is_terminal_active = false;
    }

    pub fn is_terminal_active(&self) -> &bool {
        return &self.is_terminal_active;
    }
    
    pub fn shutdown(&self) {

    }
}

fn validate_input_char(char: &char) -> bool {
    let char_code = *char as u32;
    if char_code > 127 { return false; }
    return true;
}

fn ui(state: &mut TerminalUiState, frame: &mut Frame<CrosstermBackend<Stdout>>) {
    let terminal_height = frame.size().height;

    let layout_main = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(terminal_height - 1),
            Constraint::Length(1),
        ])
        .split(frame.size());

    frame.render_widget(Block::new().title("CSCTRL".red().bold().underlined()).borders(Borders::all()), layout_main[0]);

    let time_in_secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let cursor = if time_in_secs % 2 == 0 { "█" } else { "" };
    frame.render_widget(
        Paragraph::new(format!("> {}{}", state.input_box, cursor)),
        layout_main[1]
    );
}
