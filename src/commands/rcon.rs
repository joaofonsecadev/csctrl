use crate::commands::base::{CommandExec, CommandInfo};
use crate::csctrl::csctrl::Csctrl;

pub struct Rcon {
    command_info: CommandInfo
}

impl CommandExec for Rcon {
    fn command(&self, csctrl: &mut Csctrl, command_string: &String) {
        tracing::info!("Command '{}' is being executed: {}", self.command_info.command, command_string)
    }
}