use crate::csctrl::csctrl::Csctrl;

pub trait CommandExec {
    fn command(&self, csctrl: &mut Csctrl, command_string: &String);
}

pub struct CommandInfo {
    pub command: String,
    pub description: String,
    pub variables: Vec<String>,
    pub example: String
}