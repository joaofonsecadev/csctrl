use std::collections::HashMap;
use std::ops::Add;
use std::sync::{OnceLock, RwLock};
use crate::{csctrl, system};
use crate::commands::base::Command;
use crate::commands::rcon::Rcon;
use crate::terminal::terminal::Terminal;
use crate::webserver::webserver::Webserver;

pub struct Csctrl {
    requested_exit: bool,
    pub csctrl_config: csctrl::types::CsctrlConfig,
    webserver: Webserver,
    terminal: Terminal,
}

pub fn get_command_messenger() -> &'static RwLock<Vec<String>> {
    static COMMAND_MESSENGER: OnceLock<RwLock<Vec<String>>> = OnceLock::new();
    COMMAND_MESSENGER.get_or_init(|| RwLock::new(vec![]))
}

pub fn get_registered_commands() -> &'static RwLock<HashMap<String, Box<dyn Command + Sync + Send>>> {
    static REGISTERED_COMMANDS: OnceLock<RwLock<HashMap<String, Box<dyn Command + Sync + Send>>>> = OnceLock::new();
    REGISTERED_COMMANDS.get_or_init(|| RwLock::new(HashMap::new()))

}

impl Csctrl {
    pub fn csctrl() -> Csctrl {
        Self {
            requested_exit: false,
            csctrl_config: system::utilities::load_config(),
            webserver: Webserver::webserver(),
            terminal: Terminal::terminal(),
        }
    }

    fn register_commands(&mut self) {
        let mut registered_commands =
            get_registered_commands().write().unwrap();

        let command_rcon = Box::new(Rcon);
        registered_commands.insert(command_rcon.name(), command_rcon);
    }

    pub fn has_requested_exit(&self) -> &bool {
        return &self.requested_exit;
    }

    pub fn init(&mut self) {
        tracing::info!("CSCTRL Version {}", env!("CARGO_PKG_VERSION"));
        let _ = self.register_commands();
        let _ = self.webserver.init(&self.csctrl_config);
        let _ = self.terminal.init();
    }

    pub fn tick(&mut self) {
        if *self.terminal.is_terminal_active() { self.terminal.tick(); }
        else { self.requested_exit = true; }

        self.process_command_messenger();
    }

    pub fn shutdown(&self) {
        tracing::info!("Exiting CSCTRL");
        let _ = &self.terminal.shutdown();
        let _ = &self.webserver.shutdown();
    }

    fn process_command_messenger(&mut self) {
        let is_command_messenger_empty = get_command_messenger().read().unwrap().is_empty();
        if is_command_messenger_empty { return; }

        let command = get_command_messenger().write().unwrap().pop().unwrap();
        self.handle_command(command);
    }

    fn handle_command(&mut self, command_string: String) {
        tracing::trace!("Processing command '{}'", command_string);
        let trimmed_string = command_string.trim();
        let split_string: Vec<&str> = trimmed_string.split(" ").collect();

        let registered_commands = get_registered_commands().read().unwrap();
        match registered_commands.get(split_string[0]) {
            None => {
                tracing::error!("No command '{}' exists", split_string[0]);
                return;
            }
            Some(found_command) => {
                let mut arguments = "".to_string();
                let mut iteration_number = -1;
                for arg in &split_string {
                    iteration_number += 1;
                    if iteration_number == 0 { continue; }
                    if arg.is_empty() { continue; }
                    arguments.push_str(arg);
                    arguments.push(' ');
                }

                tracing::trace!("Executing command '{}' with arguments '{}'", split_string[0], arguments.trim_end());
                found_command.exec(self, trimmed_string.to_string())
            }
        }
    }
}
