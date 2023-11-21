use crate::csctrl::csctrl::{Csctrl, get_data};
use crate::csctrl::types::{CsctrlDataPlayer, CsctrlDataServer};

pub fn player_say(csctrl: &mut Csctrl, server_data: &mut CsctrlDataServer, username: String, steam_id: String, team_side: String, chat: String) {
    let ready_command = chat.contains(".ready");
    let unready_command = chat.contains(".unready");

    let mut found_player: &mut CsctrlDataPlayer = &mut CsctrlDataPlayer {
        name: "".to_string(),
        steam3: "".to_string(),
        is_ready: false,
    };

    for player in &mut server_data.team_ct.players {
        if player.steam3 != steam_id { continue; }
        found_player = player;
        break;
    }

    if found_player.steam3.is_empty() {
        for player in &mut server_data.team_t.players {
            if player.steam3 != steam_id { continue; }
            found_player = player;
            break;
        }
    }

    if found_player.steam3.is_empty() {
        tracing::error!("No record of a player on server with steamid3 '{}'", steam_id);
        return;
    }

    if !ready_command && !unready_command {
        return;
    }

    let is_player_ready = ready_command && !unready_command;
    let current_player_ready_status = found_player.is_ready;
    if is_player_ready == current_player_ready_status { return; }

    if is_player_ready {
        found_player.is_ready = true;
        server_data.player_ready_amount = server_data.player_ready_amount + 1;
    } else {
        found_player.is_ready = false;
        server_data.player_ready_amount = server_data.player_ready_amount - 1;
    }

    csctrl.set_data_dirty();
}
