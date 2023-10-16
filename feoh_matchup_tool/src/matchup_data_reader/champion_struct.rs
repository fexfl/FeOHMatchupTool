use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Champion {
    pub id: usize,
    pub name: String,
    pub iconname: String,
    pub counters: Vec<(String, MatchupSafety)>,
}

impl Champion {
    /*
    pub fn print_counters(&self) -> String{
        let mut out_string = String::from("Counters:");
        for cntr in &self.counters {
            let (st, ms) = cntr;
            out_string.push_str(" ");
            out_string.push_str(&st);
        }
        return out_string;
    }
    */
    pub fn equals(&self, other: &String) -> bool {
        self.name == String::from(other)
    }
    /*
    pub fn get_champion_with_name<'a>(input: &'a String, vect: &'a Vec<Champion>) -> &'a Champion {
        let index = vect.iter().position(|&r| r.equals(input)).unwrap();
        return &vect[index];
    }
    */
    
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

#[derive(Debug, Clone, Copy)]
pub enum MatchupSafety {
    Safe,
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

    fn _ms_to_int(&self) -> usize {
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
            id: rd.id,
            name: rd.name,
            iconname: rd.iconname,
            counters: countertuples,
        };
        champion_vector.push(champ);
    }
    return champion_vector;
    
}
