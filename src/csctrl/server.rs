use std::collections::{HashMap, VecDeque};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::error::TryRecvError;
use crate::csctrl::csctrl::{FORMAT_SEPARATOR, get_data, get_static_data};
use crate::csctrl::types::{CsctrlLogType, CsctrlServerSetup, MatchSetup};
use crate::rcon::connection::RconConnection;
use crate::system::utilities::get_csctrl_config_file_path;

pub struct CsctrlServer {
    address: String,
    rcon_connection: crate::rcon::connection::RconConnection,
    thread_receiver: tokio::sync::mpsc::UnboundedReceiver<String>,
    thread_sender: tokio::sync::mpsc::UnboundedSender<String>,
    last_rcon_success: bool,
}

impl CsctrlServer {
    pub fn csctrl_server(setup: CsctrlServerSetup, sender: tokio::sync::mpsc::UnboundedSender<String>, receiver: tokio::sync::mpsc::UnboundedReceiver<String>) -> CsctrlServer {
        CsctrlServer {
            address: setup.address.to_string(),
            rcon_connection: RconConnection::create_rcon_connection(&setup.address, &setup.rcon_password),
            thread_receiver: receiver,
            thread_sender: sender,
            last_rcon_success: false,
        }
    }

    pub fn main(&mut self) {
        tracing::debug!("Thread created");

        loop {
            if !self.tick() { break; };
        }

        tracing::debug!("Thread shutting down");
    }

    pub fn tick(&mut self) -> bool{
        match self.thread_receiver.try_recv() {
            Ok(message) => { self.handle_thread_message(message); }
            Err(error) => {
                if error == TryRecvError::Disconnected {
                    tracing::error!("Thread has no sender counterpart and has detached. Closing itself");
                    return false;
                }

                if error == TryRecvError::Empty {
                    return true;
                }

                tracing::error!("Can't receive message from main thread. Error: {}", error);
            }
        }
        return true;
    }

    fn handle_thread_message(&mut self, message: String) {
        tracing::trace!("Received message: '{}'", message);
        let mut split_string: VecDeque<&str> = message.split(" ").collect();
        let first_word = split_string[0];
        match first_word {
            "rcon" => {
                split_string.pop_front().unwrap();
                let mut arguments = "".to_string();
                for word in split_string {
                    arguments.push_str(format!("{} ", word).as_str());
                }

                Runtime::new().unwrap().block_on(self.rcon(arguments.trim().to_string()));
            }
            "server.match.start" => {
                let data = get_data().read().unwrap();
                let server_data = data.servers.get(&self.address).unwrap();

                let mut cmd_vec = vec![
                    self.generate_say_command("Loading match..."),
                    format!("mp_teamname_1 \"{}\"", server_data.match_setup.team_a_name),
                    format!("mp_teamname_2 \"{}\"", server_data.match_setup.team_b_name)
                ];

                let mut match_cfg_path = get_csctrl_config_file_path();
                match_cfg_path.pop();
                match_cfg_path.push(format!("cfg/{}.cfg", &server_data.match_setup.cfg_filename));

                let match_cfg_string = std::fs::read_to_string(match_cfg_path);
                if match_cfg_string.is_err() {
                    tracing::error!("Error reading match cfg file '{}'", &server_data.match_setup.cfg_filename);
                    return;
                }
                let fixed_line_endings_split_cfg = match_cfg_string.unwrap().replace("\r\n", "\n");
                let split_cfg: Vec<&str> = fixed_line_endings_split_cfg.split("\n").collect();
                for split_cmd in split_cfg {
                    cmd_vec.push(split_cmd.to_string());
                }

                cmd_vec.push("mp_warmup_pausetimer 1".to_string());
                cmd_vec.push("mp_warmup_start".to_string());
                cmd_vec.push(self.generate_say_command("WARMUP START"));
                cmd_vec.push(self.generate_say_command("Type '.ready' or '.unready' to change your readiness status"));

                for cmd in cmd_vec {
                    Runtime::new().unwrap().block_on(self.rcon(cmd));
                }

                if !self.last_rcon_success {
                    return;
                }

                self.send_message_to_main_thread("CsctrlMatchStatus:PreMatchWarmup");
            }
            &_ => {}
        }
    }

    pub async fn rcon(&mut self, command: String) -> bool {
        let mut response = match self.rcon_connection.execute_command(&command).await {
            Ok(res) => { res }
            Err(error) => {
                tracing::error!("Error while attempting rcon command. {}", error);
                self.last_rcon_success = false;
                return false;
            }
        };

        // Remove new line at the end
        response.pop();

        tracing::trace!("Rcon response:\n{}", response);

        self.last_rcon_success = true;
        return true;
    }

    fn generate_say_command(&self, say_text: &str) -> String {
        return format!("say [{}] {}", &get_static_data().read().unwrap().chat_signature, say_text);
    }

    fn send_message_to_main_thread(&self, message: &str) {
        self.thread_sender.send(format!("{}{}{}", self.address, FORMAT_SEPARATOR, message)).expect("Can't send message to main thread");
    }
}