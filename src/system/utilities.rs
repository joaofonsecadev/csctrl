use std::path::PathBuf;
use clap::Parser;
use rand::Rng;
use crate::ClapParser;
use crate::csctrl::types::CsctrlConfig;

pub fn get_csctrl_config_file_path() -> PathBuf {
    let mut csctrl_config_file_path = std::env::current_exe().unwrap();
    csctrl_config_file_path.pop();
    csctrl_config_file_path.push("csctrl.json");
    return csctrl_config_file_path;
}

pub fn write_config(config: &CsctrlConfig) {
    let csctrl_config_file_path = get_csctrl_config_file_path();
    let file = std::fs::OpenOptions::new().write(true).truncate(true).create(true).open(&csctrl_config_file_path).unwrap();
    match serde_json::to_writer_pretty(file, config) {
        Err(_) => { tracing::error!("Can't write config to file '{}'", csctrl_config_file_path.to_str().unwrap()) }
        _ => {}
    }
}

pub fn load_config() -> CsctrlConfig {
    let command_line_args = ClapParser::parse();
    if command_line_args.reset { return generate_default_config(); }

    let config_string = std::fs::read_to_string(get_csctrl_config_file_path());
    if config_string.is_err() { return generate_default_config(); }

    match serde_json::from_str(&config_string.unwrap()) {
        Ok(config_json) => { return config_json; }
        Err(_) => {
            let error_message = "Can't load a valid config from 'csctrl.json'. Aborting execution. Tip: Rename or move the file so a new one is generated";
            tracing::error!(error_message);
            panic!("{}", error_message);
        }
    }
}

fn generate_default_config() -> CsctrlConfig {
    tracing::info!("Creating config file 'csctrl.json' at executable directory");

    let config = CsctrlConfig {
        chat_signature: "csctrl".to_string(),
        cs_listen_path: "/cslog".to_string(),
        rest_api_address: "0.0.0.0:27016".to_string(),
        secret: rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(16).map(char::from).collect(),
        servers: vec![],
        tracing_env_filter: "csctrl=info".to_string(),
    };

    write_config(&config);
    return config;
}

pub fn configure_tracing(env_filter: &str) -> tracing_appender::non_blocking::WorkerGuard {
    let mut csctrl_binary_path = std::env::current_exe().unwrap();
    csctrl_binary_path.pop();
    csctrl_binary_path.push("logs");
    csctrl_binary_path.push("csctrl");


    let timestamp = chrono::Local::now().format("%Y-%m-%d---%H-%M-%S");
    let file_appender = tracing_appender::rolling::never(csctrl_binary_path, format!("csctrl_{}.log", timestamp));
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing::subscriber::set_global_default(tracing_subscriber::fmt().with_writer(non_blocking)
        .with_target(false)
        .with_env_filter(env_filter)
        .with_ansi(false)
        .finish()).expect("Failed tracing subscriber creation");

    return _guard;
}
