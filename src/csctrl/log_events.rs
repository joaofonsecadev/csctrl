use crate::csctrl::csctrl::{Csctrl, get_data};
use crate::csctrl::types::CsctrlDataServer;

pub fn player_say(csctrl: &mut Csctrl, server_data: &mut CsctrlDataServer, username: String, steam_id: String, team_side: String, chat: String) {
    tracing::info!("'{}':'{}':'{}':'{}'", username, steam_id, team_side, chat);
    if chat.contains(".ready") {
        for player in &mut server_data.team_ct.players {
            if player.steam3 != steam_id { continue; }
            player.is_ready = true;

        }
    }

    if chat.contains(".unready") {

    }
}