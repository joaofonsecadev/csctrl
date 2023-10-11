use crate::csctrl::types::CsctrlServerSetup;

pub struct CsctrlServer {
    setup: CsctrlServerSetup,
}

impl CsctrlServer {
    pub fn csctrl_server(setup: CsctrlServerSetup) -> CsctrlServer {
        CsctrlServer {
            setup,
        }
    }

    pub fn init(&self) {

    }

    pub fn tick(&self) {

    }

    pub fn get_setup(&self) -> &CsctrlServerSetup { return &self.setup }
}