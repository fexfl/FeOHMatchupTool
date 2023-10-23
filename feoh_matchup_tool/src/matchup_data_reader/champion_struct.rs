use serde::{Deserialize, Serialize};
use iced::widget::image;

use crate::feoh_matchup_tool::FeohError;

#[derive(Debug, Clone)]
pub struct Champion {
    //pub id: usize,
    pub name: String,
    pub iconname: String,
    pub counters: Vec<(String, MatchupSafety)>,
}

impl Champion {
    pub fn equals(&self, other: &String) -> bool {
        self.name == String::from(other)
    }
}

pub async fn get_images_with_ownership(champ: Champion) -> Result<(image::Handle,Vec<image::Handle>), FeohError> {
    let path = format!(
        "https://ddragon.leagueoflegends.com/cdn/13.20.1/img/champion/{}.png", champ.iconname
    );
    
    let bytes = reqwest::get(&path).await?.bytes().await?;

    let main_handle = image::Handle::from_memory(bytes);

    let mut counter_handles: Vec<image::Handle> = vec![];

    for (cntr,_ms) in champ.counters {
        let mut name = cntr.to_lowercase();
        
        if name.contains("\u{0027}") {
            name = name.replace("\u{0027}", "");
        }
        name = first_char_uppercase(&name);
        name = match &name[..] {
            "Aurelionsol" => "AurelionSol".to_string(),
            "Ksante" => "KSante".to_string(),
            "Wukong" => "MonkeyKing".to_string(),
            "Leesin" => "LeeSin".to_string(),
            _ => name,
        };
        let path_cntr = format!(
            "https://ddragon.leagueoflegends.com/cdn/13.20.1/img/champion/{}.png", name
        );
        
        let bytes_cntr = reqwest::get(&path_cntr).await?.bytes().await?;

        let handle_cntr = image::Handle::from_memory(bytes_cntr);

        counter_handles.push(handle_cntr);
        
    }
    return Ok((main_handle, counter_handles));
}

pub fn first_char_uppercase(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
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
    //pub id: usize,
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
            //id: rd.id,
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
            //id: champ.id,
            name: (&champ.name).to_string(),
            iconname: (&champ.iconname).to_string(),
            counters: pure_counter_vec,
            safeties: pure_ms_vec,
        };
        out_vec.push(rd);
    }
    return out_vec;
}

// The raw data format of the old Matchup Tool written in Java
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OldFormat {
    //pub id: usize,
    pub championId: String,
    pub championName: String,
    pub counters: Vec<String>,
    pub counterCompositions: Vec<String>,
    pub provenCounters: Vec<String>,
    pub playableCounters: Vec<String>,
    pub ireliaWins: usize,
    pub ireliaLosses: usize,
}