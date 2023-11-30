use regex::{Captures, Regex};
use crate::csctrl::csctrl::{Csctrl, get_data};
use crate::csctrl::types::{CsctrlDataPlayer, CsctrlDataServer};

fn find_player_index_and_team_with_steamid3<'a>(server_data: &'a mut CsctrlDataServer, steam_id_3: &str, out_player_team: &mut String) -> i32 {
    let mut index = 0;
    let ct_players = &mut server_data.team_ct.players;
    let t_players = &mut server_data.team_t.players;

    for player in ct_players.clone() {
        if player.steam3 != steam_id_3 {
            index = index + 1;
            continue;
        }
        *out_player_team = "CT".to_string();
        return index;
    }

    index = 0;
    for player in t_players.clone() {
        if player.steam3 != steam_id_3 {
            index = index + 1;
            continue;
        }
        *out_player_team = "TERRORIST".to_string();
        return index;
    }

    return -1;
}

pub fn player_say(csctrl: &mut Csctrl, server_data: &mut CsctrlDataServer, regex_captures: &Captures) {
    let chat = regex_captures["chat"].to_string();
    let steam_id = regex_captures["steam_id"].to_string();

    let ready_command = chat.contains(".ready");
    let unready_command = chat.contains(".unready");

    let mut player_team: String = "".to_string();
    let player_index = find_player_index_and_team_with_steamid3(server_data, &steam_id, &mut player_team);
    if player_index < 0 {
        tracing::error!("No record of a player on server with steamid3 '{}'", steam_id);
        return;
    }
    let mut found_player = if player_team.eq_ignore_ascii_case("TERRORIST") {
        server_data.team_t.players.get_mut(player_index as usize).unwrap()
    } else {
        server_data.team_ct.players.get_mut(player_index as usize).unwrap()
    };

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

pub fn player_switch_team(csctrl: &mut Csctrl, server_data: &mut CsctrlDataServer, regex_captures: &Captures) {
    let steam_id = regex_captures["steam_id"].to_string();
    let player_username = regex_captures["username"].to_string();
    let player_data = CsctrlDataPlayer {
        name: player_username,
        steam3: steam_id.clone(),
        is_ready: false,
    };

    let team_to = regex_captures["team_to"].to_string();
    let team_from = regex_captures["team_from"].to_string();
    if team_from.eq_ignore_ascii_case("Unassigned") {
        if team_to.eq_ignore_ascii_case("TERRORIST") {
            server_data.team_t.players.push(player_data);
        } else if team_to.eq_ignore_ascii_case("CT") {
            server_data.team_ct.players.push(player_data);
        }
        return;
    }

    let mut player_team: String = "".to_string();
    let player_index = find_player_index_and_team_with_steamid3(server_data, &steam_id, &mut player_team);
    if player_index > -1 {
        if player_team.eq_ignore_ascii_case("CT") {
            server_data.team_ct.players.remove(player_index as usize);
        } else if player_team.eq_ignore_ascii_case("TERRORIST") {
            server_data.team_t.players.remove(player_index as usize);
        }
    }

    if team_to.eq_ignore_ascii_case("TERRORIST") {
        server_data.team_t.players.push(player_data);
    } else if team_to.eq_ignore_ascii_case("CT") {
        server_data.team_ct.players.push(player_data);
    }

    csctrl.set_data_dirty();
}
