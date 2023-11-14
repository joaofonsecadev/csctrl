use tokio::sync::mpsc::error::SendError;
use crate::csctrl::csctrl::Csctrl;
use crate::csctrl::types::MatchSetup;
use crate::system::utilities::get_csctrl_config_file_path;

pub struct ServerMatchSetupLoad;

impl crate::commands::base::Command for ServerMatchSetupLoad {
    fn exec(&self, csctrl: &mut Csctrl, target_address: String, arguments: String) {
        let found_server = csctrl.servers.get_mut(&target_address);
        if found_server.is_none() {
            tracing::error!("Can't find a server with address '{}'", &target_address);
            return;
        }

        let split_arguments: Vec<&str> = arguments.split(" ").collect();
        let mut match_setup_path = get_csctrl_config_file_path();
        match_setup_path.pop();
        match_setup_path.push(format!("matches/{}.json", &split_arguments[0]));
        if !match_setup_path.exists() {
            tracing::error!("Match setup file '{}' does not exist", &split_arguments[0]);
            return;
        }

        let setup_as_string = std::fs::read_to_string(match_setup_path);
        if setup_as_string.is_err() {
            tracing::error!("Error reading match setup file '{}'", &split_arguments[0]);
            return;
        }

        match found_server.unwrap().sender.send(setup_as_string.unwrap()) {
            Ok(_) => {}
            Err(error) => { tracing::error!("Can't send message to thread belonging to server '{}'. Error: {}", target_address, error); }
        }
    }

    fn name(&self) -> String {
        "server.match.setup.load".to_string()
    }

    fn description(&self) -> String {
        "Loads a match setup to a server instance".to_string()
    }

    fn variables(&self) -> String {
        "1. Name of the match setup file".to_string()
    }

    fn example(&self) -> String {
        "server.match.setup.load default".to_string()
    }
}