use iced::{Sandbox, Settings};
use crate::feoh_matchup_tool::MatchupTool;

pub mod feoh_matchup_tool;
pub mod matchup_data_reader;

fn main() -> iced::Result {
    println!("Searching matchup json file...");
    if matchup_data_reader::get_file_location().exists() {
        println!("File found!");
    } else {
        matchup_data_reader::create_json_file();
        println!("Empty matchup file created!");
    }
    println!("Starting Matchup Tool.");
    MatchupTool::run(Settings::default())
}