use crate::{csctrl, system};
use crate::terminal::terminal::Terminal;
use crate::webserver::webserver::Webserver;

pub struct Csctrl {
    requested_exit: bool,
    csctrl_config: csctrl::types::CsctrlConfig,
    webserver: Webserver,
    terminal: Terminal,
}

impl Csctrl {
    pub fn csctrl() -> Csctrl {
        Self {
            requested_exit: false,
            csctrl_config: system::utilities::load_config(),
            webserver: Webserver::webserver(),
            terminal: Terminal::terminal()
        }
    }

    pub fn init(&mut self) {
        system::utilities::configure_tracing(&self.csctrl_config.tracing_env_filter);
        tracing::info!("CSCTRL Version {}", env!("CARGO_PKG_VERSION"));

        let _ = &self.webserver.init(&self.csctrl_config);
        let _ = &self.terminal.init();
    }

    pub fn tick(&mut self) {
        if *self.terminal.is_terminal_active() { &self.terminal.tick(); }
        else { self.requested_exit = true; }
    }

    pub fn has_requested_exit(&self) -> &bool {
        return &self.requested_exit;
    }

    pub fn shutdown(&self) {
        tracing::info!("Exiting CSCTRL");
        let _ = &self.terminal.shutdown();
        let _ = &self.webserver.shutdown();
    }
}