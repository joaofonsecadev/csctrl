use std::collections::HashMap;

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
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct MatchSetup {
    pub team_a_name: String,
    pub team_b_name: String,
    pub knife_round: bool,
    pub cfg_filename: String,
    pub player_amount: u8,
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
    pub logs: Vec<String>
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
}
