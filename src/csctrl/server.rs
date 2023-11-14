use std::collections::VecDeque;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::error::TryRecvError;
use crate::csctrl::types::{CsctrlServerSetup, MatchSetup};
use crate::rcon::connection::RconConnection;

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

    pub fn set_match_setup(&mut self, setup: &MatchSetup) {
        self.match_setup = setup.clone();
    }

    pub fn get_setup(&self) -> &CsctrlServerSetup { return &self.config }
}