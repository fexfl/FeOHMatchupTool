use std::{fs, path::PathBuf};

pub mod champion_struct;

fn get_file_location() -> PathBuf {
    //PathBuf::from("C:\\Users\\fexfl\\Documents\\GitHub\\FEOHMatchupTool\\feoh_matchup_tool\\data\\matchups.json")
    PathBuf::from(".\\data\\matchups.json")
}

pub fn read_file() -> Vec<champion_struct::Champion> {
    println!("Reading file at: {}",get_file_location().display());
    let data = fs::read_to_string(get_file_location()).expect("Unable to read file at file location");
    let rawdata_array: Vec<champion_struct::RawData> = serde_json::from_str(&data).expect("JSON was not well-formatted");
    champion_struct::transform_raw_to_champ(rawdata_array)
}