use serde::{Deserialize, Serialize};
use iced::widget::image;

#[derive(Debug, Clone)]
pub struct Champion {
    pub id: usize,
    pub name: String,
    pub iconname: String,
    pub counters: Vec<(String, MatchupSafety)>,
}

impl Champion {
    pub fn equals(&self, other: &String) -> bool {
        self.name == String::from(other)
    }

    pub fn get_champion_image(&self) -> image::Handle {
        let path = format!(
            ".\\img\\{}.png", self.iconname
        );
        image::Handle::from_path(path)
    }
}

impl std::fmt::Display for Champion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.name
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct RawData {
    pub id: usize,
    pub name: String,
    pub iconname: String,
    pub counters: Vec<String>,
    pub safeties: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MatchupSafety {
    Safe,
    #[default]
    Normal,
    Playable,
}

impl MatchupSafety {
    fn int_to_ms(int: usize) -> MatchupSafety {
        match int {
            0 => Self::Playable,
            1 => Self::Normal,
            2 => Self::Safe,
            _ => Self::Normal,
        }
    }

    fn ms_to_int(&self) -> usize {
        match self {
            Self::Playable => 0,
            Self::Normal => 1,
            Self::Safe => 2,
        }
    }

    pub const ALL: [MatchupSafety; 3] = [
        MatchupSafety::Playable,
        MatchupSafety::Normal,
        MatchupSafety::Safe,
    ];
}

impl std::fmt::Display for MatchupSafety {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            self
        )
    }
}

pub fn transform_raw_to_champ (rdvec: Vec<RawData>) -> Vec<Champion> {
    let mut champion_vector = vec![];
    for rd in rdvec {
        let mut countertuples: Vec<(String, MatchupSafety)> = vec![];
        for i in 0..rd.counters.len() {
            let ms = MatchupSafety::int_to_ms(rd.safeties[i]);
            let cntr = String::from(&rd.counters[i]);
            let tup: (String, MatchupSafety) = (cntr, ms);
            countertuples.push(tup);
        }
        let champ = Champion {
            id: rd.id,
            name: rd.name,
            iconname: rd.iconname,
            counters: countertuples,
        };
        champion_vector.push(champ);
    }
    return champion_vector;
    
}

pub fn export_champ_to_raw (champvec: &Vec<Champion>) -> Vec<RawData> {
    let mut out_vec: Vec<RawData> = vec![];
    for champ in champvec {
        let mut pure_counter_vec: Vec<String> = vec![];
        let mut pure_ms_vec: Vec<usize> = vec![];

        for (cntr, ms) in &champ.counters {
            pure_counter_vec.push(cntr.to_string());
            pure_ms_vec.push(ms.ms_to_int());
        }

        let rd = RawData {
            id: champ.id,
            name: (&champ.name).to_string(),
            iconname: (&champ.iconname).to_string(),
            counters: pure_counter_vec,
            safeties: pure_ms_vec,
        };
        out_vec.push(rd);
    }
    return out_vec;
}

pub fn get_champion_image_from_name(name: &str) -> image::Handle {
    let mut path: String = String::new();
    if name.contains("\u{0027}") {
        path = format!(
            ".\\img\\{}.png", name.to_lowercase().replace("\u{0027}", "")
        );
    } else {
        path = format!(
            ".\\img\\{}.png", name.to_lowercase()
        );
    }
    image::Handle::from_path(path)
}
