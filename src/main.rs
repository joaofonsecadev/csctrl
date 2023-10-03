mod csctrl;
mod webserver;
mod terminal;
mod system;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct ClapParser {
    /// Overwrites the default config file and creates a fresh one
    #[arg(long)]
    reset: bool,
}

fn main() {
    let mut csctrl = csctrl::csctrl::Csctrl::csctrl();

    csctrl.init();
    while !csctrl.has_requested_exit() {
        csctrl.tick();
    }

    csctrl.shutdown();
}
