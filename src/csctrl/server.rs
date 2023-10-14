use std::future::Future;
use std::sync::OnceLock;
use rcon::Connection;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use crate::csctrl::types::CsctrlServerSetup;

pub struct CsctrlServer {
    config: CsctrlServerSetup,
    is_rcon_connected: bool,
    rcon_connection: OnceLock<Connection<TcpStream>>,
    thread_receiver: tokio::sync::mpsc::UnboundedReceiver<String>
}

impl CsctrlServer {
    pub fn csctrl_server(setup: CsctrlServerSetup, receiver: tokio::sync::mpsc::UnboundedReceiver<String>) -> CsctrlServer {
        CsctrlServer {
            config: setup,
            rcon_connection: OnceLock::new(),
            thread_receiver: receiver,
        }
    }

    pub fn main(&self) {
        tracing::debug!("Thread created");
        loop {
            self.tick();
        }
    }

    pub fn init(&mut self) {
        Runtime::new().unwrap().block_on(self.init_rcon_connection());
    }

    pub fn tick(&self) {

    }

    async fn init_rcon_connection(&mut self) {
        match <Connection<TcpStream>>::builder()
            .connect(&self.config.address, &self.config.rcon_password).await {
            Ok(connection) => {
                self.rcon_connection.get_or_init(|| connection);
                self.is_rcon_connected = true;
            }
            Err(error) => {
                tracing::error!("Can't establish a RCON connection to rcon address '{}' with password '{}'. Error: {}",
                    self.config.address, self.config.rcon_password, error);
                return;
            }
        };
    }

    pub async fn rcon(&mut self, command: String) {
        if !self.is_rcon_connected {
            tracing::error!("No existing rcon connection for server '{}'", self.config.address);
            return;
        }
        let connection = self.rcon_connection.get_mut().unwrap();
        let response = match connection.cmd(&command).await {
            Ok(res) => { res }
            Err(error) => {
                tracing::error!("Error from rcon response: {}", error);
                return;
            }
        };

        tracing::trace!("response: {}", response);
    }

    pub fn get_setup(&self) -> &CsctrlServerSetup { return &self.config }
}