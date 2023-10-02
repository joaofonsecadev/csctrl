use crate::{csctrl, system};
use crate::webserver::webserver::Webserver;

pub struct Csctrl {
    requested_exit: bool,
    csctrl_config: csctrl::types::CsctrlConfig,
    webserver: Webserver
}

impl Csctrl {
    pub fn csctrl() -> Csctrl {
        Self {
            requested_exit: false,
            csctrl_config: system::utilities::load_config(),
            webserver: Webserver::webserver(),
        }
    }

    pub fn init(&self) {
        system::utilities::configure_tracing(&self.csctrl_config.tracing_env_filter);
        tracing::info!("CSCTRL Version {}", env!("CARGO_PKG_VERSION"));

        let _ = &self.webserver.init(&self.csctrl_config);

        //let terminal = terminal::terminal::Terminal::default(&global_values, &config);
        //terminal.start_terminal();
    }

    pub fn tick(&self) {

    }

    pub fn has_requested_exit(&self) -> &bool {
        return &self.requested_exit;
    }

    pub fn shutdown(&self) {
        tracing::info!("Exiting CSCTRL");
    }
}