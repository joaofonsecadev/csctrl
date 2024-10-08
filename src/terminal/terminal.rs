use std::io::{stdout, Stdout};
use crossterm::ExecutableCommand;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::{Backend, CrosstermBackend};
use std::cell::OnceCell;
use std::cmp;
use std::str::Lines;
use clap::Parser;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::prelude::{Color, Direction};
use ratatui::style::{Style, Stylize};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::widgets::GraphType::Line;
use tracing_subscriber::fmt::format;
use crate::ClapParser;
use crate::csctrl::types::CsctrlDataParent;

struct TerminalUiState {
    input_box: String,
    last_type_time_secs: u64,
    selected_server_address: String
}

pub struct Terminal {
    terminal_ui: OnceCell<ratatui::Terminal<CrosstermBackend<Stdout>>>,
    is_terminal_active: bool,
    terminal_ui_state: TerminalUiState,
    cached_server_data: CsctrlDataParent,
}

impl Terminal {
    pub fn terminal() -> Terminal {
        Terminal {
            terminal_ui_state: TerminalUiState {
                input_box: "".to_string(),
                last_type_time_secs: 0,
                selected_server_address: "".to_string(),
            },
            terminal_ui: OnceCell::new(),
            is_terminal_active: false,
            cached_server_data: CsctrlDataParent { servers: Default::default() },
        }
    }
    
