mod webserver;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct ClapParser {
    /// Overwrites the default config file and creates a fresh one
    #[arg(long)]
    reset: bool,
}

fn main() {
    configure_tracing();
    tracing::info!("CSCTRL Version {}", env!("CARGO_PKG_VERSION"));

    let thread_webserver = std::thread::spawn(|| { webserver::rest_api::start_rest_api(); });

    thread_webserver.join().unwrap();
    tracing::info!("Exiting CSCTRL");
}

fn configure_tracing() {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::builder()
            .with_target(false)
            .with_env_filter("csctrl=trace")
            .finish())
        .expect("Failed tracing subscriber creation");
}
