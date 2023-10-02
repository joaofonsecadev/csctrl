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
    name: String,
    address: String,
    rcon_address: String,
    rcon_password: String,
    csctrl_token: String,
}