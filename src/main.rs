use std::io::Write;
use rand::Rng;

mod webserver;
mod terminal;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct ClapParser {
    /// Overwrites the default config file and creates a fresh one
    #[arg(long)]
    reset: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Config {
    chat_signature: String,
    secret: String,
    servers: Vec<ServerConfig>,
    tracing_env_filter: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ServerConfig {
    name: String,
    ip: String,
    csctrl_token: String,
}

struct GlobalValues {
    config_file_path: std::path::PathBuf,
    executable_directory: std::path::PathBuf,
}

fn main() {
    let global_values = init();

    let config = load_config(&global_values);
    configure_tracing(&config.tracing_env_filter);

    tracing::info!("CSCTRL Version {}", env!("CARGO_PKG_VERSION"));
    let thread_webserver = std::thread::spawn(|| { webserver::rest_api::start_rest_api(); });

    thread_webserver.join().unwrap();
    tracing::info!("Exiting CSCTRL");
}

fn init() -> GlobalValues {
    let mut dynamic_path = std::env::current_exe().unwrap();
    dynamic_path.pop();

    let executable_directory = dynamic_path.to_path_buf();
    dynamic_path.push("csctrl.json");
    let config_file_path = dynamic_path.to_path_buf();

    GlobalValues {
        config_file_path,
        executable_directory,
    }
}

fn write_config(global_values: &GlobalValues, config: &Config) {
    let file = std::fs::OpenOptions::new().write(true).truncate(true).create(true).open(&global_values.config_file_path).unwrap();
    match serde_json::to_writer_pretty(file, config) {
        Err(_) => { tracing::error!("Can't write config to file '{}'", global_values.config_file_path.to_str().unwrap()) }
        _ => {}
    }
}

fn load_config(global_values: &GlobalValues) -> Config {
    let config_string = std::fs::read_to_string(&global_values.config_file_path);
    if config_string.is_err() { return generate_default_config(global_values); }

    match serde_json::from_str(&config_string.unwrap()) {
        Ok(config_json) => { return config_json; }
        Err(_) => {
            tracing::error!("Can't load a valid config from 'csctrl.json'. Aborting execution");
            panic!();
        }
    }
}

fn generate_default_config(global_values: &GlobalValues) -> Config {
    tracing::info!("Creating config file 'csctrl.json' at executable directory");

    let config = Config {
        chat_signature: "csctrl".to_string(),
        secret: rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(16).map(char::from).collect(),
        servers: vec![],
        tracing_env_filter: "csctrl=info".to_string(),
    };

    write_config(global_values, &config);
    return config;
}

fn configure_tracing(env_filter: &str) {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::builder()
            .with_target(false)
            .with_env_filter(env_filter)
            .finish())
        .expect("Failed tracing subscriber creation");
}
