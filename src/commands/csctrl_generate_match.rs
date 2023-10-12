use crate::commands::base::Command;
use crate::csctrl::csctrl::Csctrl;

pub struct CsctrlGenerateMatch;
impl Command for CsctrlGenerateMatch {
    fn exec(&self, csctrl: &mut Csctrl, target_address: String, arguments: String) {
        todo!()
    }

    fn name(&self) -> String { "csctrl.generate.match".to_string() }

    fn description(&self) -> String {
        "Generate an empty match config file with a given name".to_string()
    }

    fn variables(&self) -> String {
        "1. Name for the match config file that is generated".to_string()
    }

    fn example(&self) -> String {
        "csctrl.generate.match TeamA-v-TeamB".to_string()
    }
}