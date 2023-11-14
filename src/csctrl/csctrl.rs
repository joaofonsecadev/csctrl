use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::string::ToString;
use std::sync::{OnceLock, RwLock};
use crate::{csctrl, system};
use crate::commands::base::Command;
use crate::commands::csctrl_generate_match::CsctrlGenerateMatch;
use crate::commands::csctrl_generate_server::CsctrlGenerateServer;
use crate::commands::rcon::Rcon;
use crate::commands::server_match_setup_load::ServerMatchSetupLoad;
use crate::commands::terminal_server_select::TerminalServerSelect;
use crate::csctrl::server::CsctrlServer;
use crate::csctrl::types::{CsctrlDataParent, CsctrlDataServer, CsctrlDataTeam, CsctrlMatchStatus, CsctrlServerContainer, CsctrlServerSetup};
use crate::terminal::terminal::Terminal;
use crate::webserver::webserver::Webserver;

pub const FORMAT_SEPARATOR: &str = "<csctrlseptarget>";

pub fn get_command_messenger() -> &'static RwLock<VecDeque<String>> {
    static COMMAND_MESSENGER: OnceLock<RwLock<VecDeque<String>>> = OnceLock::new();
    COMMAND_MESSENGER.get_or_init(|| RwLock::new(VecDeque::new()))
}

pub fn get_weblogs_messenger() -> &'static RwLock<VecDeque<String>> {
    static WEBLOGS_MESSENGER: OnceLock<RwLock<VecDeque<String>>> = OnceLock::new();
    WEBLOGS_MESSENGER.get_or_init(|| RwLock::new(VecDeque::new()))
}

pub fn get_registered_commands() -> &'static RwLock<HashMap<String, Box<dyn Command + Sync + Send>>> {
    static REGISTERED_COMMANDS: OnceLock<RwLock<HashMap<String, Box<dyn Command + Sync + Send>>>> = OnceLock::new();
    REGISTERED_COMMANDS.get_or_init(|| RwLock::new(HashMap::new()))
}

pub fn get_data() -> &'static RwLock<CsctrlDataParent> {
    static CSCTRL_READ_DATA: OnceLock<RwLock<CsctrlDataParent>> = OnceLock::new();
    CSCTRL_READ_DATA.get_or_init(|| RwLock::new(CsctrlDataParent { servers: HashMap::new() }))
}

pub struct Csctrl {
    requested_exit: bool,
    pub csctrl_config: csctrl::types::CsctrlConfig,
    webserver: Webserver,
    pub terminal: Terminal,
    pub servers: HashMap<String, CsctrlServerContainer>,
    server_threads_receiver: OnceLock<tokio::sync::mpsc::UnboundedReceiver<String>>,
    server_threads_sender: OnceLock<tokio::sync::mpsc::UnboundedSender<String>>,
    is_data_dirty: bool,
}

impl Csctrl {
    pub fn csctrl() -> Csctrl {
        Self {
            requested_exit: false,
            csctrl_config: system::utilities::load_config(),
            webserver: Webserver::webserver(),
            terminal: Terminal::terminal(),
            servers: HashMap::new(),
            server_threads_receiver: OnceLock::new(),
            server_threads_sender: OnceLock::new(),
            is_data_dirty: false,
        }
    }

