use std::fs;

pub mod champion_struct;

fn get_file_location() -> String {
    String::from("../data/matchups.json")
}

pub fn read_file() -> Vec<champion_struct::Champion> {
    let data = fs::read_to_string(get_file_location()).expect("Unable to read file");
    let champion_array: Vec<champion_struct::Champion> = serde_json::from_str(data).expect("JSON was not well-formatted");
    for ch in champion_array {
        println!(ch.name);
    }
}