#[derive(serde::Serialize, serde::Deserialize)]
pub struct CsctrlConfig {
    pub chat_signature: String,
    pub cs_listen_path: String,
    pub rest_api_address: String,
    pub secret: String,
    pub servers: Vec<CsServerConfig>,
    pub tracing_env_filter: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CsServerConfig {
    pub name: String,
    pub address: String,
    pub rcon_address: String,
    pub rcon_password: String,
    pub csctrl_token: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MatchSetup {
    pub team_a: TeamConfig,
    pub team_b: TeamConfig,
    pub knife_round: bool,
    pub cfg_filename: String
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TeamConfig {
    pub name: String,
    pub members_steam_64: Vec<String>
}