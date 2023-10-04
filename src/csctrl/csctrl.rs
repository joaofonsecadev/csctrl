use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use crate::{csctrl, system};
use crate::terminal::terminal::Terminal;
use crate::webserver::webserver::Webserver;



pub struct Csctrl {
    requested_exit: bool,
    pub csctrl_config: csctrl::types::CsctrlConfig,
    webserver: Webserver,
    terminal: Terminal,
}

pub fn command_messenger() -> &'static RwLock<Vec<String>> {
    static COMMAND_MESSENGER: OnceLock<RwLock<Vec<String>>> = OnceLock::new();
    COMMAND_MESSENGER.get_or_init(|| RwLock::new(vec![]))
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

    pub fn has_requested_exit(&self) -> &bool {
        return &self.requested_exit;
    }

    pub fn init(&mut self) {
        tracing::info!("CSCTRL Version {}", env!("CARGO_PKG_VERSION"));

        let _ = &self.webserver.init(&self.csctrl_config);
        let _ = &self.terminal.init();
    }

    pub fn tick(&mut self) {
        if *self.terminal.is_terminal_active() { &self.terminal.tick(); }
        else { self.requested_exit = true; }

        self.process_command_messenger();
    }

    pub fn shutdown(&self) {
        tracing::info!("Exiting CSCTRL");
        let _ = &self.terminal.shutdown();
        let _ = &self.webserver.shutdown();
    }

    fn process_command_messenger(&self) {
        let is_command_messenger_empty = command_messenger().read().unwrap().is_empty();
        if is_command_messenger_empty { return; }

        let command = command_messenger().write().unwrap().pop().unwrap();
        self.handle_command(command);
    }

    fn handle_command(&self, command_string: String) {
        tracing::debug!("Processing command '{}'", command_string);


    }
}
