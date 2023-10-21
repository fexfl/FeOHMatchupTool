use std::{fs, path::PathBuf};

use crate::feoh_matchup_tool::ChampEnum;

use self::champion_struct::RawData;

pub mod champion_struct;

pub fn get_file_location() -> PathBuf {
    //PathBuf::from("C:\\Users\\fexfl\\Documents\\GitHub\\FEOHMatchupTool\\feoh_matchup_tool\\data\\matchups.json")
    PathBuf::from(".\\data\\matchups.json")
}

pub fn read_file() -> Vec<champion_struct::Champion> {
    println!("Reading file at: {}",get_file_location().display());
    let data = fs::read_to_string(get_file_location()).expect("Unable to read file at file location");
    let rawdata_array: Vec<champion_struct::RawData> = serde_json::from_str(&data).expect("JSON was not well-formatted");
    champion_struct::transform_raw_to_champ(rawdata_array)
}

pub fn write_file(rdvec: Vec<RawData>) {
    println!("Writing file at: {}", get_file_location().display());
    let out_string = serde_json::to_string_pretty(&rdvec).expect("Error in writing vector to string for file.");
    fs::write(get_file_location(), out_string).expect("File could not be written!");
}

pub fn create_json_file() {
    let mut empty_vec: Vec<RawData> = vec![];
    for champ_name in ChampEnum::all() {
        let new_rd = RawData {
            name: champ_name.to_string(),
            iconname: champ_name.iconname_from_enum(),
            counters: vec![],
            safeties: vec![],
        };
        empty_vec.push(new_rd);
    }
    let out_string = serde_json::to_string_pretty(&empty_vec).expect("Error creating json string of empty json file");
    fs::write(get_file_location(), out_string).expect("Error writing file!");
}