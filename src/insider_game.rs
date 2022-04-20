use rand::seq::SliceRandom;

/// get theme
pub fn get_theme() -> Option<String> {
    let mut theme_list = vec!["山", "学校", "算数", "ドラゴン", "テスト"];
    let mut rng = rand::thread_rng();
    theme_list.shuffle(&mut rng);
    let option_target = theme_list.get(0);
    if let Some(target) = option_target {
        return Some(target.to_string());
    };
    None
}

/// player and role struct
pub struct PlayerRole {
    player_name: String,
    role: String
}

/// hand out roles.
/// # Arguments
/// * `players` - player name list
pub fn hand_out_role(players: Vec<String>) -> Vec<PlayerRole> {
    let mut roles = vec!["マスター", "インサイダー", "市民", "市民"];
    if players.len() > 4 {
        let append_player_count = players.len() - 1;
        for _count in 1..append_player_count {
            roles.push("市民");
        }
    }
    let mut rng = rand::thread_rng();
    roles.shuffle(&mut rng);

    let mut player_role_list: Vec<PlayerRole> = vec![];
    for player in players {
        let option_role = roles.pop();
        if let Some(role) = option_role {
            player_role_list.push(PlayerRole {player_name: player, role: role.to_string()});
        }
    }
    player_role_list
}