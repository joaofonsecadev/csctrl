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