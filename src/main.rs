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
}

fn configure_tracing() {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing::level_filters::LevelFilter::TRACE)
            .with_target(false)
            .finish())
        .expect("Failed tracing subscriber creation");
}
