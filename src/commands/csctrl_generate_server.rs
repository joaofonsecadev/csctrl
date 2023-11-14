use crate::csctrl::csctrl::Csctrl;
use crate::csctrl::types::CsctrlServerSetup;

pub struct CsctrlGenerateServer;

impl crate::commands::base::Command for CsctrlGenerateServer {
    fn exec(&self, csctrl: &mut Csctrl, target_address: String, arguments: String) {
        let split_arguments: Vec<&str> = arguments.split(" ").collect();

        if split_arguments.len() != 3 {
            tracing::error!("{} expects 3 arguments but was provided {}", self.name(), split_arguments.len());
            return;
        }

        let server_name = split_arguments[0].to_string();
        let server_address = split_arguments[1].to_string();
        let server_rcon_password = split_arguments[2].to_string();

        csctrl.csctrl_config.servers.push(CsctrlServerSetup {
            name: server_name,
            address: server_address,
            rcon_password: server_rcon_password,
        });

        csctrl.write_config();
    }

    fn name(&self) -> String {
        "csctrl.generate.server".to_string()
    }

    fn description(&self) -> String {
        "Add a server entry to CSCTRL".to_string()
    }

    fn variables(&self) -> String {
        "1. Name to save the server with; 2. Server address; 3. Server rcon password".to_string()
    }

    fn example(&self) -> String {
        "csctrl.generate.server ServerA 0.0.0.0:27015 SuperRconPassword3".to_string()
    }
}