use std::io::{stdout, Stdout};
use crossterm::ExecutableCommand;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::{Backend, CrosstermBackend};
use crate::csctrl::types::CsServerConfig;
use std::cell::OnceCell;
use std::ops::Deref;
use std::time::Duration;
use crossterm::event::{Event, KeyCode};
use ratatui::Frame;

pub struct Terminal {
    selected_server: CsServerConfig,
    terminal_ui: OnceCell<ratatui::Terminal<CrosstermBackend<Stdout>>>,
    is_terminal_active: bool
}

impl Terminal {
    pub fn terminal() -> Terminal {
        Terminal {
            selected_server: CsServerConfig {
                name: "".to_string(),
                address: "".to_string(),
                rcon_address: "".to_string(),
                rcon_password: "".to_string(),
                csctrl_token: "".to_string(),
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

        self.terminal_ui.get_mut().unwrap().draw(ui).unwrap();
    }

    fn handle_events(&mut self) {
        if !crossterm::event::poll(Duration::from_millis(50)).unwrap() { return; }

        if let Event::Key(key) = crossterm::event::read().unwrap() {
            if KeyCode::Char('q') == key.code { self.close_terminal(); }
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

fn ui(frame: &mut Frame<CrosstermBackend<Stdout>>) {

}
