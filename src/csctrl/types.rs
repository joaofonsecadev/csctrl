#[derive(serde::Serialize, serde::Deserialize)]
pub struct CsctrlConfig {
    pub chat_signature: String,
    pub cs_listen_path: String,
    pub rest_api_address: String,
    pub secret: String,
    pub servers: Vec<CsctrlServerSetup>,
    pub tracing_env_filter: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct CsctrlServerSetup {
    pub name: String,
    pub address: String,
    pub rcon_password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MatchSetup {
    pub team_a: TeamSettings,
    pub team_b: TeamSettings,
    pub knife_round: bool,
    pub cfg_filename: String
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TeamSettings {
    pub name: String,
    pub members_steam_64: Vec<String>
}

pub struct CsctrlDataParent {
    pub servers: Vec<CsctrlDataServer>
}

pub struct CsctrlDataServer {
    pub setup: CsctrlServerSetup,
    pub is_online: bool,
    pub team_a: CsctrlDataTeam,
    pub team_b: CsctrlDataTeam,
    pub status: CsctrlMatchStatus
}

pub struct CsctrlDataPlayer {
    pub name: String,
    pub steam64: String,
    pub is_ready: bool,
}

pub struct CsctrlDataTeam {
    pub name: String,
    pub score: u8,
    pub players: Vec<CsctrlDataPlayer>
}

pub enum CsctrlMatchStatus {
    NotStarted,
    KnifeRound,
    Warmup,
    Live,
    Finished,
}