    pub fn init(&mut self) {
        tracing::info!("CSCTRL Version {}", env!("CARGO_PKG_VERSION"));

        let _ = self.register_commands();
        let _ = self.webserver.init(&self.csctrl_config);
        let _ = self.terminal.init();

        let(sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        self.server_threads_receiver.get_or_init(|| receiver);
        self.server_threads_sender.get_or_init(|| sender);
        self.reset_registered_servers();
    }

    pub fn tick(&mut self) {
        if *self.terminal.is_terminal_active() { self.terminal.tick(); }
        else { self.requested_exit = true; }

        self.process_command_messenger();
        self.process_weblog_messenger();

        if self.is_data_dirty { self.handle_dirty_data(); }
    }

    pub fn shutdown(&self) {
        tracing::info!("Exiting CSCTRL");
        let _ = &self.terminal.shutdown();
        let _ = &self.webserver.shutdown();
    }

    fn reset_registered_servers(&mut self) {
        self.servers.clear();

        for server in &self.csctrl_config.servers {
            if self.servers.contains_key(server.address.as_str()) {
                tracing::error!("A server with address '{}' is already registered", server.address);
                continue;
            }

            let (local_sender, local_receiver) = tokio::sync::mpsc::unbounded_channel();
            let cloned_server_config = server.clone();
            let cloned_sender = self.server_threads_sender.get().unwrap().clone();
            let local_thread = std::thread::Builder::new().name(format!("[{}]", server.address)).spawn(move || {
                let mut server = CsctrlServer::csctrl_server(cloned_server_config, cloned_sender, local_receiver);
                server.main();
            }).unwrap();

            let server_container = CsctrlServerContainer {
                thread: local_thread,
                sender: local_sender,
            };
            self.servers.insert(server.address.to_string(), server_container);

            get_data().write().unwrap().servers.insert(server.address.to_string(), CsctrlDataServer {
                config: server.clone(),
                is_online: false,
                team_ct: CsctrlDataTeam {
                    name: "".to_string(),
                    score: 0,
                    players: vec![],
                },
                team_t: CsctrlDataTeam {
                    name: "".to_string(),
                    score: 0,
                    players: vec![],
                },
                status: CsctrlMatchStatus::NoStartHook,
                logs: vec![],
            });

            self.is_data_dirty = true;
        }
    }

    fn register_commands(&mut self) {
        let mut registered_commands =
            get_registered_commands().write().unwrap();

        let command_rcon = Box::new(Rcon);
        registered_commands.insert(command_rcon.name(), command_rcon);

        let command_csctrl_generate_server = Box::new(CsctrlGenerateServer);
        registered_commands.insert(command_csctrl_generate_server.name(), command_csctrl_generate_server);

        let command_csctrl_generate_match = Box::new(CsctrlGenerateMatch);
        registered_commands.insert(command_csctrl_generate_match.name(), command_csctrl_generate_match);

        let command_terminal_server_select = Box::new(TerminalServerSelect);
        registered_commands.insert(command_terminal_server_select.name(), command_terminal_server_select);

        let command_server_match_setup_load = Box::new(ServerMatchSetupLoad);
        registered_commands.insert(command_server_match_setup_load.name(), command_server_match_setup_load);
    }
    
    fn process_command_messenger(&mut self) {
        let is_command_messenger_empty = get_command_messenger().read().unwrap().is_empty();
        if is_command_messenger_empty { return; }

        let command = get_command_messenger().write().unwrap().pop_front().unwrap();
        self.handle_command(command);
    }

    fn process_weblog_messenger(&mut self) {
        let is_weblog_empty = get_weblogs_messenger().read().unwrap().is_empty();
        if is_weblog_empty { return; }

        let weblog = get_weblogs_messenger().write().unwrap().pop_front().unwrap();
        let split_weblog: Vec<&str> = weblog.split(FORMAT_SEPARATOR).collect();
        let address = split_weblog[0].replace("\"", "");
        let logs = split_weblog[1];
        let weblog_lines: Vec<&str> = logs.split("\n").collect();

        for line in weblog_lines {
            if line.contains("CsctrlTerminatingRconCommand") { return; }
            let mut csctrl_data_servers = &mut get_data().write().unwrap().servers;
            let mut data_server = csctrl_data_servers.get_mut(&address);
            if data_server.is_none() { return; }
            self.handle_weblog(data_server.unwrap(), line);
        }
    }

    fn handle_weblog(&mut self, server_data: &mut CsctrlDataServer, log_line: &str) {
        self.is_data_dirty = true;
        server_data.logs.push(log_line.to_string());
    }

    fn handle_dirty_data(&mut self) {
        self.is_data_dirty = false;
        if *self.terminal.is_terminal_active() {
            self.terminal.update_cached_server_data(get_data().read().unwrap().deref().clone());
        }
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
