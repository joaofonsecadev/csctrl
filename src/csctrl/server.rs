use crate::csctrl::types::CsctrlServerSetup;

pub struct CsctrlServer {
    setup: CsctrlServerSetup,
    should_tick: bool
}

impl CsctrlServer {
    pub fn csctrl_server(setup: CsctrlServerSetup) -> CsctrlServer {
        CsctrlServer {
            setup,
            should_tick: false,
        }
    }

    pub fn init(&self) {

    }

    pub fn tick(&self) {

    }

    pub fn get_setup(&self) -> &CsctrlServerSetup { return &self.setup }
}