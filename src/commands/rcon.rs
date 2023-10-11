use crate::commands::base::{Command};
use crate::csctrl::csctrl::Csctrl;

pub struct Rcon;
impl Command for Rcon {
    fn exec(&self, csctrl: &mut Csctrl, arguments: String) {
        tracing::info!("Command '{}' is being executed: '{}'", self.name(), arguments)
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