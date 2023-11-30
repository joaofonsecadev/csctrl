use std::collections::HashMap;
use crate::system::utilities::get_csctrl_config_file_path;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CsctrlConfig {
    pub chat_signature: String,
    pub cs_listen_path: String,
    pub rest_api_address: String,
    pub secret: String,
    pub servers: Vec<CsctrlServerSetup>,
    pub tracing_env_filter: String,
}

#[derive(Clone)]
pub struct CsctrlStaticData {
    pub chat_signature: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct CsctrlServerSetup {
    pub name: String,
    pub address: String,
    pub rcon_password: String,
    pub match_setup: String
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct MatchSetup {
    pub team_a_name: String,
    pub team_b_name: String,
    pub knife_round: bool,
    pub cfg_filename: String,
    pub player_amount: i8,
}

impl MatchSetup {
    pub fn load_match_setup(file_name: &str) -> Result<MatchSetup, String> {
        let match_setup_string = match MatchSetup::load_match_setup_as_string(file_name) {
            Ok(valid_string) => { valid_string }
            Err(error) => { return Err(error); }
        };

        return match serde_json::from_str(&match_setup_string) {
            Ok(match_setup_json) => { Ok(match_setup_json) }
            _ => { Err(format!("Can't convert read string from '{}' to a valid MatchSetup", file_name)) }
        }
    }
    pub fn load_match_setup_as_string(file_name: &str) -> Result<String, String> {
        let mut match_setup_path = get_csctrl_config_file_path();
        match_setup_path.pop();
        match_setup_path.push(format!("matches/{}.json", file_name));
        if !match_setup_path.exists() {
            return Err(format!("Match setup file '{}' does not exist", file_name));
        }
        let setup_as_string = std::fs::read_to_string(match_setup_path);
        if setup_as_string.is_err() {
            return Err(format!("Error reading match setup file '{}'. {}", file_name, setup_as_string.err().unwrap()));
        }
        return Ok(setup_as_string.unwrap());
    }
}

pub struct CsctrlServerContainer {
    pub thread: std::thread::JoinHandle<()>,
    pub sender: tokio::sync::mpsc::UnboundedSender<String>
}

#[derive(Clone)]
pub struct CsctrlDataParent {
    pub servers: HashMap<String, CsctrlDataServer>
}

#[derive(Clone)]
pub struct CsctrlDataServer {
    pub config: CsctrlServerSetup,
    pub is_online: bool,
    pub team_ct: CsctrlDataTeam,
    pub team_t: CsctrlDataTeam,
    pub status: CsctrlMatchStatus,
    pub player_ready_amount: i8,
    pub logs: Vec<String>,
    pub match_setup: MatchSetup
}

#[derive(Clone)]
pub struct CsctrlDataPlayer {
    pub name: String,
    pub steam3: String,
    pub is_ready: bool,
}

#[derive(Clone)]
pub struct CsctrlDataTeam {
    pub name: String,
    pub score: u8,
    pub players: Vec<CsctrlDataPlayer>
}

#[derive(Clone, PartialEq)]
pub enum CsctrlMatchStatus {
    NoHook,
    PreMatchWarmup,
    KnifeRound,
    SwitchTeamsWarmup,
    Live,
    Finished,
    Paused,
    Invalid
}

impl CsctrlMatchStatus {
    pub fn string_to_enum(string: &str) -> CsctrlMatchStatus {
        return match string {
            "NoHook" => { CsctrlMatchStatus::NoHook },
            "PreMatchWarmup" => { CsctrlMatchStatus::PreMatchWarmup },
            "KnifeRound" => { CsctrlMatchStatus::KnifeRound },
            "SwitchTeamsWarmup" => { CsctrlMatchStatus::SwitchTeamsWarmup },
            "Live" => { CsctrlMatchStatus::Live },
            "Finished" => { CsctrlMatchStatus::Finished },
            "Paused" => { CsctrlMatchStatus::Paused },
            &_ => { CsctrlMatchStatus::Invalid }
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum CsctrlLogType {
    Invalid,
    PlayerSay,
    PlayerSwitchTeam,
}
