use std::thread::sleep;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::error::SendError;
use tokio::task;
use crate::commands::base::{Command};
use crate::csctrl::csctrl::Csctrl;
use crate::csctrl::server::CsctrlServer;

pub struct Rcon;
impl Command for Rcon {
    fn exec(&self, csctrl: &mut Csctrl, target_address: String, arguments: String) {
        let found_server = match csctrl.servers.get(&target_address) {
            Some(server) => { server }
            None => {
                tracing::error!("No server with address '{}' to run rcon on", target_address);
                return;
            }
        };

        match found_server.sender.send(format!("rcon {}", arguments)) {
            Ok(_) => {}
            Err(error) => { tracing::error!("Can't send message to thread belonging to server '{}'. Error: {}", target_address, error); }
        }
    }

    fn name(&self) -> String {
        "rcon".to_string()
    }

    fn description(&self) -> String {
        "Executes commands in the currently selected server".to_string()
    }

    fn variables(&self) -> String {
        "1. Command(s) text to send to the server".to_string()
    }

    fn example(&self) -> String {
        "rcon sv_cheats 1".to_string()
    }
}