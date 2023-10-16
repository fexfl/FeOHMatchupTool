pub struct Champion {
    pub name: String,
    pub iconname: String,
    pub counters: Vec<(String, MatchupSafety)>,
}

enum MatchupSafety {
    Safe,
    Normal,
    Playable,
}