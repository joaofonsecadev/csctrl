mod csctrl;
mod webserver;
mod terminal;
mod system;
mod commands;
mod rcon;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct ClapParser {
    /// Overwrites the default config file and creates a fresh one
    #[arg(long)]
    reset: bool,

    /// Disable the Terminal User Interface
    #[arg(long)]
    disable_terminal: bool,
}

fn main() {
    let mut csctrl = csctrl::csctrl::Csctrl::csctrl();
    let _tracing_guard = system::utilities::configure_tracing(&csctrl.csctrl_config.tracing_env_filter);

    csctrl.init();
    while !csctrl.has_requested_exit() {
        csctrl.tick();
    }

    csctrl.shutdown();
}
