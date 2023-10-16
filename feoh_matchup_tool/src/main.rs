use iced::{Sandbox, Settings};
use crate::feoh_matchup_tool::MatchupTool;

pub mod feoh_matchup_tool;

fn main() -> iced::Result {
    println!("uwu");
    MatchupTool::run(Settings::default())
}