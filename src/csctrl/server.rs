use std::future::Future;
use std::sync::OnceLock;
use rcon::Connection;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use crate::csctrl::types::CsctrlServerSetup;

pub struct CsctrlServer {
    setup: CsctrlServerSetup,
    is_rcon_connected: bool,
    rcon_connection: OnceLock<Connection<TcpStream>>,
}

impl CsctrlServer {
    pub fn csctrl_server(setup: CsctrlServerSetup) -> CsctrlServer {
        CsctrlServer {
            setup,
            is_rcon_connected: false,
            rcon_connection: OnceLock::new(),
        }
    }

    pub fn init(&mut self) {
        Runtime::new().unwrap().block_on(self.init_rcon_connection());
    }

    pub fn tick(&self) {
        if !self.is_rcon_connected { return; }
    }

    async fn init_rcon_connection(&mut self) {
        match <Connection<TcpStream>>::builder()
            .connect(&self.setup.address, &self.setup.rcon_password).await {
            Ok(connection) => {
                self.rcon_connection.get_or_init(|| connection);
                self.is_rcon_connected = true;
            }
            Err(error) => {
                tracing::error!("Can't establish a RCON connection to rcon address '{}' with password '{}'. Error: {}",
                    self.setup.address, self.setup.rcon_password, error);
                return;
            }
        };
    }

    pub async fn rcon(&mut self, command: String) {
        if !self.is_rcon_connected {
            tracing::error!("No existing rcon connection for server '{}'", self.setup.address);
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

    pub fn get_setup(&self) -> &CsctrlServerSetup { return &self.setup }
}