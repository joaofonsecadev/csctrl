use tokio::sync::mpsc::error::SendError;
use crate::csctrl::csctrl::{Csctrl, get_data};
use crate::system::utilities::get_csctrl_config_file_path;

pub struct ServerMatchStart;

impl crate::commands::base::Command for ServerMatchStart {
    fn exec(&self, csctrl: &mut Csctrl, target_address: String, arguments: String) {
        let found_server = csctrl.servers.get_mut(&target_address);
        if found_server.is_none() {
            tracing::error!("Can't find a server with address '{}'", &target_address);
            return;
        }

        match found_server.unwrap().sender.send(self.name()) {
            Ok(_) => {}
            Err(error) => { tracing::error!("Can't send message to thread belonging to server '{}'. Error: {}", target_address, error); }
        }
    }

    fn name(&self) -> String {
        "server.match.start".to_string()
    }

    fn description(&self) -> String {
        "Starts a match taking into account a servers' previously loaded match setup".to_string()
    }

    fn variables(&self) -> String {
        "None".to_string()
    }

    fn example(&self) -> String {
        "server.match.start".to_string()
    }
}