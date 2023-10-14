use std::collections::VecDeque;
use std::future::Future;
use std::sync::OnceLock;
use rcon::Connection;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::error::TryRecvError;
use crate::csctrl::types::CsctrlServerSetup;

pub struct CsctrlServer {
    config: CsctrlServerSetup,
    rcon_connection: OnceLock<Connection<TcpStream>>,
    thread_receiver: tokio::sync::mpsc::UnboundedReceiver<String>,
    thread_sender: tokio::sync::mpsc::UnboundedSender<String>
}

impl CsctrlServer {
    pub fn csctrl_server(setup: CsctrlServerSetup, sender: tokio::sync::mpsc::UnboundedSender<String>, receiver: tokio::sync::mpsc::UnboundedReceiver<String>) -> CsctrlServer {
        CsctrlServer {
            config: setup,
            rcon_connection: OnceLock::new(),
            thread_receiver: receiver,
            thread_sender: sender,
        }
    }

    pub fn main(&mut self) {
        tracing::debug!("Thread created");
        loop {
            if !self.tick() { break; };
        }
        tracing::debug!("Thread shutting down");
    }

    pub fn init(&mut self) {
        Runtime::new().unwrap().block_on(self.init_rcon_connection());
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

    async fn init_rcon_connection(&self) {
        match <Connection<TcpStream>>::builder()
            .connect(&self.config.address, &self.config.rcon_password).await {
            Ok(connection) => {
                self.rcon_connection.get_or_init(|| connection);
            }
            Err(error) => {
                tracing::error!("Can't establish a RCON connection to rcon address '{}' with password '{}'. Error: {}",
                    self.config.address, self.config.rcon_password, error);
                return;
            }
        };
    }

    pub async fn rcon(&self, command: String) {
        if !false {
            tracing::error!("No existing rcon connection for server '{}'", self.config.address);
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