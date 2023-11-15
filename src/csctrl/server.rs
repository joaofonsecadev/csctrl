use std::collections::VecDeque;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::error::TryRecvError;
use crate::csctrl::types::{CsctrlServerSetup, MatchSetup};
use crate::rcon::connection::RconConnection;
use crate::system::utilities::get_csctrl_config_file_path;

pub struct CsctrlServer {
    config: CsctrlServerSetup,
    match_setup: MatchSetup,
    rcon_connection: crate::rcon::connection::RconConnection,
    thread_receiver: tokio::sync::mpsc::UnboundedReceiver<String>,
    thread_sender: tokio::sync::mpsc::UnboundedSender<String>
}

impl CsctrlServer {
    pub fn csctrl_server(setup: CsctrlServerSetup, sender: tokio::sync::mpsc::UnboundedSender<String>, receiver: tokio::sync::mpsc::UnboundedReceiver<String>) -> CsctrlServer {
        CsctrlServer {
            rcon_connection: RconConnection::create_rcon_connection(&setup.address, &setup.rcon_password),
            config: setup,
            thread_receiver: receiver,
            thread_sender: sender,
            match_setup: MatchSetup {
                team_a_name: "".to_string(),
                team_b_name: "".to_string(),
                knife_round: false,
                cfg_filename: "".to_string(),
                player_amount: 0,
            },
        }
    }

    pub fn main(&mut self) {
        tracing::debug!("Thread created");
        Runtime::new().unwrap().block_on(self.try_rcon_connection());

        loop {
            if !self.tick() { break; };
        }

        tracing::debug!("Thread shutting down");
    }

    async fn try_rcon_connection(&mut self) {
        self.rcon_connection.init_rcon_connection().await;
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

        match serde_json::from_str(&message) {
            Ok(match_setup_json) => {
                self.set_match_setup(&match_setup_json);
                return;
            }
            _ => {}
        };

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
                let mut cmd_vec = vec![
                    format!("mp_teamname_1 \"{}\"", self.match_setup.team_a_name),
                    format!("mp_teamname_2 \"{}\"", self.match_setup.team_b_name)
                ];

                let mut match_cfg_path = get_csctrl_config_file_path();
                match_cfg_path.pop();
                match_cfg_path.push(format!("cfg/{}.cfg", &self.match_setup.cfg_filename));

                let match_cfg_string = std::fs::read_to_string(match_cfg_path);
                if match_cfg_string.is_err() {
                    tracing::error!("Error reading match cfg file '{}'", &self.match_setup.cfg_filename);
                    return;
                }

                let fixed_line_endings_split_cfg = match_cfg_string.unwrap().replace("\r\n", "\n");
                let split_cfg: Vec<&str> = fixed_line_endings_split_cfg.split("\n").collect();
                for split_cmd in split_cfg {
                    cmd_vec.push(split_cmd.to_string());
                }



                for cmd in cmd_vec {
                    Runtime::new().unwrap().block_on(self.rcon(cmd));
                }
            }
            &_ => {}
        }
    }

    pub async fn rcon(&mut self, command: String) {
        if !self.rcon_connection.get_is_valid() {
            tracing::error!("No existing rcon connection to execute command 'rcon {}'", command);
            return;
        }

        let mut response = match self.rcon_connection.execute_command(&command).await {
            Ok(res) => { res }
            Err(error) => {
                tracing::error!("Error while attempting rcon command. {}", error);
                return;
            }
        };

        // Remove new line at the end
        response.pop();

        tracing::trace!("Rcon response:\n{}", response);
    }

    fn generate_say_command(&self, say_text: &str) -> String {
        return format!("[{}]{}", self., say_text);
    }

    pub fn set_match_setup(&mut self, setup: &MatchSetup) {
        self.match_setup = setup.clone();
    }

    pub fn get_setup(&self) -> &CsctrlServerSetup { return &self.config }
}