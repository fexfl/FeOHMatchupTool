use serde::{Deserialize, Serialize};

pub struct Champion {
    pub name: String,
    pub iconname: String,
    pub counters: Vec<(String, MatchupSafety)>,
}

#[derive(Serialize, Deserialize)]
pub struct RawData {
    pub name: String,
    pub iconname: String,
    pub counters: Vec<String>,
    pub safeties: Vec<i32>,
}

pub enum MatchupSafety {
    Safe,
    Normal,
    Playable,
}

impl MatchupSafety {
    fn int_to_ms(int: i32) -> MatchupSafety {
        match int {
            0 => Self::Playable,
            1 => Self::Normal,
            2 => Self::Safe,
            _ => Self::Normal,
        }
    }

    fn ms_to_int(&self) -> i32 {
        match self {
            Self::Playable => 0,
            Self::Normal => 1,
            Self::Safe => 2,
        }
    }
}

pub fn transform_raw_to_champ(rdvec: Vec<RawData>) -> Vec<Champion> {
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
            name: rd.name,
            iconname: rd.iconname,
            counters: countertuples,
        };
        champion_vector.push(champ);
    }
    return champion_vector;
    
}