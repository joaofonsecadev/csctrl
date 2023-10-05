use crate::csctrl::csctrl::Csctrl;

pub trait Command {
    fn exec(&self, csctrl: &mut Csctrl, command_string: String);
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn variables(&self) -> String;
    fn example(&self) -> String;
}
