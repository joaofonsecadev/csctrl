use crate::commands::csctrl_generate_server::CsctrlGenerateServer;
use crate::csctrl::csctrl::{Csctrl, get_data};

pub struct TerminalServerSelect;
impl crate::commands::base::Command for TerminalServerSelect {
    fn exec(&self, csctrl: &mut Csctrl, target_address: String, arguments: String) {
        let split_arguments: Vec<&str> = arguments.split(" ").collect();

        let data = get_data().read().unwrap();
        let found_server = data.servers.get(split_arguments[0]);
        if found_server.is_some() {
            csctrl.terminal.set_selected_server_address(&found_server.unwrap().config.address);
            return;
        }

        for (key, value) in &data.servers {
            if value.config.name == split_arguments[0] {
                csctrl.terminal.set_selected_server_address(&value.config.address);
                return;
            }
        }

        tracing::error!("No server found which goes by '{}'", split_arguments[0]);
    }

    fn name(&self) -> String {
        "terminal.server.select".to_string()
    }

    fn description(&self) -> String {
        todo!()
    }

    fn variables(&self) -> String {
        todo!()
    }

    fn example(&self) -> String {
        todo!()
    }
}