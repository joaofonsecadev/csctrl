use std::collections::HashMap;
use std::ops::Add;
use std::sync::{OnceLock, RwLock};
use crate::{csctrl, system};
use crate::commands::base::Command;
use crate::commands::csctrl_generate_server::CsctrlGenerateServer;
use crate::commands::rcon::Rcon;
use crate::csctrl::server::CsctrlServer;
use crate::csctrl::types::CsctrlDataParent;
use crate::terminal::terminal::Terminal;
use crate::webserver::webserver::Webserver;

pub fn get_command_messenger() -> &'static RwLock<Vec<String>> {
    static COMMAND_MESSENGER: OnceLock<RwLock<Vec<String>>> = OnceLock::new();
    COMMAND_MESSENGER.get_or_init(|| RwLock::new(vec![]))
}

pub fn get_registered_commands() -> &'static RwLock<HashMap<String, Box<dyn Command + Sync + Send>>> {
    static REGISTERED_COMMANDS: OnceLock<RwLock<HashMap<String, Box<dyn Command + Sync + Send>>>> = OnceLock::new();
    REGISTERED_COMMANDS.get_or_init(|| RwLock::new(HashMap::new()))
}

pub fn get_data() -> &'static RwLock<CsctrlDataParent> {
    static CSCTRL_DATA: OnceLock<RwLock<CsctrlDataParent>> = OnceLock::new();
    CSCTRL_DATA.get_or_init(|| RwLock::new(CsctrlDataParent { servers: vec![] }))
}

pub struct Csctrl {
    requested_exit: bool,
    pub csctrl_config: csctrl::types::CsctrlConfig,
    webserver: Webserver,
    terminal: Terminal,
    pub servers: HashMap<String, CsctrlServer>,
    server_threads_receiver: tokio::sync::mpsc::Receiver<String>,
}

impl Csctrl {
    pub fn csctrl() -> Csctrl {
        Self {
            requested_exit: false,
            csctrl_config: system::utilities::load_config(),
            webserver: Webserver::webserver(),
            terminal: Terminal::terminal(),
            servers: HashMap::new(),
        }
    }

    pub fn init(&mut self) {
        tracing::info!("CSCTRL Version {}", env!("CARGO_PKG_VERSION"));
        let _ = self.register_commands();
        let _ = self.webserver.init(&self.csctrl_config);
        let _ = self.terminal.init();

        self.reset_registered_servers();
    }

    pub fn tick(&mut self) {
        if *self.terminal.is_terminal_active() { self.terminal.tick(); }
        else { self.requested_exit = true; }

        self.process_command_messenger();
        for (_sv_address, server) in &self.servers {
            server.tick();
        }
    }

    pub fn shutdown(&self) {
        tracing::info!("Exiting CSCTRL");
        let _ = &self.terminal.shutdown();
        let _ = &self.webserver.shutdown();
    }

    fn reset_registered_servers(&mut self) {
        self.servers.clear();
        for server in &self.csctrl_config.servers {
            let (transmitter, receiver) = tokio::sync::mpsc::unbounded_channel();
            
            if self.servers.contains_key(server.address.as_str()) {
                tracing::error!("A server with address '{}' is already registered", server.address);
                continue;
            }

            let mut registered_server = CsctrlServer::csctrl_server(server.clone());
            registered_server.init();
            self.servers.insert(server.address.to_string(), registered_server);
        }
    }

    fn register_commands(&mut self) {
        let mut registered_commands =
            get_registered_commands().write().unwrap();

        let command_rcon = Box::new(Rcon);
        registered_commands.insert(command_rcon.name(), command_rcon);

        let command_csctrl_generate_server = Box::new(CsctrlGenerateServer);
        registered_commands.insert(command_csctrl_generate_server.name(), command_csctrl_generate_server);
    }
    
    fn process_command_messenger(&mut self) {
        let is_command_messenger_empty = get_command_messenger().read().unwrap().is_empty();
        if is_command_messenger_empty { return; }

        let command = get_command_messenger().write().unwrap().pop().unwrap();
        self.handle_command(command);
    }

    fn handle_command(&mut self, command_string: String) {
        let split_target_address: Vec<&str> = command_string.split("<csctrlseptarget>").collect();
        let target_address = split_target_address[1];
        let trimmed_string = split_target_address[2].trim();
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

                tracing::trace!("Executing command '{}' on target '{}' with arguments '{}'", split_string[0], target_address, arguments.trim_end());
                found_command.exec(self, target_address.to_string(), arguments.trim_end().to_string())
            }
        }
    }

    pub fn write_config(&self) {
        system::utilities::write_config(&self.csctrl_config);
    }

    pub fn has_requested_exit(&self) -> &bool {
        return &self.requested_exit;
    }
}
