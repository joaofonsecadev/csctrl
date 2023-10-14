use std::collections::VecDeque;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::error::TryRecvError;
use crate::csctrl::types::CsctrlServerSetup;
use crate::rcon::connection::RconConnection;

pub struct CsctrlServer {
    config: CsctrlServerSetup,
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
            }
        }
        return true;
    }

    fn handle_thread_message(&self, message: String) {
        tracing::trace!("Received message: '{}'", message);
        let mut split_string: VecDeque<&str> = message.split(" ").collect();
        let first_word = split_string[0];

        match first_word {
            "rcon" => {
                split_string.pop_front().unwrap();
                let mut arguments = "".to_string();
                for word in split_string {
                    arguments.push_str(word);
                }

                Runtime::new().unwrap().block_on(self.rcon(arguments));
            }
            &_ => {}
        }
    }

    pub async fn rcon(&self, command: String) {
        if !self.rcon_connection.get_is_valid() {
            tracing::error!("No existing rcon connection to execute command 'rcon {}'", command);
            return;
        }
        /*let connection = self.rcon_connection.get_mut().unwrap();
        let response = match connection.cmd(&command).await {
            Ok(res) => { res }
            Err(error) => {
                tracing::error!("Error from rcon response: {}", error);
                return;
            }
        };

        tracing::trace!("Rcon response:\n{}", response);*/
    }

    pub fn get_setup(&self) -> &CsctrlServerSetup { return &self.config }
}