use std::path::Path;
use crate::commands::base::Command;
use crate::csctrl::csctrl::Csctrl;

pub struct CsctrlGenerateMatch;
impl Command for CsctrlGenerateMatch {
    fn exec(&self, csctrl: &mut Csctrl, target_address: String, arguments: String) {
        let match_name_vec: Vec<&str> = arguments.split(" ").collect();
        let match_name = match_name_vec[0].to_owned() + ".json";
        tracing::trace!("Attempting to create match setup file with name '{}'", match_name);

        let mut csctrl_binary_path = std::env::current_exe().unwrap();
        csctrl_binary_path.pop();
        csctrl_binary_path.push("matches");
        csctrl_binary_path.push(&match_name);

        if Path::new(&csctrl_binary_path).exists() {
            tracing::error!("Match setup file '{}' already exists", &match_name);
            return;
        }

        let match_setup = crate::csctrl::types::MatchSetup {
            team_a: crate::csctrl::types::TeamSettings { name: "".to_string(), members_steam_3: vec![] },
            team_b: crate::csctrl::types::TeamSettings { name: "".to_string(), members_steam_3: vec![] },
            knife_round: false,
            cfg_filename: "".to_string(),
        };

        let file = std::fs::OpenOptions::new().write(true).truncate(true).create(true).open(&csctrl_binary_path).unwrap();
        match serde_json::to_writer_pretty(file, &match_setup) {
            Err(_) => { tracing::error!("Can't write match setup to file '{}'", csctrl_binary_path.to_str().unwrap()) }
            _ => {}
        }
    }

    fn name(&self) -> String { "csctrl.generate.match".to_string() }

    fn description(&self) -> String {
        "Generate a default match config file with a given name".to_string()
    }

    fn variables(&self) -> String {
        "1. Name for the match config file that is generated".to_string()
    }

    fn example(&self) -> String {
        "csctrl.generate.match TeamA-v-TeamB".to_string()
    }
}