    pub fn init(&mut self) {
        if ClapParser::parse().disable_terminal { return; }
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
            ui(&mut self.terminal_ui_state, &mut self.cached_server_data, frame);
        }).unwrap();
    }

    pub fn update_cached_server_data(&mut self, new_server_data: CsctrlDataParent) {
        self.cached_server_data = new_server_data;
    }

    fn handle_events(&mut self) {
        if !crossterm::event::poll(Default::default()).unwrap() { return; }

        if let Event::Key(key) = crossterm::event::read().unwrap() {
            match key.code {
                KeyCode::Char(value) => {
                    if !validate_input_char(&value) { return; }
                    if key.kind != KeyEventKind::Press { return; }
                    &self.terminal_ui_state.input_box.push(value);
                    self.terminal_ui_state.last_type_time_secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
                }
                KeyCode::Backspace => {
                    if key.kind != KeyEventKind::Press { return; }
                    &self.terminal_ui_state.input_box.pop();
                    self.terminal_ui_state.last_type_time_secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
                }
                KeyCode::Enter => {
                    if key.kind != KeyEventKind::Press { return; }

                    let input = format!("<csctrlseptarget>{}<csctrlseptarget>{}", self.terminal_ui_state.selected_server_address, self.terminal_ui_state.input_box.to_string());
                    if input.is_empty() { return; }
                    self.terminal_ui_state.input_box.clear();
                    crate::csctrl::csctrl::get_command_messenger().write().unwrap().push_back(input);
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

    pub fn set_selected_server_address(&mut self, new_selected_server: &String) {
        self.terminal_ui_state.selected_server_address = new_selected_server.to_string();
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

fn ui(state: &mut TerminalUiState, data: &mut CsctrlDataParent, frame: &mut Frame<CrosstermBackend<Stdout>>) {
    let terminal_height = frame.size().height;

    let layout_main = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Length(cmp::max(terminal_height as i32 - 2, 1) as u16),
            Constraint::Length(1),
        ])
        .split(frame.size());

    let layout_servers_active = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(80)
        ])
        .split(layout_main[1]);

    let layout_active_logs = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(70),
            Constraint::Percentage(30)
        ])
        .split(layout_servers_active[1]);

    let layout_active_borders = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Length(cmp::max(layout_active_logs[0].width as i32 - 2, 1) as u16),
            Constraint::Length(1)
        ])
        .split(layout_active_logs[0]);

    let layout_active_borders_inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Length(cmp::max(layout_active_logs[0].height as i32 - 2, 1) as u16),
            Constraint::Length(1)
        ])
        .split(layout_active_borders[1]);

    let layout_active_server_data = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(40),
        ])
        .split(layout_active_borders_inner[1]);

    let mut server_list: Vec<ratatui::prelude::Line<'_>> = vec![
        ratatui::prelude::Line::from(vec!["Selected".bg(Color::Green).black().bold(), "  ".into(), "Online".green(), "  ".into(), "Offline".dark_gray()]),
        "".into(),
    ];
    for (server_address, server_data) in &data.servers {
        if state.selected_server_address == server_address.to_string() {
            server_list.push(Span::styled(format!("{} - {}", server_data.config.name, server_address), Style::default().bg(Color::Green).black().bold()).into());
        }
        else if server_data.is_online {
            server_list.push(Span::styled(format!("{} - {}", server_data.config.name, server_address), Style::default().green()).into());
        }
        else {
            server_list.push(Span::styled(format!("{} - {}", server_data.config.name, server_address), Style::default().dark_gray()).into());
        }
        server_list.push("".into());
    }
    let servers_block = Block::new().title("Servers").borders(Borders::all());

    frame.render_widget(Block::new().title("CSCTRL".red().bold().underlined()), layout_main[0]);
    frame.render_widget(Paragraph::new(server_list).block(servers_block).wrap(Wrap { trim: false }), layout_servers_active[0]);
    frame.render_widget(Block::new().title("Selected server data").borders(Borders::all()), layout_active_logs[0]);
    frame.render_widget(Block::new().title("Logs").borders(Borders::all()), layout_active_logs[1]);

    let selected_server_data_option = data.servers.get(&state.selected_server_address);
    if selected_server_data_option.is_some() {
        let selected_server_data = selected_server_data_option.unwrap();

        let team_ct_score = selected_server_data.team_ct.score.to_string();
        let mut team_ct_list: Vec<ratatui::prelude::Line<'_>> = vec![];

        let team_t_score = selected_server_data.team_t.score.to_string();
        let mut team_t_list: Vec<ratatui::prelude::Line<'_>> = vec![];

        for player_ct in &selected_server_data.team_ct.players {
            team_ct_list.push(Span::styled(&player_ct.name, Style::default()).into());
        }

        for player_t in &selected_server_data.team_t.players {
            team_t_list.push(Span::styled(&player_t.name, Style::default()).into());
        }

        let team_ct_name = if !selected_server_data.team_ct.name.is_empty() {
            format!("{} - ", selected_server_data.team_ct.name)
        } else {
            "".to_string()
        };

        let team_t_name = if !selected_server_data.team_t.name.is_empty() {
            format!("- {}", selected_server_data.team_t.name)
        } else {
            "".to_string()
        };

        let team_ct_border_title = format!(" {}Team CT - {} ", team_ct_name, selected_server_data.team_ct.score);
        let team_t_border_title = format!(" {} - Team T{} ", selected_server_data.team_ct.score, team_t_name);

        let team_ct_block = Block::new().title(team_ct_border_title).border_style(Style::default().light_blue()).title_style(Style::default().light_blue()).title_alignment(Alignment::Right).borders(Borders::all());
        let team_t_block = Block::new().title(team_t_border_title).border_style(Style::default().yellow()).title_style(Style::default().yellow()).title_alignment(Alignment::Left).borders(Borders::all());
        let events_block = Block::new().title("Events").borders(Borders::all());

        frame.render_widget(Paragraph::new(team_ct_list).block(team_ct_block).alignment(Alignment::Right), layout_active_server_data[0]);
        frame.render_widget(Paragraph::new(team_t_list).block(team_t_block).alignment(Alignment::Left), layout_active_server_data[1]);
        frame.render_widget(events_block, layout_active_server_data[2]);
    }

    let time_in_secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let cursor = if time_in_secs % 2 == 0 || time_in_secs - state.last_type_time_secs < 1 { "█" } else { "" };
    frame.render_widget(
        Paragraph::new(format!("> {}{}", state.input_box, cursor)),
        layout_main[2]
    );
}
