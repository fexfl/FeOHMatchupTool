use iced::{Sandbox, Settings};
use crate::feoh_matchup_tool::MatchupTool;

pub mod feoh_matchup_tool;
pub mod matchup_data_reader;

fn main() -> iced::Result {
    println!("uwu");
    let mut champion_array = matchup_data_reader::read_file();
    MatchupTool::run(Settings::default())
}