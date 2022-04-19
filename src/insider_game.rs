use rand::seq::SliceRandom;

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